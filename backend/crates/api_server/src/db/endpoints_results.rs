use crate::db::Builder;
use shared::structs::endpoints::EndpointResult;

use crate::Pool;

pub async fn get_endpoints_results(
    pool: &Pool,
    collector_id: i32,
    limit: Option<i32>,
) -> Result<Vec<EndpointResult>, shared::Error> {
    let mut builder = Builder::new(
        "select r.*
        from endpoints_results r
        join endpoints e on e.id = r.endpoint_id
        where collector_id = ",
    );
    builder.push_bind(collector_id);

    if let Some(val) = limit {
        builder.push(
            " and time in (
            select distinct time
            from endpoints_results r
            join endpoints e on e.id = r.endpoint_id
            where collector_id = ",
        );
        builder.push_bind(collector_id);
        builder.push(" order by time desc limit ");
        builder.push_bind(val);
        builder.push(" )");
    }

    let result = builder
        .build_query_as::<EndpointResult>()
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
        "select distinct on (endpoint_id) endpoint_id, time, result, latency_microseconds
        from endpoints_results
        where endpoint_id in (
        select id from endpoints where collector_id = $1)
        order by endpoint_id, time desc",
        id
    )
    .fetch_all(pool)
    .await?;

    Ok(result)
}

pub async fn get_endpoints_results_by_endpoint_id(
    pool: &Pool,
    endpoint_id: i32,
    limit: i32,
) -> Result<Vec<EndpointResult>, shared::Error> {
    let result = sqlx::query_as!(
        EndpointResult,
        "select * 
        from endpoints_results 
        where endpoint_id = $1 
        limit $2",
        endpoint_id,
        limit as i64
    )
    .fetch_all(pool)
    .await?;

    Ok(result)
}

pub async fn insert_endpoints_results(
    pool: &Pool,
    endpoint_results: Vec<EndpointResult>,
) -> Result<(), shared::Error> {
    let mut builder = Builder::new(
        "insert into endpoints_results (endpoint_id, time, result, latency_microseconds) ",
    );

    builder.push_values(endpoint_results, |mut b, val| {
        b.push_bind(val.endpoint_id)
            .push_bind(val.time)
            .push_bind(val.result)
            .push_bind(val.latency_microseconds);
    });

    builder.build().execute(pool).await?;

    Ok(())
}
