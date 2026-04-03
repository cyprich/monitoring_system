use sqlx::postgres::PgPoolOptions;

mod collector;
mod endpoints;
mod endpoints_results;
mod metrics;
mod notifications;
mod thresholds;

pub use collector::*;
pub use endpoints::*;
pub use endpoints_results::*;
pub use metrics::*;
pub use notifications::*;
pub use thresholds::*;

pub type Pool = sqlx::Pool<sqlx::Postgres>;
pub type Builder<'a> = sqlx::QueryBuilder<'a, sqlx::Postgres>;

pub async fn get_pool() -> Result<Pool, shared::Error> {
    let url = shared::env::get("DATABASE_URL")?;

    Ok(PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?)
}
