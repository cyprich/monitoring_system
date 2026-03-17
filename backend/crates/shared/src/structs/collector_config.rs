use std::fs;

use serde::{Deserialize, Serialize};

use crate::CONFIG_FILENAME;

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectorConfig {
    pub id: Option<i32>,
}

// TODO error message on erro would be nice
impl CollectorConfig {
    pub fn load() -> Option<CollectorConfig> {
        let text = fs::read_to_string(CONFIG_FILENAME);
        match text {
            Ok(val) => toml::from_str(&val).ok(),
            Err(_) => None,
        }
    }

    pub fn save(&self) -> bool {
        let text = toml::to_string(self);
        match text {
            Ok(val) => fs::write(CONFIG_FILENAME, val).is_ok(),
            Err(_) => false,
        }
    }
}
