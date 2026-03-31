use shared::structs::{
    db::{EndpointInsert, EndpointsTable},
    endpoints::Endpoint,
};

use crate::Pool;

pub async fn get_collector_endpoints(pool: &Pool, id: i32) -> Result<Vec<Endpoint>, shared::Error> {
    let result = sqlx::query_as!(
        EndpointsTable,
        "select * from endpoints where collector_id = $1 order by id",
        id
    )
    .fetch_all(pool)
    .await?;

    let result = result.into_iter().map(Endpoint::from);

    Ok(result.collect())
}

pub async fn insert_collector_endpoints(
    pool: &Pool,
    collector_id: i32,
    endpoint: &EndpointInsert,
) -> Result<(), shared::Error> {
    let codes = endpoint
        .expected_codes
        .iter()
        .map(|c| *c as i32)
        .collect::<Vec<i32>>();

    sqlx::query!(
        "insert into endpoints ( collector_id, url, expected_codes ) values ( $1, $2, $3 )",
        collector_id,
        endpoint.url,
        &codes
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_collector_endpoints(
    pool: &Pool,
    endpoint: &Endpoint,
) -> Result<(), shared::Error> {
    // TODO remove duplicity
    let codes = endpoint
        .expected_codes
        .iter()
        .map(|c| *c as i32)
        .collect::<Vec<i32>>();

    sqlx::query!(
        "update endpoints set ( url, expected_codes ) = ( $1, $2 ) where id = $3",
        endpoint.url,
        &codes,
        endpoint.id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_collector_endpoint(pool: &Pool, id: i32) -> Result<(), shared::Error> {
    let mut transaction = pool.begin().await?;

    sqlx::query!("delete from endpoints_results where endpoint_id = $1", id)
        .execute(&mut *transaction)
        .await?;

    sqlx::query!("delete from endpoints where id = $1", id)
        .execute(&mut *transaction)
        .await?;

    transaction.commit().await?;

    Ok(())
}
