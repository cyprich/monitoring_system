use shared::structs::{db::MetricsThresholdsTable, thresholds::EndpointsThresholds};

use crate::db::Pool;

pub async fn get_metrics_thresholds(
    pool: &Pool,
    collector_id: i32,
) -> Result<Vec<MetricsThresholdsTable>, shared::Error> {
    let result = sqlx::query_as!(
        MetricsThresholdsTable,
        "select * from metrics_thresholds where collector_id = $1",
        collector_id
    )
    .fetch_all(pool)
    .await?;

    Ok(result)
}

pub async fn get_endpoints_thresholds(
    pool: &Pool,
    collector_id: i32,
) -> Result<Vec<EndpointsThresholds>, shared::Error> {
    let result = sqlx::query_as!(
        EndpointsThresholds,
        "select t.*
        from endpoints_thresholds t
        join endpoints e on t.endpoint_id = e.id
        where collector_id = $1;",
        collector_id
    )
    .fetch_all(pool)
    .await?;

    Ok(result)
}
