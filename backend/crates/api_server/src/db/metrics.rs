use shared::{
    enums::metric_type::MetricType,
    structs::{db::MetricsTable, metrics::Metrics},
};

use crate::{
    AppState,
    db::{Builder, Pool},
    notifications,
};

pub async fn insert_metrics(state: &AppState, metrics: &Metrics) -> Result<(), shared::Error> {
    let mut builder = Builder::new(
        "insert into metrics (time, value, metric_type, collector_id, component_name) ",
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
        b.push_bind(metrics.time)
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
    time_limit_hours: Option<i32>,
    limit: Option<i32>,
) -> Result<Vec<MetricsTable>, shared::Error> {
    let mut builder = Builder::new("select * from metrics where collector_id = ");
    builder.push_bind(collector_id);

    if let Some(val) = time_limit_hours {
        builder.push(" and time > (now() - ");
        builder.push_bind(val);
        builder.push(" * '1 hour'::interval) ");
        // val * '1 hour'
        // else it will become `... '$2 hour'::intverval`, and because it's a string literal,
        // it will be converted to `'2 hour'::intverval`
        // it was giving the last 2 hours every time
    };

    if let Some(val) = limit {
        builder.push(
            " and time in (
            select distinct time
            from metrics 
            where collector_id = ",
        );

        builder.push_bind(collector_id);
        builder.push(" order by time desc limit ");
        builder.push_bind(val);
        builder.push(" )");
    }

    // TODO should i order it?
    builder.push(" order by time ");

    let result = builder
        .build_query_as::<MetricsTable>()
        .fetch_all(pool)
        .await?;

    Ok(result)
}

pub async fn get_metrics_by_type_and_component(
    pool: &Pool,
    collector_id: i32,
    metric_type: &str,
    component_name: &str,
    limit: i32,
) -> Result<Vec<MetricsTable>, shared::Error> {
    let result = sqlx::query_as!(
        MetricsTable,
        "select * 
        from metrics 
        where collector_id = $1 
        and metric_type = $2
        and component_name = $3
        order by time desc
        limit $4",
        collector_id,
        metric_type,
        component_name,
        limit as i64
    )
    .fetch_all(pool)
    .await?;

    Ok(result)
}
