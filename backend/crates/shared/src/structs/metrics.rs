use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Metrics {
    pub collector_id: i32,
    pub timestamp: NaiveDateTime,
    pub used_memory_mb: u64,
    pub used_swap_mb: u64,
    pub cpu_usage_global: f32,
    pub cpu_usage_cores: Vec<f32>,
    pub drives: Vec<DriveMetrics>,
    pub network_interfaces: Vec<NetworkInterfaceMetrics>,
}

impl Metrics {
    pub fn new() -> Metrics {
        Metrics::default()
    }

    pub fn json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveMetrics {
    pub mountpoint: String,
    pub used_space_gb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterfaceMetrics {
    pub name: String,
    pub upload_kb: u64,
    pub download_kb: u64,
}
