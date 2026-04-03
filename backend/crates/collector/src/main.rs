use std::time::Duration;

use chrono::NaiveDateTime;
use shared::structs::{
    Collector, UnidentifiedCollector, db,
    endpoints::{Endpoint, EndpointResult},
};
use tokio::time::{sleep, timeout};

// TODO make this user-configurable
const METRICS_DELAY: u64 = 5;
const ENDPOINTS_DELAY: u64 = 10;
const ENDPOINT_TIMEOUT: u64 = 5;

#[tokio::main]
pub async fn main() -> Result<(), shared::Error> {
    if !sysinfo::IS_SUPPORTED_SYSTEM {
        eprintln!("System is not supported!");
        eprintln!("These systems are supported: ");
        get_supported_systems()
            .iter()
            .for_each(|s| eprintln!("  {}", s));
        return Err(shared::Error::UnsupportedSystem);
    }

    let uc = UnidentifiedCollector::new();
    let mut collector = uc.identify().await?;

    let base_url = shared::env::base_url()?;
    let metrics_url = format!("{}/metrics", base_url);
    let endpoints_url = format!("{}/collector/{}/endpoints", base_url, collector.id);
    let endpoints_results_url =
        format!("{}/collector/{}/endpoints_results", base_url, collector.id);

    // TODO use only this client
    let client = reqwest::Client::new();

    // separate task
    tokio::spawn(async move {
        loop {
            handle_metrics(&mut collector, &metrics_url).await;
            sleep(Duration::from_secs(METRICS_DELAY)).await;
        }
    });

    loop {
        handle_endpoints(&endpoints_url, &endpoints_results_url, &client).await;
        sleep(Duration::from_secs(ENDPOINTS_DELAY)).await;
    }
}

async fn handle_metrics(collector: &mut Collector, url: &str) {
    let metrics = collector.get_metrics();
    let resp = collector.client.post(url).json(&metrics).send().await;

    match resp {
        Ok(val) => {
            if val.status() == reqwest::StatusCode::UNAUTHORIZED {
                let result = collector.try_get_new_id().await;
                if let Err(val) = result {
                    eprintln!("Error while getting collector ID: {}", val)
                }
            }
        }
        Err(val) => {
            eprintln!("Error: {}", val);
        }
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

    // send requests to user-specified endpoints
    for e in endpoints {
        // if timeout passes
        let response = timeout(Duration::from_secs(ENDPOINT_TIMEOUT), e.send(client))
            .await
            .unwrap_or(Err(shared::Error::Elapsed));

        let result = match response {
            Ok(mut val) => {
                val.timestamp = timestamp;
                val
            }
            Err(val) => {
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

pub fn get_supported_systems() -> Vec<String> {
    vec![
        "Android".to_string(),
        "FreeBSD".to_string(),
        "NetBSD".to_string(),
        "iOS".to_string(),
        "Linux".to_string(),
        "macOS".to_string(),
        "Raspberry Pi".to_string(),
        "Windows".to_string(),
    ]
}
