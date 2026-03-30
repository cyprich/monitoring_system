use std::collections::BTreeMap;

use shared::structs::{
    UnidentifiedCollector,
    db::{
        metric_type::MetricType,
        tables::{CollectorTable, DriveTable, EndpointTable, MetricsTable, NetworkInterfaceTable},
    },
    endpoints::{Endpoint, EndpointInsert, EndpointResult},
    metrics::{DriveMetrics, Metrics, NetworkInterfaceMetrics},
};
use sqlx::{
    Postgres, QueryBuilder, postgres::PgPoolOptions, query_scalar, types::chrono::NaiveDateTime,
};

pub type Pool = sqlx::Pool<sqlx::Postgres>;

pub async fn get_pool() -> Result<Pool, shared::Error> {
    let url = shared::env::get("DATABASE_URL")?;

    Ok(PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?)
}

pub async fn insert_metrics(pool: &Pool, metrics: &Metrics) -> Result<(), shared::Error> {
    let mut builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "insert into metrics (timestamp, value, metric_type, collector_id, component_name) ",
    );

    let mut values = vec![
        (
            metrics.cpu_usage_global as f64,
            MetricType::CpuUsageGlobal,
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

    // cpu cores
    for (i, val) in metrics.cpu_usage_cores.iter().enumerate() {
        values.push(((*val) as f64, MetricType::CpuUsageCores, i.to_string()));
    }

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
            .push_bind(val.1)
            .push_bind(metrics.collector_id)
            .push_bind(val.2);
    });

    builder.build().execute(pool).await?;

    Ok(())
}

pub async fn register_collector(
    pool: &Pool,
    collector: &UnidentifiedCollector,
) -> Result<i32, shared::Error> {
    let mut transaction = pool.begin().await?;

    // collector
    let id = sqlx::query_scalar!(
        "insert into collectors 
        (name, system_name, host_name, kernel_version, total_memory_mb, total_swap_mb, cpu_count) 
        values ($1, $2, $3, $4, $5, $6, $7) 
        returning id",
        collector.name,
        collector.system_name,
        collector.host_name,
        collector.kernel_version,
        collector.total_memory_mb as i32,
        collector.total_swap_mb as i32,
        collector.cpu_count as i32
    )
    .fetch_one(&mut *transaction)
    .await?;

    // drives
    let mut builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "insert into drives (mountpoint, collector_id, capacity_gb, file_system) ",
    );

    builder.push_values(collector.drives.clone(), |mut b, drive| {
        b.push_bind(drive.mountpoint)
            .push_bind(id)
            .push_bind(drive.capacity_gb as i32)
            .push_bind(drive.file_system);
    });

    builder.build().execute(&mut *transaction).await?;

    // network interfaces
    let mut builder: QueryBuilder<Postgres> =
        QueryBuilder::new("insert into network_interfaces (name, collector_id, mac) ");

    builder.push_values(collector.network_interfaces.clone(), |mut b, net| {
        b.push_bind(net.name).push_bind(id).push_bind(net.mac);
    });

    builder.build().execute(&mut *transaction).await?;

    transaction.commit().await?;

    Ok(id)
}

pub async fn get_collectors(pool: &Pool) -> Result<Vec<CollectorTable>, shared::Error> {
    Ok(
        sqlx::query_as!(CollectorTable, "select * from collectors order by id")
            .fetch_all(pool)
            .await?,
    )
}

pub async fn get_collector_by_id(pool: &Pool, id: i32) -> Result<CollectorTable, shared::Error> {
    Ok(sqlx::query_as!(
        CollectorTable,
        "select * from collectors where id = $1 order by id",
        id
    )
    .fetch_one(pool)
    .await?)
}

pub async fn get_collector_metrics(
    pool: &Pool,
    id: i32,
    limit: Option<i32>,
) -> Result<Vec<Metrics>, shared::Error> {
    let mut builder: QueryBuilder<Postgres> =
        QueryBuilder::new("select * from metrics where collector_id = ");
    builder.push_bind(id);

    if let Some(val) = limit {
        builder.push(
            " and timestamp in (
            select distinct timestamp
            from metrics
            where collector_id = ",
        );
        builder.push_bind(id);
        builder.push(" order by timestamp desc limit ");
        builder.push_bind(val);
        builder.push(" )");
    };

    let result = builder
        .build_query_as::<MetricsTable>()
        .fetch_all(pool)
        .await?;

    let cpu_count = query_scalar!("select cpu_count from collectors where id = $1", id)
        .fetch_one(pool)
        .await?;

    let mut map: BTreeMap<NaiveDateTime, Metrics> = BTreeMap::new();

    for row in result {
        let entry = map.entry(row.timestamp).or_insert(Metrics {
            // TODO it's useless to carry collector id, too much overhead
            collector_id: id,
            timestamp: row.timestamp,
            used_memory_mb: 0,
            used_swap_mb: 0,
            cpu_usage_global: 0.0,
            cpu_usage_cores: vec![0.0; cpu_count as usize],
            drives: vec![],
            network_interfaces: vec![],
        });

        match row.metric_type {
            MetricType::CpuUsageGlobal => entry.cpu_usage_global = row.value as f32,
            MetricType::UsedMemoryMb => entry.used_memory_mb = row.value as u64,
            MetricType::UsedSwapMb => entry.used_swap_mb = row.value as u64,
            MetricType::CpuUsageCores => {
                let index = row.component_name.parse::<usize>();
                if let Ok(i) = index {
                    entry.cpu_usage_cores.insert(i, row.value as f32);
                }
            }
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

pub async fn rename_collector(pool: &Pool, id: i32, name: String) -> Result<(), shared::Error> {
    // TODO should i return the name?
    let result = sqlx::query_scalar!("update collectors set name = $1 where id = $2", name, id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        Err(shared::Error::DbNothingChanged)
    } else {
        Ok(())
    }
}

pub async fn get_collector_drives(pool: &Pool, id: i32) -> Result<Vec<DriveTable>, shared::Error> {
    Ok(sqlx::query_as!(
        DriveTable,
        "select * from drives where collector_id = $1",
        id
    )
    .fetch_all(pool)
    .await?)
}

pub async fn get_collector_network_interfaces(
    pool: &Pool,
    id: i32,
) -> Result<Vec<NetworkInterfaceTable>, shared::Error> {
    Ok(sqlx::query_as!(
        NetworkInterfaceTable,
        "select * from network_interfaces where collector_id = $1",
        id
    )
    .fetch_all(pool)
    .await?)
}

pub async fn get_collector_endpoints(pool: &Pool, id: i32) -> Result<Vec<Endpoint>, shared::Error> {
    let result = sqlx::query_as!(
        EndpointTable,
        "select * from endpoints where collector_id = $1 order by id",
        id
    )
    .fetch_all(pool)
    .await?;

    let result = result.into_iter().map(Endpoint::from);

    Ok(result.collect())
}

pub async fn insert_collector_endpoints(
    pool: &Pool,
    collector_id: i32,
    endpoint: &EndpointInsert,
) -> Result<(), shared::Error> {
    let codes = endpoint
        .expected_codes
        .iter()
        .map(|c| *c as i32)
        .collect::<Vec<i32>>();

    sqlx::query!(
        "insert into endpoints ( collector_id, url, expected_codes ) values ( $1, $2, $3 )",
        collector_id,
        endpoint.url,
        &codes
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_collector_endpoints(
    pool: &Pool,
    endpoint: &Endpoint,
) -> Result<(), shared::Error> {
    // TODO remove duplicity
    let codes = endpoint
        .expected_codes
        .iter()
        .map(|c| *c as i32)
        .collect::<Vec<i32>>();

    sqlx::query!(
        "update endpoints set ( url, expected_codes ) = ( $1, $2 ) where id = $3",
        endpoint.url,
        &codes,
        endpoint.id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_collector_endpoint(pool: &Pool, id: i32) -> Result<(), shared::Error> {
    let mut transaction = pool.begin().await?;

    sqlx::query!("delete from endpoints_results where endpoint_id = $1", id)
        .execute(&mut *transaction)
        .await?;

    sqlx::query!("delete from endpoints where id = $1", id)
        .execute(&mut *transaction)
        .await?;

    transaction.commit().await?;

    Ok(())
}

pub async fn get_collector_endpoints_results(
    pool: &Pool,
    id: i32,
) -> Result<Vec<EndpointResult>, shared::Error> {
    let result = sqlx::query_as!(
        EndpointResult,
        "select * 
        from endpoints_results 
        where endpoint_id in (
        select id from endpoints where collector_id = $1)",
        id
    )
    .fetch_all(pool)
    .await?;

    Ok(result)
}

pub async fn get_collector_endpoints_results_last(
    pool: &Pool,
    id: i32,
) -> Result<Vec<EndpointResult>, shared::Error> {
    let result = sqlx::query_as!(
        EndpointResult,
        "select distinct on (endpoint_id) endpoint_id, timestamp, result, latency_microseconds
        from endpoints_results
        where endpoint_id in (
        select id from endpoints where collector_id = $1)
        order by endpoint_id, timestamp desc",
        id
    )
    .fetch_all(pool)
    .await?;

    Ok(result)
}

pub async fn insert_collector_endpoints_results(
    pool: &Pool,
    endpoint_results: Vec<EndpointResult>,
) -> Result<(), shared::Error> {
    let mut builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "insert into endpoints_results (endpoint_id, timestamp, result, latency_microseconds) ",
    );

    builder.push_values(endpoint_results, |mut b, val| {
        b.push_bind(val.endpoint_id)
            .push_bind(val.timestamp)
            .push_bind(val.result)
            .push_bind(val.latency_microseconds);
    });

    builder.build().execute(pool).await?;

    Ok(())
}
