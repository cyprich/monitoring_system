use shared::structs::{metrics::Metrics, unidentified_collector::UnidentifiedCollector};
use sqlx::postgres::PgPoolOptions;

use crate::db::models::CollectorDB;

mod models;

pub type Pool = sqlx::Pool<sqlx::Postgres>;

pub async fn get_pool() -> Option<Pool> {
    let url = shared::get_env("DATABASE_URL")?;

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .ok()
}

pub async fn insert_metrics(pool: &Pool, metrics: &Metrics) {
    let _ = sqlx::query!(
        "insert into metrics (timestamp, host_id, type_id, value) values ( $1, $2, $3, $4 )",
        metrics.timestamp,
        metrics.collector_id,
        1,
        metrics.cpu_usage as f64
    )
    .execute(pool)
    .await;
}

pub async fn register_collector(pool: &Pool, collector: &UnidentifiedCollector) -> Option<i32> {
    let id = sqlx::query_scalar!(
        "insert into hosts (name, system_name, host_name, kernel_version, total_memory, cpu_count) values ($1, $2, $3, $4, $5, $6) returning id",
        collector.name,
        collector.system_name,
        collector.host_name,
        collector.kernel_version,
        collector.total_memory as i32, 
        collector.cpu_count as i32
    ).fetch_one(pool).await;

    if id.is_err() {
        dbg!(&id);
    }

    id.ok()
}

pub async fn get_collectors(pool: &Pool) -> Vec<CollectorDB> {
    let result = sqlx::query_as!(
        CollectorDB, 
        "select * from hosts")
    .fetch_all(pool)
    .await;

    // TODO error handling 
    result.unwrap_or_default()
}

pub async fn get_collector_by_id(pool: &Pool, id: i32) -> Option<CollectorDB> {
    let result = sqlx::query_as!(
        CollectorDB, 
        "select * from hosts where id = $1", 
        id)
    .fetch_one(pool)
    .await;

    dbg!(&result);

    // TODO error handling 
    result.ok()
}
