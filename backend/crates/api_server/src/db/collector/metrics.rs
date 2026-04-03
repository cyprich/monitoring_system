use std::{collections::BTreeMap, str::FromStr};

use shared::{
    enums::metric_type::MetricType,
    structs::{
        db::MetricsTable,
        metrics::{DriveMetrics, Metrics, NetworkInterfaceMetrics},
    },
};
use sqlx::{Postgres, QueryBuilder, types::chrono::NaiveDateTime};

use crate::Pool;

pub async fn get_collector_metrics_table(
    pool: &Pool,
    collector_id: i32,
    limit: Option<i32>,
) -> Result<Vec<MetricsTable>, shared::Error> {
    let mut builder: QueryBuilder<Postgres> =
        QueryBuilder::new("select * from metrics where collector_id = ");
    builder.push_bind(collector_id);

    if let Some(val) = limit {
        builder.push(
            " and timestamp in (
            select distinct timestamp
            from metrics
            where collector_id = ",
        );
        builder.push_bind(collector_id);
        builder.push(" order by timestamp desc limit ");
        builder.push_bind(val);
        builder.push(" )");
    };

    let result = builder
        .build_query_as::<MetricsTable>()
        .fetch_all(pool)
        .await?;

    Ok(result)
}

pub async fn get_collector_metrics(
    pool: &Pool,
    collector_id: i32,
    limit: Option<i32>,
) -> Result<Vec<Metrics>, shared::Error> {
    let result = get_collector_metrics_table(pool, collector_id, limit).await?;

    let mut map: BTreeMap<NaiveDateTime, Metrics> = BTreeMap::new();

    for row in result {
        let entry = map.entry(row.timestamp).or_insert(Metrics {
            // TODO it's useless to carry collector id, too much overhead
            collector_id,
            timestamp: row.timestamp,
            used_memory_mb: 0,
            used_swap_mb: 0,
            cpu_usage: 0.0,
            drives: vec![],
            network_interfaces: vec![],
        });

        let metric_type = match MetricType::from_str(&row.metric_type) {
            Ok(val) => val,
            Err(val) => {
                eprintln!("Invalid Metric Type value: {}", val);
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
                    Some(val) => {
                        val.download_kb = row.value as u64;
                    }
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
                    Some(val) => {
                        val.upload_kb = row.value as u64;
                    }
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
