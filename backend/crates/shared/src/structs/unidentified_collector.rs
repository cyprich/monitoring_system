use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};

use crate::{
    BASE_URL, UNKNOWN, UNNAMED,
    structs::{collector::Collector, collector_config::CollectorConfig},
};

#[derive(Default, Serialize, Deserialize)]
pub struct UnidentifiedCollector {
    pub name: String,
    pub system_name: String,
    pub host_name: String,
    pub kernel_version: String,
    pub total_memory: u64,
    pub cpu_count: usize,
    #[serde(skip)]
    sysinfo: sysinfo::System,
    #[serde(skip)]
    disks: sysinfo::Disks,
    #[serde(skip)]
    networks: sysinfo::Networks,
}

impl UnidentifiedCollector {
    pub fn new() -> UnidentifiedCollector {
        let sysinfo = sysinfo::System::new_all();
        let total_memory = sysinfo.total_memory();
        let cpu_count = sysinfo.cpus().len();

        UnidentifiedCollector {
            name: UNNAMED.to_string(),
            system_name: sysinfo::System::name().unwrap_or(UNKNOWN.to_string()),
            host_name: sysinfo::System::host_name().unwrap_or(UNKNOWN.to_string()),
            kernel_version: sysinfo::System::kernel_version().unwrap_or(UNKNOWN.to_string()),
            total_memory,
            cpu_count,
            sysinfo,
            disks: sysinfo::Disks::new(),
            networks: sysinfo::Networks::new(),
        }
    }

    pub async fn identify(self) -> Option<Collector> {
        // TODO maybe some error on None?
        let config = CollectorConfig::load();

        if let Some(val) = &config
            && let Some(id) = val.id
        {
            return self.new_collector(id);
        }

        if let Some(id) = self.register_to_api().await {
            let config = if let Some(mut config) = config {
                config.id = Some(id);
                config
            } else {
                CollectorConfig { id: Some(id) }
            };

            config.save();

            self.new_collector(id)
        } else {
            None
        }
    }

    async fn register_to_api(&self) -> Option<i32> {
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
        Some(Collector {
            id,
            name: self.name,
            system_name: self.system_name,
            host_name: self.host_name,
            kernel_version: self.kernel_version,
            total_mem: self.total_memory,
            cpu_count: self.cpu_count,
            sysinfo: self.sysinfo,
            disks: self.disks,
            networks: self.networks,
        })
    }
}
