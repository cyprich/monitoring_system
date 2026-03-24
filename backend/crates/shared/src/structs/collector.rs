use serde::{Deserialize, Serialize};

use crate::{
    error::CollectorError,
    structs::{
        collector_config::CollectorConfig, disk::Disk, metrics::Metrics,
        netword_interface::NetworkInterface, unidentified_collector::UnidentifiedCollector,
    },
};

#[derive(Serialize, Deserialize)]
pub struct Collector {
    pub id: i32,
    pub name: String,
    pub system_name: String,
    pub host_name: String,
    pub kernel_version: String,
    pub total_memory_mb: u64,
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
            collector_id: self.id,
            timestamp: chrono::Local::now().naive_local(),
            used_memory_mb: self.sysinfo.used_memory() / 1_000_000,
            used_swap_mb: self.sysinfo.used_memory() / 1_000_000,
            cpu_usage: self.sysinfo.global_cpu_usage(),
            disks: self
                .disks
                .iter()
                .map(|d| Disk {
                    mountpoint: d.mount_point().to_string_lossy().to_string(),
                    available_space_mb: d.available_space() / 1_000_000,
                })
                .collect(),
            networks: self
                .networks
                .iter()
                .map(|(name, data)| NetworkInterface {
                    name: name.to_string(),
                    upload_mb: data.transmitted() / 1_000_000,
                    download_mb: data.received() / 1_000_000,
                })
                .collect(),
        }
    }

    pub async fn try_get_new_id(&mut self) -> Result<(), CollectorError> {
        let uc = UnidentifiedCollector::from(&*self);
        let id = uc.register_to_api().await.ok_or(CollectorError::Api())?;

        self.id = id;

        let mut config = CollectorConfig::load()?;
        config.id = Some(id);
        config.save()?;

        Ok(())
    }
}
