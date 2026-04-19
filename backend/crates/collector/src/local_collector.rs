use shared::structs::collector_info::{CollectorInfo, DriveInfo, NetworkInterfaceInfo};
use shared::structs::endpoints::Endpoint;
use shared::structs::metrics::{DriveMetrics, Metrics, NetworkInterfaceMetrics};
use shared::traits::Collector;
use sysinfo::DiskRefreshKind;

const UNKNOWN: &str = "<<unknown>>";

pub struct LocalCollector {
    pub id: Option<i32>,
    name: String,
    system_name: String,
    host_name: String,
    kernel_version: String,
    total_memory_mb: u64,
    total_swap_mb: u64,
    cpu_count: usize,
    // drives: Vec<Drive>,
    // network_interfaces: Vec<NetworkInterface>,
    base_url: String,
    client: reqwest::Client,
    sysinfo_system: sysinfo::System,
    sysinfo_drives: sysinfo::Disks,
    sysinfo_network_interfaces: sysinfo::Networks,
}

// implementations
#[async_trait::async_trait]
impl Collector for LocalCollector {
    fn get_info(&self) -> CollectorInfo {
        CollectorInfo {
            id: self.id,
            name: self.name.clone(),
            system_name: self.system_name.clone(),
            host_name: self.host_name.clone(),
            kernel_version: self.kernel_version.clone(),
            total_memory_mb: self.total_memory_mb,
            total_swap_mb: self.total_swap_mb,
            cpu_count: self.cpu_count,
            drives: self.sysinfo_drives.iter().map(DriveInfo::from).collect(),
            network_interfaces: self
                .sysinfo_network_interfaces
                .iter()
                .map(|n| NetworkInterfaceInfo::from((n.0.as_str(), n.1)))
                .collect(),
        }
    }

    fn get_metrics(&mut self) -> Metrics {
        self.sysinfo_system.refresh_memory();
        self.sysinfo_system.refresh_cpu_usage();
        self.sysinfo_drives
            .refresh_specifics(false, DiskRefreshKind::nothing().with_storage());
        self.sysinfo_network_interfaces.refresh(false);

        Metrics {
            collector_id: self.id.unwrap(),
            time: chrono::Utc::now(),
            used_memory_mb: self.sysinfo_system.used_memory() / 1_000_000,
            used_swap_mb: self.sysinfo_system.used_swap() / 1_000_000,
            cpu_usage: self.sysinfo_system.global_cpu_usage(),
            drives: self.sysinfo_drives.iter().map(DriveMetrics::from).collect(),
            network_interfaces: self
                .sysinfo_network_interfaces
                .iter()
                .map(|n| NetworkInterfaceMetrics::from((n.0.as_str(), n.1)))
                .collect(),
        }
    }

    async fn get_endpoints(&self) -> Result<Vec<Endpoint>, shared::Error> {
        let url = format!("{}/collector/{}/endpoints", self.base_url, self.id.unwrap());

        let resp = self.client.get(url).send().await?;
        let endpoints = resp.json::<Vec<Endpoint>>().await?;
        Ok(endpoints)
    }

    fn get_id(&self) -> Option<i32> {
        self.id
    }

    fn set_id(&mut self, id: i32) {
        self.id = Some(id)
    }

    fn get_client(&self) -> &reqwest::Client {
        &self.client
    }

    fn get_base_url(&self) -> &str {
        &self.base_url
    }
}

// basic functions
impl LocalCollector {
    pub fn new() -> Result<Self, shared::Error> {
        // see if system is supported
        if !sysinfo::IS_SUPPORTED_SYSTEM {
            eprintln!("System is not supported!");
            return Err(shared::Error::UnsupportedSystem);
        }

        let sysinfo_system = sysinfo::System::new_all();
        let sysinfo_drives = sysinfo::Disks::new_with_refreshed_list();
        let sysinfo_network_interfaces = sysinfo::Networks::new_with_refreshed_list();
        let base_url = shared::env::base_url().unwrap();

        // `name` and `host_name` are the same by default
        Ok(Self {
            id: None,
            name: sysinfo::System::host_name().unwrap_or(UNKNOWN.to_string()),
            system_name: sysinfo::System::name().unwrap_or(UNKNOWN.to_string()),
            host_name: sysinfo::System::host_name().unwrap_or(UNKNOWN.to_string()),
            kernel_version: sysinfo::System::kernel_long_version(),
            total_memory_mb: sysinfo_system.total_memory() / 1_000_000,
            total_swap_mb: sysinfo_system.total_swap() / 1_000_000,
            cpu_count: sysinfo_system.cpus().len(),
            base_url,
            client: reqwest::Client::new(),
            sysinfo_system,
            sysinfo_drives,
            sysinfo_network_interfaces,
        })
    }
}
