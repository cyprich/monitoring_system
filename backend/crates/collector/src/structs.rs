use shared::structs::{Disk, Host, Metrics, NetworkInterface};

use crate::UNKNOWN;

pub struct Collector {
    sysinfo: sysinfo::System,
    disks: sysinfo::Disks,
    networks: sysinfo::Networks,
    pub host: Host,
}

impl Collector {
    pub fn new() -> Option<Collector> {
        if !sysinfo::IS_SUPPORTED_SYSTEM {
            return None;
        }

        let sysinfo = sysinfo::System::new_all();
        let used_memory = sysinfo.used_memory();
        let cpu_count = sysinfo.cpus().len();

        Some(Collector {
            sysinfo,
            disks: sysinfo::Disks::new(),
            networks: sysinfo::Networks::new(),
            host: Host::new(
                sysinfo::System::name().unwrap_or(UNKNOWN.to_string()),
                sysinfo::System::host_name().unwrap_or(UNKNOWN.to_string()),
                sysinfo::System::kernel_long_version(),
                used_memory,
                cpu_count,
            ),
        })
    }

    pub fn is_supported_system() -> bool {
        sysinfo::IS_SUPPORTED_SYSTEM
    }

    pub fn get_metrics(&mut self) -> Metrics {
        self.sysinfo.refresh_memory();
        self.sysinfo.refresh_cpu_usage();
        self.disks.refresh(true);
        self.networks.refresh(true);

        Metrics {
            // TODO treba ten clone?
            hostname: self.host.host_name.clone(),
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
