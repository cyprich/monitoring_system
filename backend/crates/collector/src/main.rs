use std::time::Duration;

use shared::structs::unidentified_collector::UnidentifiedCollector;
use tokio::time::sleep;

const DELAY: u64 = 5;
// TODO temp
const BASE_URL: &str = "http://localhost:5000";

#[tokio::main]
pub async fn main() {
    // TODO
    if !sysinfo::IS_SUPPORTED_SYSTEM {
        eprintln!("System is not supported!");
        eprintln!("These systems are supported: ");
        get_supported_systems()
            .iter()
            .for_each(|s| eprintln!("  {}", s));
        panic!()
    }

    let uc = UnidentifiedCollector::new();
    // TODO
    let mut collector = uc.identify().await.unwrap();

    let client = reqwest::Client::new();

    // TODO
    // let port = shared::get_env("API_PORT").unwrap();
    // let port: u16 = port.parse().expect("Couldn't convert port ({port}) to u16");
    // let url = format!("http://localhost:{port}/metrics");
    let url = format!("{BASE_URL}/metrics");

    loop {
        let metrics = collector.get_metrics();
        let resp = client.post(&url).json(&metrics).send().await;

        if let Err(e) = resp {
            println!("Unsuccessful POST: {}", e);
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
