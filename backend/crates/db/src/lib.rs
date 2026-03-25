use std::collections::BTreeMap;

use shared::structs::{
    db::{collector::CollectorDB, metric_type::MetricType, metrics::MetricsDB},
    metrics::Metrics,
    unidentified_collector::UnidentifiedCollector,
};
use sqlx::{postgres::PgPoolOptions, types::chrono::NaiveDateTime};

pub type Pool = sqlx::Pool<sqlx::Postgres>;

pub async fn get_pool() -> Result<Pool, shared::Error> {
    let url = shared::env::get("DATABASE_URL")?;

    Ok(PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?)
}

pub async fn insert_metrics(pool: &Pool, metrics: &Metrics) -> Result<(), shared::Error> {
    let mut transaction = pool.begin().await?;

    // TODO this surely can be done prettier - insert multiple rows
    sqlx::query!(
        "insert into metrics (timestamp, value, metric_type, collector_id, component_name) values ( $1, $2, ($3::text)::metric_type, $4, $5 )",
        metrics.timestamp,
        metrics.cpu_usage as f64,
        MetricType::CpuUsage.to_string(),
        metrics.collector_id,
        ""
    )
    .execute(&mut *transaction)
    .await?;

    sqlx::query!(
        "insert into metrics 
        (timestamp, value, metric_type, collector_id, component_name) 
        values ( $1, $2, ($3::text)::metric_type, $4, $5 )",
        metrics.timestamp,
        metrics.used_memory_mb as f64,
        MetricType::UsedMemoryMb.to_string(),
        metrics.collector_id,
        ""
    )
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    Ok(())
}

pub async fn register_collector(
    pool: &Pool,
    collector: &UnidentifiedCollector,
) -> Result<i32, shared::Error> {
    Ok(sqlx::query_scalar!(
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
    .fetch_one(pool)
    .await?)
}

pub async fn get_collectors(pool: &Pool) -> Result<Vec<CollectorDB>, shared::Error> {
    Ok(
        sqlx::query_as!(CollectorDB, "select * from collectors order by id")
            .fetch_all(pool)
            .await?,
    )
}

pub async fn get_collector_by_id(pool: &Pool, id: i32) -> Result<CollectorDB, shared::Error> {
    Ok(
        sqlx::query_as!(CollectorDB, "select * from collectors where id = $1", id)
            .fetch_one(pool)
            .await?,
    )
}

pub async fn get_collector_metrics(
    pool: &Pool,
    id: i32,
    limit: Option<i32>,
) -> Result<Vec<Metrics>, shared::Error> {
    let sql = "select * from metrics where collector_id = $1";

    let result = match limit {
        Some(val) => {
            let sql = format!(
                "{sql} and timestamp in (
                    select distinct timestamp
                    from metrics
                    where collector_id = $1
                    order by timestamp desc
                    limit $2)"
            );

            sqlx::query_as::<_, MetricsDB>(&sql)
                .bind(id)
                .bind(val)
                .fetch_all(pool)
                .await?
        }

        None => {
            sqlx::query_as::<_, MetricsDB>(sql)
                .bind(id)
                .fetch_all(pool)
                .await?
        }
    };

    let mut map: BTreeMap<NaiveDateTime, Metrics> = BTreeMap::new();

    for row in result {
        let entry = map.entry(row.timestamp).or_insert(Metrics {
            // TODO it's useless to carry collector id, too much overhead
            collector_id: id,
            timestamp: row.timestamp,
            used_memory_mb: 0,
            used_swap_mb: 0,
            cpu_usage: 0.0,
            disks: vec![],
            networks: vec![],
        });

        match row.metric_type {
            MetricType::CpuUsage => entry.cpu_usage = row.value as f32,
            MetricType::UsedMemoryMb => entry.used_memory_mb = row.value as u64,
            MetricType::UsedSwapMb => entry.used_swap_mb = row.value as u64,
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
