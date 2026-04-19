use serde::{Deserialize, Serialize};
use std::hash::Hash;

use crate::structs::db::PortsTable;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Port {
    pub address: String,
    pub port: u16,
    pub protocol: String,
    pub last_update: chrono::DateTime<chrono::Utc>,
}

impl From<&PortsTable> for Port {
    fn from(value: &PortsTable) -> Self {
        Self {
            address: value.address.clone(),
            port: value.port as u16,
            protocol: value.protocol.clone(),
            last_update: value.last_update,
        }
    }
}

impl PartialEq for Port {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address && self.port == other.port && self.protocol == other.protocol
    }
}

impl Eq for Port {}

impl Hash for Port {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.address.hash(state);
        self.port.hash(state);
        self.protocol.hash(state);
    }
}
