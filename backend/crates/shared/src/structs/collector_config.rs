use std::fs;

use serde::{Deserialize, Serialize};

use crate::{CONFIG_FILENAME, error::CollectorError};

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectorConfig {
    pub id: Option<i32>,
}

// TODO error message on erro would be nice
impl CollectorConfig {
    pub fn load() -> Result<CollectorConfig, CollectorError> {
        let text = fs::read_to_string(CONFIG_FILENAME)?;

        toml::from_str(&text).map_err(|val| CollectorError::ConfigDeserialize(val))
    }

    pub fn save(&self) -> Result<(), CollectorError> {
        let text = toml::to_string(self);
        match text {
            Ok(val) => {
                fs::write(CONFIG_FILENAME, val)?;
                Ok(())
            }
            Err(val) => Err(CollectorError::ConfigSerialize(val)),
        }
    }
}
