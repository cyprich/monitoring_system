use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Metrics {
    pub collector_id: i32,
    pub timestamp: NaiveDateTime,
    pub used_memory_mb: u64,
    pub used_swap_mb: u64,
    pub cpu_usage: f32,
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

impl From<&sysinfo::Disk> for DriveMetrics {
    fn from(value: &sysinfo::Disk) -> Self {
        Self {
            mountpoint: value.mount_point().to_string_lossy().to_string(),
            used_space_gb: (value.total_space() - value.available_space()) / 1_000_000_000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterfaceMetrics {
    pub name: String,
    pub upload_kb: u64,
    pub download_kb: u64,
}

impl From<(&str, &sysinfo::NetworkData)> for NetworkInterfaceMetrics {
    fn from(value: (&str, &sysinfo::NetworkData)) -> Self {
        Self {
            name: value.0.to_string(),
            upload_kb: value.1.transmitted() / 1_000,
            download_kb: value.1.received() / 1_000,
        }
    }
}
