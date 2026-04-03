use shared::structs::{
    db::{EndpointThresholdsTable, EndpointsThresholdsJoin, MetricsThresholdsTable},
    thresholds::EndpointsThreshold,
};

use crate::db::Pool;

pub async fn get_collector_metrics_thresholds(
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

pub async fn get_collector_endpoints_thresholds_join(
    pool: &Pool,
    collector_id: i32,
) -> Result<Vec<EndpointsThresholdsJoin>, shared::Error> {
    let result = sqlx::query_as!(
        EndpointsThresholdsJoin,
        "select 
            t.id threshold_id, 
            e.id endpoint_id,
            collector_id, 
            value threshold_value,
            url, 
            expected_codes
        from endpoints_thresholds t
        join endpoints e on t.endpoint_id =  e.id
        where collector_id = $1;",
        collector_id
    )
    .fetch_all(pool)
    .await?;

    Ok(result)
}
