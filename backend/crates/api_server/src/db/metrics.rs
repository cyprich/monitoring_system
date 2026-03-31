use shared::structs::{metric_type_enum::MetricTypeEnum, metrics::Metrics};
use sqlx::{Postgres, QueryBuilder};

use crate::{Pool, notifications};

pub async fn insert_metrics(pool: &Pool, metrics: &Metrics) -> Result<(), shared::Error> {
    let mut builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "insert into metrics (timestamp, value, metric_type, collector_id, component_name) ",
    );

    let mut values = vec![
        (
            metrics.cpu_usage as f64,
            MetricTypeEnum::CpuUsage,
            String::default(),
        ),
        (
            metrics.used_memory_mb as f64,
            MetricTypeEnum::UsedMemoryMb,
            String::default(),
        ),
        (
            metrics.used_swap_mb as f64,
            MetricTypeEnum::UsedSwapMb,
            String::default(),
        ),
    ];

    // drives
    for d in metrics.drives.clone() {
        values.push((
            d.used_space_gb as f64,
            MetricTypeEnum::DriveUsedSpace,
            d.mountpoint,
        ));
    }

    // network_interfaces
    for n in metrics.network_interfaces.clone() {
        values.push((
            n.download_kb as f64,
            MetricTypeEnum::NetworkDownload,
            n.name.clone(),
        ));
        values.push((n.upload_kb as f64, MetricTypeEnum::NetworkUpload, n.name));
    }

    builder.push_values(values, |mut b, val| {
        b.push_bind(metrics.timestamp)
            .push_bind(val.0)
            .push_bind(val.1.to_string())
            .push_bind(metrics.collector_id)
            .push_bind(val.2);
    });

    builder.build().execute(pool).await?;

    let notifications = notifications::handle_metrics(pool, metrics.collector_id).await;
    if let Err(val) = notifications {
        eprintln!("Error with metrics notifications: {}", val);
    }

    Ok(())
}
