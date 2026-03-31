use sqlx::postgres::PgPoolOptions;

mod collector;
mod metrics;

pub use collector::*;
pub use metrics::*;

pub type Pool = sqlx::Pool<sqlx::Postgres>;

pub async fn get_pool() -> Result<Pool, shared::Error> {
    let url = shared::env::get("DATABASE_URL")?;

    Ok(PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?)
}
