use serde::{Deserialize, Serialize};

use crate::structs::db::NetworkInterfaceTable;

#[derive(Serialize, Deserialize, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub mac: String,
}

impl From<&NetworkInterfaceTable> for NetworkInterface {
    fn from(value: &NetworkInterfaceTable) -> Self {
        Self {
            name: value.name.clone(),
            mac: value.mac.clone(),
        }
    }
}
