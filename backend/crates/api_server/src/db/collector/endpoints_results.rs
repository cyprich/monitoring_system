use shared::structs::endpoints::EndpointResult;
use sqlx::{Postgres, QueryBuilder};

use crate::Pool;

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
