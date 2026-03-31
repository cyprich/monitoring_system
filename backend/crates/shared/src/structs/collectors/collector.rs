use crate::structs::{
    CollectorConfig, UnidentifiedCollector, collectors,
    db::{CollectorTable, DriveTable, NetworkInterfaceTable},
    endpoints::Endpoint,
};
use serde::{Deserialize, Serialize};

use crate::structs::metrics::{DriveMetrics, Metrics, NetworkInterfaceMetrics};

#[derive(Serialize, Deserialize)]
pub struct Collector {
    pub id: i32,
    pub name: String,
    pub system_name: String,
    pub host_name: String,
    pub kernel_version: String,
    pub total_memory_mb: u64,
    pub total_swap_mb: u64,
    pub cpu_count: usize,
    pub drives: Vec<collectors::Drive>,
    pub network_interfaces: Vec<collectors::NetworkInterface>,

    #[serde(skip)]
    pub client: reqwest::Client,
    #[serde(skip)]
    sysinfo_system: sysinfo::System,
    #[serde(skip)]
    sysinfo_drives: sysinfo::Disks,
    #[serde(skip)]
    sysinfo_network_interfaces: sysinfo::Networks,
}

impl Collector {
    pub fn from_unidentified(unidentified_collector: UnidentifiedCollector, id: i32) -> Collector {
        let u = unidentified_collector;
        let (sysinfo_system, sysinfo_drives, sysinfo_network_interfaces) =
            Collector::default_sysinfo();

        Collector {
            id,
            name: u.name,
            system_name: u.system_name,
            host_name: u.host_name,
            kernel_version: u.kernel_version,
            total_memory_mb: u.total_memory_mb,
            total_swap_mb: u.total_swap_mb,
            cpu_count: u.cpu_count,
            drives: u.drives,
            network_interfaces: u.network_interfaces,
            sysinfo_system,
            sysinfo_drives,
            sysinfo_network_interfaces,
            client: u.client,
        }
    }

    pub async fn from_db(
        collector: CollectorTable,
        drives: Vec<DriveTable>,
        network_interfaces: Vec<NetworkInterfaceTable>,
    ) -> Collector {
        let c = collector;
        let (sysinfo_system, sysinfo_drives, sysinfo_network_interfaces) =
            Collector::default_sysinfo();

        Collector {
            id: c.id,
            name: c.name,
            system_name: c.system_name,
            host_name: c.host_name,
            kernel_version: c.kernel_version,
            total_memory_mb: c.total_memory_mb as u64,
            total_swap_mb: c.total_swap_mb as u64,
            cpu_count: c.cpu_count as usize,
            drives: drives.iter().map(collectors::Drive::from).collect(),
            network_interfaces: network_interfaces
                .iter()
                .map(collectors::NetworkInterface::from)
                .collect(),
            sysinfo_system,
            sysinfo_drives,
            sysinfo_network_interfaces,
            client: reqwest::Client::new(),
        }
    }

    pub fn get_metrics(&mut self) -> Metrics {
        self.sysinfo_system.refresh_memory();
        self.sysinfo_system.refresh_cpu_usage();
        self.sysinfo_drives.refresh(true);
        self.sysinfo_network_interfaces.refresh(true);

        Metrics {
            collector_id: self.id,
            timestamp: chrono::Local::now().naive_local(),
            used_memory_mb: self.sysinfo_system.used_memory() / 1_000_000,
            used_swap_mb: self.sysinfo_system.used_swap() / 1_000_000,
            cpu_usage: self.sysinfo_system.global_cpu_usage(),
            // TODO
            drives: self
                .sysinfo_drives
                .iter()
                .map(|d| DriveMetrics {
                    mountpoint: d.mount_point().to_string_lossy().to_string(),
                    used_space_gb: (d.total_space() - d.available_space()) / 1_000_000_000,
                })
                .collect(),
            network_interfaces: self
                .sysinfo_network_interfaces
                .iter()
                .map(|(name, data)| NetworkInterfaceMetrics {
                    name: name.to_string(),
                    upload_kb: data.transmitted() / 1_000,
                    download_kb: data.received() / 1_000,
                })
                .collect(),
        }
    }

    pub async fn try_get_new_id(&mut self) -> Result<(), crate::Error> {
        let uc = UnidentifiedCollector::from(&*self);
        let id = uc.register_to_api().await?;

        self.id = id;

        let mut config = CollectorConfig::load()?;
        config.id = Some(id);
        config.save()?;

        Ok(())
    }

    fn default_sysinfo() -> (sysinfo::System, sysinfo::Disks, sysinfo::Networks) {
        (
            sysinfo::System::new_all(),
            sysinfo::Disks::new(),
            sysinfo::Networks::new(),
        )
    }

    pub async fn get_endpoints(&self) -> Result<Vec<Endpoint>, crate::Error> {
        let url = format!(
            "{}/collector/{}/endpoints",
            crate::env::base_url()?,
            self.id
        );

        let resp = self.client.get(url).send().await?;
        let endpoints = resp.json::<Vec<Endpoint>>().await?;
        Ok(endpoints)
    }
}
