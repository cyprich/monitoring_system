use std::{collections::BTreeMap, str::FromStr};

use shared::{
    enums::metric_type::MetricType,
    structs::{
        db::MetricsTable,
        metrics::{DriveMetrics, Metrics, NetworkInterfaceMetrics},
    },
};
use sqlx::types::chrono::NaiveDateTime;

use crate::{
    AppState,
    db::{Builder, Pool},
    notifications,
};

pub async fn insert_metrics(state: &AppState, metrics: &Metrics) -> Result<(), shared::Error> {
    let mut builder = Builder::new(
        "insert into metrics (timestamp, value, metric_type, collector_id, component_name) ",
    );

    let mut values = vec![
        (
            metrics.cpu_usage as f64,
            MetricType::CpuUsage,
            String::default(),
        ),
        (
            metrics.used_memory_mb as f64,
            MetricType::UsedMemoryMb,
            String::default(),
        ),
        (
            metrics.used_swap_mb as f64,
            MetricType::UsedSwapMb,
            String::default(),
        ),
    ];

    // drives
    for d in metrics.drives.clone() {
        values.push((
            d.used_space_gb as f64,
            MetricType::DriveUsedSpace,
            d.mountpoint,
        ));
    }

    // network_interfaces
    for n in metrics.network_interfaces.clone() {
        values.push((
            n.download_kb as f64,
            MetricType::NetworkDownload,
            n.name.clone(),
        ));
        values.push((n.upload_kb as f64, MetricType::NetworkUpload, n.name));
    }

    builder.push_values(values, |mut b, val| {
        b.push_bind(metrics.timestamp)
            .push_bind(val.0)
            .push_bind(val.1.to_string())
            .push_bind(metrics.collector_id)
            .push_bind(val.2);
    });

    builder.build().execute(&state.pool).await?;

    // notifications
    let notifications = notifications::handle_metrics(state, metrics.collector_id).await;
    if let Err(val) = notifications {
        eprintln!("Error with metrics notifications: {}", val);
    }

    Ok(())
}

pub async fn get_metrics_table(
    pool: &Pool,
    collector_id: i32,
    limit: Option<i32>,
) -> Result<Vec<MetricsTable>, shared::Error> {
    let mut builder = Builder::new("select * from metrics where collector_id = ");
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

pub async fn get_metrics(
    pool: &Pool,
    collector_id: i32,
    limit: Option<i32>,
) -> Result<Vec<Metrics>, shared::Error> {
    let result = get_metrics_table(pool, collector_id, limit).await?;

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
