use std::time::Duration;

use reqwest::StatusCode;
use shared::structs::{Collector, UnidentifiedCollector, endpoints::Endpoint};
use tokio::time::sleep;

// TODO make this user-configurable
const DELAY: u64 = 10;

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
    let endpoints = collector.get_endpoints().await?;

    let base_url = shared::env::base_url()?;
    let metrics_url = format!("{}/metrics", base_url);
    let endpoints_url = format!("{}/collector/{}/endpoint_results", base_url, collector.id);

    // TODO use only this client
    let client = reqwest::Client::new();

    loop {
        handle_metrics(&mut collector, &metrics_url).await;
        handle_endpoints(&endpoints, &endpoints_url, &client).await;
        // TODO different delay for endpoints
        sleep(Duration::from_secs(DELAY)).await;
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

async fn handle_endpoints(endpoints: &Vec<Endpoint>, url: &str, client: &reqwest::Client) {
    let mut endpoint_results = vec![];

    for e in endpoints {
        let response = e.send(client).await;
        let result = match response {
            Ok(val) => val,
            Err(val) => {
                eprintln!("Error while checking endpoint: {}", val);
                continue;
            }
        };

        endpoint_results.push(result);
    }

    let resp = client.post(url).json(&endpoint_results).send().await;
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
