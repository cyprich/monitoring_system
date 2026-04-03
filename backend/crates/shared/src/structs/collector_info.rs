use serde::{Deserialize, Serialize};

use crate::structs::db::DriveTable;
use crate::structs::db::NetworkInterfaceTable;

// todo
#[derive(Serialize, Deserialize)]
pub struct CollectorInfo {
    pub id: i32,
    pub name: String,
    pub system_name: String,
    pub host_name: String,
    pub kernel_version: String,
    pub total_memory_mb: u64,
    pub total_swap_mb: u64,
    pub cpu_count: usize,
    pub drives: Vec<DriveInfo>,
    pub network_interfaces: Vec<NetworkInterfaceInfo>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DriveInfo {
    pub mountpoint: String,
    pub capacity_gb: u32,
    pub file_system: String,
}

impl From<&DriveTable> for DriveInfo {
    fn from(value: &DriveTable) -> Self {
        Self {
            mountpoint: value.mountpoint.clone(),
            capacity_gb: value.capacity_gb as u32,
            file_system: value.file_system.clone(),
        }
    }
}

impl From<&sysinfo::Disk> for DriveInfo {
    fn from(value: &sysinfo::Disk) -> Self {
        Self {
            mountpoint: value.mount_point().to_string_lossy().to_string(),
            capacity_gb: (value.total_space() / 1_000_000_000) as u32,
            file_system: value.file_system().to_string_lossy().to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NetworkInterfaceInfo {
    pub name: String,
    pub mac: String,
}

impl From<&NetworkInterfaceTable> for NetworkInterfaceInfo {
    fn from(value: &NetworkInterfaceTable) -> Self {
        Self {
            name: value.name.clone(),
            mac: value.mac.clone(),
        }
    }
}

impl From<(&str, &sysinfo::NetworkData)> for NetworkInterfaceInfo {
    fn from(value: (&str, &sysinfo::NetworkData)) -> Self {
        Self {
            name: value.0.to_string(),
            mac: value.1.mac_address().to_string(),
        }
    }
}
