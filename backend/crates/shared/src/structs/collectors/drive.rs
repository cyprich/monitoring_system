use serde::{Deserialize, Serialize};

use crate::structs::db::DriveTable;

#[derive(Serialize, Deserialize, Clone)]
pub struct Drive {
    pub mountpoint: String,
    pub capacity_gb: u32,
    pub file_system: String,
}

impl From<&DriveTable> for Drive {
    fn from(value: &DriveTable) -> Self {
        Self {
            mountpoint: value.mountpoint.clone(),
            capacity_gb: value.capacity_gb as u32,
            file_system: value.file_system.clone(),
        }
    }
}
