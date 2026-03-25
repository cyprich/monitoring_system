use std::time::Duration;

use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};

use crate::{
    UNKNOWN,
    structs::{collector::Collector, collector_config::CollectorConfig},
};

#[derive(Default, Serialize, Deserialize)]
pub struct UnidentifiedCollector {
    pub name: String,
    pub system_name: String,
    pub host_name: String,
    pub kernel_version: String,
    pub total_memory_mb: u64,
    pub total_swap_mb: u64,
    pub cpu_count: usize,
}

impl UnidentifiedCollector {
    pub fn new() -> UnidentifiedCollector {
        let sysinfo = sysinfo::System::new_all();
        let total_memory_mb = sysinfo.total_memory() / 1_000_000;
        let total_swap_mb = sysinfo.total_swap() / 1_000_000;
        let cpu_count = sysinfo.cpus().len();
        let host_name = sysinfo::System::host_name().unwrap_or(UNKNOWN.to_string());

        UnidentifiedCollector {
            name: host_name.clone(),
            system_name: sysinfo::System::name().unwrap_or(UNKNOWN.to_string()),
            host_name,
            kernel_version: sysinfo::System::kernel_version().unwrap_or(UNKNOWN.to_string()),
            total_memory_mb,
            total_swap_mb,
            cpu_count,
        }
    }

    pub async fn identify(self) -> Result<Collector, crate::Error> {
        // idetify from config file
        let config = CollectorConfig::load();
        if let Ok(c) = &config
            && let Some(val) = c.id
        {
            return Ok(self.new_collector(val));
        }

        // if loading from file fails, idetify from api
        let mut config = config.unwrap_or_default();
        let result = self.register_to_api().await;
        match result {
            Ok(val) => {
                config.id = Some(val);
                config.save()?;
                Ok(self.new_collector(val))
            }
            Err(val) => Err(val),
        }
    }

    pub async fn register_to_api(&self) -> Result<i32, crate::Error> {
        let url = format!("{}/collector/register", crate::env::api_address()?);
        let client = Client::new();

        async fn handle_register_api_response(
            resp: Result<reqwest::Response, reqwest::Error>,
        ) -> Result<i32, crate::Error> {
            match resp {
                Ok(val) => match val.status() {
                    StatusCode::CREATED => {
                        let text = val.text().await;
                        match text {
                            Ok(val) => Ok(val.parse::<i32>()?),
                            Err(val) => Err(crate::Error::ReqwestFromString(val.to_string())),
                        }
                    }
                    // TODO handle all possible responses from api server
                    _ => Err(crate::Error::HTTPResponse(val.status().as_u16())),
                },
                Err(val) => Err(crate::Error::ReqwestFromString(val.to_string())),
            }
        }

        let tries = 10;
        for i in 0..tries {
            let resp = client.post(&url).json(self).send().await;
            let result = handle_register_api_response(resp).await;
            match result {
                Ok(val) => return Ok(val),
                Err(val) => {
                    // last try
                    if i == tries - 1 {
                        return Err(val);
                    }

                    eprintln!("Error registering: {}, try: {}/{}", val, i + 1, tries);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        }

        Err(crate::Error::ReqwestFromString(format!(
            "Failed registering collector after {} tries",
            tries
        )))
    }

    fn new_collector(self, id: i32) -> Collector {
        let sysinfo = sysinfo::System::new_all();
        let disks = sysinfo::Disks::new();
        let networks = sysinfo::Networks::new();

        Collector {
            id,
            name: self.name,
            system_name: self.system_name,
            host_name: self.host_name,
            kernel_version: self.kernel_version,
            total_memory_mb: self.total_memory_mb,
            total_swap_mb: self.total_swap_mb,
            cpu_count: self.cpu_count,
            sysinfo,
            disks,
            networks,
        }
    }
}

impl From<&Collector> for UnidentifiedCollector {
    fn from(value: &Collector) -> Self {
        Self {
            name: value.name.clone(),
            system_name: value.system_name.clone(),
            host_name: value.host_name.clone(),
            kernel_version: value.kernel_version.clone(),
            total_memory_mb: value.total_memory_mb,
            total_swap_mb: value.total_swap_mb,
            cpu_count: value.cpu_count,
        }
    }
}
