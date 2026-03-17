use serde::{Deserialize, Serialize};

use crate::structs::{disk::Disk, metrics::Metrics, netword_interface::NetworkInterface};

#[derive(Serialize, Deserialize)]
pub struct Collector {
    pub id: i32,
    pub name: String,
    pub system_name: String,
    pub host_name: String,
    pub kernel_version: String,
    pub total_mem: u64,
    pub cpu_count: usize,
    #[serde(skip)]
    pub sysinfo: sysinfo::System,
    #[serde(skip)]
    pub disks: sysinfo::Disks,
    #[serde(skip)]
    pub networks: sysinfo::Networks,
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
