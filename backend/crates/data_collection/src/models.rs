use serde::{Deserialize, Serialize};

use crate::{UNKNOWN, unwrap_or_unknown};

pub struct System {
    pub sys: sysinfo::System,
    pub system_name: String,
    pub host_name: String,
    pub kernel_version: String,
    pub disks: sysinfo::Disks,
    pub networks: sysinfo::Networks,
}

impl System {
    pub fn new() -> Option<System> {
        match sysinfo::IS_SUPPORTED_SYSTEM {
            false => None,
            true => Some(System {
                sys: sysinfo::System::new_all(),
                system_name: unwrap_or_unknown(sysinfo::System::name(), "System Name"),
                host_name: unwrap_or_unknown(sysinfo::System::host_name(), "Host Name"),
                kernel_version: sysinfo::System::kernel_long_version(),
                disks: sysinfo::Disks::new(),
                networks: sysinfo::Networks::new(),
            }),
        }
    }

    pub fn get_data(&mut self) -> Data {
        self.sys.refresh_all();
        self.disks.refresh(true);
        self.networks.refresh(true);

        Data {
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
pub struct Data {
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

impl Data {
    pub fn new() -> Data {
        Data::default()
    }
}
