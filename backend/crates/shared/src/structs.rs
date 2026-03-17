use std::fs;

use chrono::NaiveDateTime;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};

use crate::{BASE_URL, CONFIG_FILENAME, UNKNOWN, UNNAMED};

// TODO split this file into multiple

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Disk {
    pub mountpoint: String,
    pub available_space: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub name: String,
    pub upload: u64,
    pub download: u64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Metrics {
    pub collector_id: i32,
    pub hostname: String,
    pub timestamp: NaiveDateTime,
    pub used_mem: u64,
    pub cpu_usage: f32,
    pub disks: Vec<Disk>,
    pub networks: Vec<NetworkInterface>,
}

impl Metrics {
    pub fn new() -> Metrics {
        Metrics::default()
    }

    pub fn json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectorConfig {
    pub id: Option<i32>,
}

// TODO error message on erro would be nice
impl CollectorConfig {
    pub fn load() -> Option<CollectorConfig> {
        let text = fs::read_to_string(CONFIG_FILENAME);
        match text {
            Ok(val) => toml::from_str(&val).ok(),
            Err(_) => None,
        }
    }

    pub fn save(&self) -> bool {
        let text = toml::to_string(self);
        match text {
            Ok(val) => fs::write(CONFIG_FILENAME, val).is_ok(),
            Err(_) => false,
        }
    }
}

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

#[derive(Serialize, Deserialize)]
pub struct Collector {
    id: i32,
    name: String,
    system_name: String,
    host_name: String,
    kernel_version: String,
    total_mem: u64,
    cpu_count: usize,
    #[serde(skip)]
    sysinfo: sysinfo::System,
    #[serde(skip)]
    disks: sysinfo::Disks,
    #[serde(skip)]
    networks: sysinfo::Networks,
}

impl Collector {
    pub fn is_supported_system() -> bool {
        sysinfo::IS_SUPPORTED_SYSTEM
    }

    pub fn get_metrics(&mut self) -> Metrics {
        self.sysinfo.refresh_memory();
        self.sysinfo.refresh_cpu_usage();
        self.disks.refresh(true);
        self.networks.refresh(true);

        Metrics {
            // todo
            collector_id: self.id,
            // TODO treba ten clone?
            hostname: self.host_name.clone(),
            timestamp: chrono::Local::now().naive_local(),
            used_mem: self.sysinfo.used_memory(),
            cpu_usage: self.sysinfo.global_cpu_usage(),
            disks: self
                .disks
                .iter()
                .map(|d| Disk {
                    mountpoint: d.mount_point().to_string_lossy().to_string(),
                    available_space: d.available_space(),
                })
                .collect(),
            networks: self
                .networks
                .iter()
                .map(|(name, data)| NetworkInterface {
                    name: name.to_string(),
                    upload: data.transmitted(),
                    download: data.received(),
                })
                .collect(),
        }
    }
}
