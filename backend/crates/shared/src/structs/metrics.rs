use std::{
    collections::{BTreeMap, HashMap},
    str::FromStr,
    vec,
};

use serde::{Deserialize, Serialize};

use crate::{enums::metric_type::MetricType, structs::db::MetricsTable};

#[derive(Debug, Default, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Metrics {
    pub collector_id: i32,
    pub time: chrono::DateTime<chrono::Utc>,
    pub used_memory_mb: u64,
    pub used_swap_mb: u64,
    pub cpu_usage: f32,
    pub drives: Vec<DriveMetrics>,
    pub network_interfaces: Vec<NetworkInterfaceMetrics>,
}

impl Metrics {
    pub fn new() -> Metrics {
        Metrics::default()
    }

    pub fn json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct DriveMetrics {
    pub mountpoint: String,
    pub used_space_gb: u64,
}

impl From<&sysinfo::Disk> for DriveMetrics {
    fn from(value: &sysinfo::Disk) -> Self {
        Self {
            mountpoint: value.mount_point().to_string_lossy().to_string(),
            used_space_gb: (value.total_space() - value.available_space()) / 1_000_000_000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct NetworkInterfaceMetrics {
    pub name: String,
    pub upload_kb: u64,
    pub download_kb: u64,
}

impl From<(&str, &sysinfo::NetworkData)> for NetworkInterfaceMetrics {
    fn from(value: (&str, &sysinfo::NetworkData)) -> Self {
        Self {
            name: value.0.to_string(),
            upload_kb: value.1.transmitted() / 1_000,
            download_kb: value.1.received() / 1_000,
        }
    }
}

impl Metrics {
    pub fn average(value: &[Metrics]) -> Metrics {
        let len = value.len();

        let mut cpu_usage = 0.0;
        let mut used_memory_mb = 0;
        let mut used_swap_mb = 0;

        // <mountpoint, used space>
        let mut drives_map: HashMap<String, u64> = HashMap::new();
        // <name, (upload, download)>
        let mut networks_map: HashMap<String, (u64, u64)> = HashMap::new();

        for m in value {
            cpu_usage += m.cpu_usage;
            used_memory_mb += m.used_memory_mb;
            used_swap_mb += m.used_swap_mb;

            for d in &m.drives {
                drives_map
                    .entry(d.mountpoint.clone())
                    .and_modify(|val| *val += d.used_space_gb)
                    .or_insert(d.used_space_gb);
            }

            for n in &m.network_interfaces {
                networks_map
                    .entry(n.name.clone())
                    .and_modify(|val| {
                        val.0 += n.upload_kb;
                        val.1 += n.download_kb;
                    })
                    .or_insert((n.upload_kb, n.download_kb));
            }
        }

        let mut drives = vec![];
        for (mountpoint, space) in drives_map {
            drives.push(DriveMetrics {
                mountpoint,
                used_space_gb: space / len as u64,
            });
        }

        let mut network_interfaces = vec![];
        for (name, (up, down)) in networks_map {
            network_interfaces.push(NetworkInterfaceMetrics {
                name,
                upload_kb: up / len as u64,
                download_kb: down / len as u64,
            });
        }

        Metrics {
            collector_id: value[0].collector_id,
            time: value[0].time,
            used_memory_mb: used_memory_mb / len as u64,
            used_swap_mb: used_swap_mb / len as u64,
            cpu_usage: cpu_usage / len as f32,
            drives,
            network_interfaces,
        }
    }

    pub fn from_metrics_table(value: Vec<MetricsTable>) -> Result<Vec<Self>, crate::Error> {
        let mut map: BTreeMap<chrono::DateTime<chrono::Utc>, Metrics> = BTreeMap::new();

        for row in value {
            let entry = map.entry(row.time).or_insert(Metrics {
                collector_id: row.collector_id,
                time: row.time,
                used_memory_mb: 0,
                used_swap_mb: 0,
                cpu_usage: 0.0,
                drives: vec![],
                network_interfaces: vec![],
            });

            let metric_type = match MetricType::from_str(&row.metric_type) {
                Ok(val) => val,
                Err(val) => {
                    eprintln!("Invalid Metric Type value: '{}'", val);
                    continue;
                }
            };

            match metric_type {
                MetricType::CpuUsage => entry.cpu_usage = row.value as f32,
                MetricType::UsedMemoryMb => entry.used_memory_mb = row.value as u64,
                MetricType::UsedSwapMb => entry.used_swap_mb = row.value as u64,
                MetricType::DriveUsedSpace => {
                    entry.drives.push(DriveMetrics {
                        mountpoint: row.component_name,
                        used_space_gb: row.value as u64,
                    });
                }
                MetricType::NetworkDownload => {
                    let net = entry
                        .network_interfaces
                        .iter_mut()
                        .find(|n| n.name == row.component_name);

                    match net {
                        Some(val) => val.download_kb = row.value as u64,
                        None => {
                            entry.network_interfaces.push(NetworkInterfaceMetrics {
                                name: row.component_name,
                                upload_kb: 0,
                                download_kb: row.value as u64,
                            });
                        }
                    }
                }
                MetricType::NetworkUpload => {
                    let net = entry
                        .network_interfaces
                        .iter_mut()
                        .find(|n| n.name == row.component_name);

                    match net {
                        Some(val) => val.upload_kb = row.value as u64,
                        None => {
                            entry.network_interfaces.push(NetworkInterfaceMetrics {
                                name: row.component_name,
                                upload_kb: row.value as u64,
                                download_kb: 0,
                            });
                        }
                    }
                }
            }
        }

        Ok(map.into_values().collect())
    }
}
