use shared::metrics::{Disk, Metrics, NetworkInterface};

use crate::{UNKNOWN, unwrap_or_unknown};

pub struct Collector {
    pub sys: sysinfo::System,
    pub system_name: String,
    pub host_name: String,
    pub kernel_version: String,
    pub disks: sysinfo::Disks,
    pub networks: sysinfo::Networks,
}

impl Collector {
    pub fn new() -> Option<Collector> {
        match sysinfo::IS_SUPPORTED_SYSTEM {
            false => None,
            true => Some(Collector {
                sys: sysinfo::System::new_all(),
                system_name: unwrap_or_unknown(sysinfo::System::name(), "System Name"),
                host_name: unwrap_or_unknown(sysinfo::System::host_name(), "Host Name"),
                kernel_version: sysinfo::System::kernel_long_version(),
                disks: sysinfo::Disks::new(),
                networks: sysinfo::Networks::new(),
            }),
        }
    }

    pub fn get_metrics(&mut self) -> Metrics {
        self.sys.refresh_all();
        self.disks.refresh(true);
        self.networks.refresh(true);

        Metrics {
            system_name: self.system_name.clone(),
            host_name: self.host_name.clone(),
            kernel_version: self.kernel_version.clone(),
            total_mem: self.sys.total_memory(),
            used_mem: self.sys.used_memory(),
            cpu_count: self.sys.cpus().len(),
            cpu_usage: self.sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect(),
            disks: self
                .disks
                .iter()
                .map(|d| Disk {
                    name: d.name().to_str().unwrap_or(UNKNOWN).to_string(),
                    mountpoint: d.mount_point().to_str().unwrap_or(UNKNOWN).to_string(),
                    filesystem: d.file_system().to_str().unwrap_or(UNKNOWN).to_string(),
                    total_space: d.total_space(),
                    available_space: d.available_space(),
                })
                .collect(),
            networks: self
                .networks
                .iter()
                .map(|(name, data)| NetworkInterface {
                    name: name.to_string(),
                    mac: data.mac_address().to_string(),
                    upload: data.transmitted(),
                    download: data.received(),
                    total_upload: data.total_transmitted(),
                    total_download: data.total_received(),
                })
                .collect(),
        }
    }
}
