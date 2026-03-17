use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::structs::{disk::Disk, netword_interface::NetworkInterface};

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
