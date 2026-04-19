use std::{collections::HashSet, time::Duration};

use shared::{
    structs::{
        db::PortsTable,
        endpoints::{Endpoint, EndpointResult},
        ports::Port,
    },
    traits::Collector,
};
use tokio::time::{sleep, timeout};

use crate::local_collector::LocalCollector;

mod local_collector;

// TODO make this user-configurable
const METRICS_DELAY: u64 = 10;
const ENDPOINTS_DELAY: u64 = 10;
const ENDPOINT_TIMEOUT: u64 = 5;
const PORTS_DELAY: u64 = 10;

#[tokio::main]
pub async fn main() -> Result<(), shared::Error> {
    let mut collector = LocalCollector::new().unwrap();
    collector.try_get_id().await.unwrap();
    let collector_id = collector.id.unwrap();

    let base_url = shared::env::base_url()?;
    let endpoints_url = format!("{}/collector/{}/endpoints", base_url, collector_id);
    let endpoints_results_url = format!(
        "{}/collector/{}/endpoints_results",
        base_url,
        collector.id.unwrap()
    );
    let ports_url = format!("{}/collector/{}/ports", base_url, collector_id);

    // separate task for metrics
    tokio::spawn(async move {
        handle_metrics(&mut collector).await;
    });

    // separate task for endpoints
    tokio::spawn(async move {
        handle_endpoints(
            &endpoints_url,
            &endpoints_results_url,
            &reqwest::Client::new(),
        )
        .await;
    });

    // handle ports
    handle_ports(&ports_url, &reqwest::Client::new()).await;

    Ok(())
}

async fn handle_metrics(collector: &mut impl Collector) {
    loop {
        let metrics = collector.get_metrics();

        let mut tries = 3;
        while tries > 0 {
            let resp = collector.send_metrics(&metrics).await;
            if let Err(val) = resp {
                tries -= 1;
                eprint!("Error sending metrics: ");

                match val {
                    shared::Error::CollectorRequiresID => {
                        eprint!("New ID required, trying... ");
                        match collector.try_get_id().await {
                            Ok(_) => eprintln!("Success"),
                            Err(val) => eprintln!("Failed: {}", val),
                        }
                    }
                    _ => eprintln!("{}", val),
                }

                tokio::time::sleep(Duration::from_secs(1)).await;
                continue;
            }
            break;
        }

        sleep(Duration::from_secs(METRICS_DELAY)).await;
    }
}

async fn handle_endpoints(endpoints_url: &str, results_url: &str, client: &reqwest::Client) {
    loop {
        let endpoints = match client.get(endpoints_url).send().await {
            Ok(val) => match val.json::<Vec<Endpoint>>().await {
                Ok(val) => val,
                Err(_) => return,
            },
            Err(_) => return,
        };

        let mut endpoint_results = vec![];
        let time = chrono::Utc::now();

        // test if services on endpoints are available, generate results
        for e in endpoints {
            // if timeout passes
            let response = timeout(Duration::from_secs(ENDPOINT_TIMEOUT), e.send(client))
                .await
                .unwrap_or(Err(shared::Error::Elapsed));

            let result = match response {
                Ok(mut val) => {
                    // set the same time to all
                    val.time = time;
                    val
                }
                Err(val) => {
                    // TODO maybe add this to db also
                    eprintln!("Error while checking endpoint: {}", val);
                    EndpointResult {
                        endpoint_id: e.id,
                        time,
                        result: false,
                        latency_microseconds: None,
                    }
                }
            };

            endpoint_results.push(result);
        }

        // send results to api server
        let resp = client
            .post(results_url)
            .json(&endpoint_results)
            .send()
            .await;

        match resp {
            Ok(val) => match val.status().as_u16() {
                400 | 404 | 500 => {
                    let code = &val.status().to_string();
                    let text = &val.text().await.unwrap_or(code.clone());

                    eprintln!(
                        "Unexpected error happened while sending endpoint results: {}",
                        text
                    )
                }
                _ => (),
            },
            Err(val) => {
                eprintln!("Error sending endpoint results: {}", val);
            }
        }
        sleep(Duration::from_secs(ENDPOINTS_DELAY)).await;
    }
}

async fn handle_ports(ports_url: &str, client: &reqwest::Client) {
    let mut old = match client.get(ports_url).send().await {
        Ok(val) => match val.json::<Vec<PortsTable>>().await {
            Ok(val) => val.iter().map(|p| p.into()).collect(),
            Err(val) => {
                eprintln!("Failed to deserialize Ports from API: {}", val);
                HashSet::new()
            }
        },
        Err(val) => {
            eprintln!("Failed to get recent ports from API: {}", val);
            HashSet::new()
        }
    };

    loop {
        let new = match LocalCollector::get_ports() {
            Ok(val) => val,
            Err(val) => {
                eprintln!("{}", val);
                sleep(Duration::from_secs(PORTS_DELAY)).await;
                continue;
            }
        };

        let opened = new.difference(&old).collect::<Vec<&Port>>();
        let closed = old.difference(&new).collect::<Vec<&Port>>();

        if !opened.is_empty() {
            let resp = client
                .put(format!("{ports_url}/opened"))
                .json(&opened)
                .send()
                .await;
            if let Err(val) = resp {
                eprintln!("Error sending newly opened ports to API: {}", val)
            }
        }

        if !closed.is_empty() {
            let resp = client
                .put(format!("{ports_url}/closed"))
                .json(&closed)
                .send()
                .await;
            if let Err(val) = resp {
                eprintln!("Error sending newly closed ports to API: {}", val)
            }
        }

        old = new.clone();
        sleep(Duration::from_secs(PORTS_DELAY)).await;
    }
}
