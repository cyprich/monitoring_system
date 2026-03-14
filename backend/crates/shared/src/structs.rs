use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Host {
    pub system_name: String,
    pub host_name: String,
    pub kernel_version: String,
    pub total_mem: u64,
    pub cpu_count: usize,
}

impl Host {
    pub fn new(
        system_name: String,
        host_name: String,
        kernel_version: String,
        total_mem: u64,
        cpu_count: usize,
    ) -> Host {
        Host {
            system_name,
            host_name,
            kernel_version,
            total_mem,
            cpu_count,
        }
    }
}
