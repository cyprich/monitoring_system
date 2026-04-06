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

use crate::DELETE_RECORDS_AFTER_HOURS;

pub type Pool = sqlx::Pool<sqlx::Postgres>;
pub type Builder<'a> = sqlx::QueryBuilder<'a, sqlx::Postgres>;

pub async fn get_pool() -> Result<Pool, shared::Error> {
    let url = shared::env::get("DATABASE_URL")?;

    Ok(PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?)
}

pub async fn delete_old_records(pool: &Pool) -> Result<(), shared::Error> {
    let mut transaction = pool.begin().await?;

    sqlx::query!(
        "delete from metrics where time < ( now() - $1 * '1 hour'::interval )",
        DELETE_RECORDS_AFTER_HOURS
    )
    .execute(&mut *transaction)
    .await?;

    sqlx::query!(
        "delete from endpoints_results where time < ( now() - $1 * '1 hour'::interval ) ",
        DELETE_RECORDS_AFTER_HOURS
    )
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    Ok(())
}
