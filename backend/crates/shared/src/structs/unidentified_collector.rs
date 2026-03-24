use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};

use crate::{
    BASE_URL, UNKNOWN,
    structs::{collector::Collector, collector_config::CollectorConfig},
};

#[derive(Default, Serialize, Deserialize)]
pub struct UnidentifiedCollector {
    pub name: String,
    pub system_name: String,
    pub host_name: String,
    pub kernel_version: String,
    pub total_memory_mb: u64,
    pub cpu_count: usize,
}

impl UnidentifiedCollector {
    pub fn new() -> UnidentifiedCollector {
        let sysinfo = sysinfo::System::new_all();
        let total_memory_mb = sysinfo.total_memory() / 1_000_000;
        let cpu_count = sysinfo.cpus().len();
        let host_name = sysinfo::System::host_name().unwrap_or(UNKNOWN.to_string());

        UnidentifiedCollector {
            name: host_name.clone(),
            system_name: sysinfo::System::name().unwrap_or(UNKNOWN.to_string()),
            host_name,
            kernel_version: sysinfo::System::kernel_version().unwrap_or(UNKNOWN.to_string()),
            total_memory_mb,
            cpu_count,
            // sysinfo,
            // disks: sysinfo::Disks::new(),
            // networks: sysinfo::Networks::new(),
        }
    }

    pub async fn identify(self) -> Option<Collector> {
        // TODO maybe some error on None?

        // idetify from config file
        let mut config = CollectorConfig::load().ok()?;
        if let Some(id) = config.id {
            return self.new_collector(id);
        }

        // idetify from api
        if let Some(id) = self.register_to_api().await {
            config.id = Some(id);
            config.save().ok()?;
            self.new_collector(id)
        } else {
            None
        }
    }

    pub async fn register_to_api(&self) -> Option<i32> {
        // TODO url
        let url = format!("{BASE_URL}/collector/register");
        let client = Client::new();

        let resp = client.post(&url).json(self).send().await;

        // TODO maybe more retries?
        match resp {
            Ok(val) => match val.status() {
                StatusCode::CREATED => {
                    let text = val.text().await;
                    match text {
                        Ok(val) => val.parse().ok(),
                        Err(_) => None,
                    }
                }
                StatusCode::NOT_FOUND => {
                    eprintln!("Endpoint {} not found while registering!", &url);
                    None
                }
                StatusCode::BAD_REQUEST => {
                    eprintln!("Bad request while registering!");
                    None
                }
                _ => None,
            },
            Err(val) => {
                dbg!(val);
                None
            }
        }
    }

    fn new_collector(self, id: i32) -> Option<Collector> {
        let sysinfo = sysinfo::System::new_all();
        let disks = sysinfo::Disks::new();
        let networks = sysinfo::Networks::new();

        Some(Collector {
            id,
            name: self.name,
            system_name: self.system_name,
            host_name: self.host_name,
            kernel_version: self.kernel_version,
            total_memory_mb: self.total_memory_mb,
            cpu_count: self.cpu_count,
            sysinfo,
            disks,
            networks,
        })
    }
}

impl From<&Collector> for UnidentifiedCollector {
    fn from(value: &Collector) -> Self {
        Self {
            name: value.name.clone(),
            system_name: value.system_name.clone(),
            host_name: value.host_name.clone(),
            kernel_version: value.kernel_version.clone(),
            total_memory_mb: value.total_memory_mb,
            cpu_count: value.cpu_count,
        }
    }
}
