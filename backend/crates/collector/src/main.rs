use std::time::Duration;

use shared::{CollectorError, structs::unidentified_collector::UnidentifiedCollector};
use tokio::time::sleep;

// TODO make this user-configurable
const DELAY: u64 = 5;

#[tokio::main]
pub async fn main() -> Result<(), CollectorError> {
    // TODO
    if !sysinfo::IS_SUPPORTED_SYSTEM {
        eprintln!("System is not supported!");
        eprintln!("These systems are supported: ");
        get_supported_systems()
            .iter()
            .for_each(|s| eprintln!("  {}", s));
        return Err(CollectorError::UnsupportedSystem);
    }

    let uc = UnidentifiedCollector::new();
    let mut collector = uc.identify().await?;

    let client = reqwest::Client::new();

    let addr = shared::env::get("API_ADDRESS")?;
    let port = shared::env::get("API_PORT")?;
    let url = format!("http://{addr}:{port}/metrics");

    loop {
        let metrics = collector.get_metrics();
        let resp = client.post(&url).json(&metrics).send().await;

        match resp {
            Ok(val) => {
                if val.status() == reqwest::StatusCode::UNAUTHORIZED {
                    let result = collector.try_get_new_id().await;
                    if let Err(val) = result {
                        eprintln!("Failed getting collector ID: {}", val)
                    }
                }
            }
            Err(val) => eprintln!("Error: {}", val),
        }

        sleep(Duration::from_secs(DELAY)).await;
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
