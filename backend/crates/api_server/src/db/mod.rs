use shared::structs::Metrics;
use sqlx::postgres::PgPoolOptions;

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
        1,
        1,
        metrics.cpu_usage as f64
    )
    .execute(pool)
    .await;
}
