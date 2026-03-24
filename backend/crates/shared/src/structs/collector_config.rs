use std::fs;

use serde::{Deserialize, Serialize};

use crate::CONFIG_FILENAME;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CollectorConfig {
    pub id: Option<i32>,
}

// TODO error message on erro would be nice
impl CollectorConfig {
    pub fn load() -> Result<CollectorConfig, crate::Error> {
        let text = fs::read_to_string(CONFIG_FILENAME)?;
        Ok(toml::from_str(&text)?)
    }

    pub fn save(&self) -> Result<(), crate::Error> {
        let text = toml::to_string(self);
        match text {
            Ok(val) => {
                fs::write(CONFIG_FILENAME, val)?;
                Ok(())
            }
            Err(val) => Err(val.into()),
        }
    }
}
