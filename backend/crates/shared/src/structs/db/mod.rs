use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectorTable {
    pub id: i32,
    pub name: String,
    pub system_name: String,
    pub host_name: String,
    pub kernel_version: String,
    pub total_memory_mb: i32,
    pub total_swap_mb: i32,
    pub cpu_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DriveTable {
    pub mountpoint: String,
    pub collector_id: i32,
    pub capacity_gb: i32,
    pub file_system: String,
}

// impl DriveTable {
//     fn from_drive(drive: &collectors::Drive, id: i32) -> Self {
//         Self {
//             mountpoint: drive.mountpoint.clone(),
//             collector_id: id,
//             capacity_gb: drive.capacity_gb as i32,
//             file_system: drive.file_system.clone(),
//         }
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkInterfaceTable {
    pub name: String,
    pub mac: String,
    pub collector_id: i32,
}

// impl NetworkInterfaceTable {
//     fn from_network_interface(network_interface: &collectors::NetworkInterface, id: i32) -> Self {
//         Self {
//             name: network_interface.name.clone(),
//             mac: network_interface.mac.clone(),
//             collector_id: id,
//         }
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricTypeTable {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct MetricsTable {
    pub timestamp: NaiveDateTime,
    pub value: f64,
    pub metric_type: String,
    pub collector_id: i32,
    pub component_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct EndpointTable {
    pub id: i32,
    pub collector_id: i32,
    pub url: String,
    // pub method: RequestMethod,
    pub expected_codes: Vec<i32>,
}

// it has the same fields, only here for compatibility
pub type EndpointResultTable = crate::structs::endpoints::EndpointResult;
