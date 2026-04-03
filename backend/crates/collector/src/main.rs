use std::time::Duration;

use shared::{
    structs::endpoints::{Endpoint, EndpointResult},
    traits::Collector,
};
use tokio::time::{sleep, timeout};

use crate::local_collector::LocalCollector;

mod local_collector;

// TODO make this user-configurable
const METRICS_DELAY: u64 = 5;
const ENDPOINTS_DELAY: u64 = 10;
const ENDPOINT_TIMEOUT: u64 = 5;

#[tokio::main]
pub async fn main() -> Result<(), shared::Error> {
    let mut collector = LocalCollector::new().unwrap();
    collector.try_get_id().await.unwrap();

    let base_url = shared::env::base_url()?;
    let endpoints_url = format!("{}/collector/{}/endpoints", base_url, collector.id.unwrap());
    let endpoints_results_url = format!(
        "{}/collector/{}/endpoints_results",
        base_url,
        collector.id.unwrap()
    );

    let client = reqwest::Client::new();

    // separate task for metrics
    tokio::spawn(async move {
        loop {
            handle_metrics(&mut collector).await;
            sleep(Duration::from_secs(METRICS_DELAY)).await;
        }
    });

    loop {
        handle_endpoints(&endpoints_url, &endpoints_results_url, &client).await;
        sleep(Duration::from_secs(ENDPOINTS_DELAY)).await;
    }
}

async fn handle_metrics(collector: &mut impl Collector) {
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
}

async fn handle_endpoints(endpoints_url: &str, results_url: &str, client: &reqwest::Client) {
    let endpoints = match client.get(endpoints_url).send().await {
        Ok(val) => match val.json::<Vec<Endpoint>>().await {
            Ok(val) => val,
            Err(_) => return,
        },
        Err(_) => return,
    };

    let mut endpoint_results = vec![];
    let timestamp = chrono::Local::now().naive_local();

    // test if services on endpoints are available, generate results
    for e in endpoints {
        // if timeout passes
        let response = timeout(Duration::from_secs(ENDPOINT_TIMEOUT), e.send(client))
            .await
            .unwrap_or(Err(shared::Error::Elapsed));

        let result = match response {
            Ok(mut val) => {
                // set the same timestamp to all
                val.timestamp = timestamp;
                val
            }
            Err(val) => {
                // TODO maybe add this to db also
                eprintln!("Error while checking endpoint: {}", val);
                EndpointResult {
                    endpoint_id: e.id,
                    timestamp,
                    result: false,
                    latency_microseconds: None,
                }
            }
        };

        endpoint_results.push(result);
    }

    // send results to backend
    let resp = client
        .post(results_url)
        .json(&endpoint_results)
        .send()
        .await;

    match resp {
        Ok(val) => match val.status().as_u16() {
            400 | 404 | 500 => {
                eprintln!(
                    "Unexpected error happened while sending endpoint results: {}",
                    val.status()
                )
            }
            _ => (),
        },
        Err(val) => {
            eprintln!("Error sending endpoint results: {}", val);
        }
    }
}
