use std::collections::HashMap;

use shared::structs::{
    db::{EndpointInsert, EndpointsTable},
    endpoints::Endpoint,
};

use crate::{Pool, db::Builder};

pub async fn get_endpoints(pool: &Pool, collector_id: i32) -> Result<Vec<Endpoint>, shared::Error> {
    let result = sqlx::query_as!(
        EndpointsTable,
        "select * from endpoints where collector_id = $1 order by id",
        collector_id
    )
    .fetch_all(pool)
    .await?;

    let result = result.into_iter().map(Endpoint::from);

    Ok(result.collect())
}

pub async fn insert_endpoint(
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

pub async fn update_endpoint(pool: &Pool, endpoint: &Endpoint) -> Result<(), shared::Error> {
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

pub async fn delete_endpoint(pool: &Pool, endpoint_id: i32) -> Result<(), shared::Error> {
    let mut transaction = pool.begin().await?;

    sqlx::query!(
        "delete from endpoints_results where endpoint_id = $1",
        endpoint_id
    )
    .execute(&mut *transaction)
    .await?;

    sqlx::query!("delete from endpoints where id = $1", endpoint_id)
        .execute(&mut *transaction)
        .await?;

    transaction.commit().await?;

    Ok(())
}

pub async fn get_endpoints_by_id(
    pool: &Pool,
    collector_id: Option<i32>,
) -> Result<HashMap<i32, Endpoint>, shared::Error> {
    let mut builder = Builder::new("select * from endpoints ");

    if let Some(val) = collector_id {
        builder.push(" where collector_id = ");
        builder.push_bind(val);
    }

    let endpoints = builder
        .build_query_as::<EndpointsTable>()
        .fetch_all(pool)
        .await?;

    let mut map = HashMap::new();

    for e in endpoints {
        map.entry(e.id).insert_entry(Endpoint::from(e));
    }

    Ok(map)
}
