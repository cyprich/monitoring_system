use serde::{Deserialize, Serialize};

// TODO rename to drive?
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Disk {
    pub mountpoint: String,
    pub available_space_mb: u64,
}
