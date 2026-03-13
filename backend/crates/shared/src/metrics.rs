use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Disk {
    pub name: String,
    pub mountpoint: String,
    pub filesystem: String,
    pub total_space: u64,
    pub available_space: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub name: String,
    pub mac: String,
    pub upload: u64,
    pub download: u64,
    pub total_upload: u64,
    pub total_download: u64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Metrics {
    pub system_name: String,
    pub host_name: String,
    pub kernel_version: String,
    pub total_mem: u64,
    pub used_mem: u64,
    pub cpu_count: usize,
    pub cpu_usage: Vec<f32>,
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
