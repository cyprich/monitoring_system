mod structs;

use std::time::Duration;

use tokio::time::sleep;

pub use crate::structs::Collector;

const UNKNOWN: &str = "<<unknown>>";

#[tokio::main]
pub async fn main() {
    // TODO
    let mut collector = Collector::new().unwrap();
    let client = reqwest::Client::new();

    // TODO
    let port = shared::get_env("API_PORT").unwrap();
    let port: u16 = port.parse().expect("Couldn't convert {port} to u16");
    let url = format!("http://localhost:{port}/metrics");

    loop {
        let m = collector.get_metrics();
        let resp = client.post(&url).json(&m).send().await;

        if let Err(e) = resp {
            println!("Unsuccessful POST: {}", e);
        }

        sleep(Duration::from_secs(1)).await;
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

fn unwrap_or_unknown(val: Option<String>, variable_name: &str) -> String {
    match val {
        Some(val) => val,
        None => {
            log::warn!("Couldn't get {}", variable_name);
            UNKNOWN.to_string()
        }
    }
}
