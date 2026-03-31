use shared::structs::{
    UnidentifiedCollector,
    db::{CollectorTable, DriveTable, NetworkInterfaceTable},
};
use sqlx::{Postgres, QueryBuilder};

use crate::Pool;

mod endpoints;
mod endpoints_results;
mod metrics;

pub use endpoints::*;
pub use endpoints_results::*;
pub use metrics::*;

pub async fn register_collector(
    pool: &Pool,
    collector: &UnidentifiedCollector,
) -> Result<i32, shared::Error> {
    let mut transaction = pool.begin().await?;

    // collector
    let id = sqlx::query_scalar!(
        "insert into collectors 
        (name, system_name, host_name, kernel_version, total_memory_mb, total_swap_mb, cpu_count) 
        values ($1, $2, $3, $4, $5, $6, $7) 
        returning id",
        collector.name,
        collector.system_name,
        collector.host_name,
        collector.kernel_version,
        collector.total_memory_mb as i32,
        collector.total_swap_mb as i32,
        collector.cpu_count as i32
    )
    .fetch_one(&mut *transaction)
    .await?;

    // drives
    let mut builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "insert into drives (mountpoint, collector_id, capacity_gb, file_system) ",
    );

    builder.push_values(collector.drives.clone(), |mut b, drive| {
        b.push_bind(drive.mountpoint)
            .push_bind(id)
            .push_bind(drive.capacity_gb as i32)
            .push_bind(drive.file_system);
    });

    builder.build().execute(&mut *transaction).await?;

    // network interfaces
    let mut builder: QueryBuilder<Postgres> =
        QueryBuilder::new("insert into network_interfaces (name, collector_id, mac) ");

    builder.push_values(collector.network_interfaces.clone(), |mut b, net| {
        b.push_bind(net.name).push_bind(id).push_bind(net.mac);
    });

    builder.build().execute(&mut *transaction).await?;

    transaction.commit().await?;

    Ok(id)
}

pub async fn get_collectors(pool: &Pool) -> Result<Vec<CollectorTable>, shared::Error> {
    Ok(
        sqlx::query_as!(CollectorTable, "select * from collectors order by id")
            .fetch_all(pool)
            .await?,
    )
}

pub async fn get_collector_by_id(pool: &Pool, id: i32) -> Result<CollectorTable, shared::Error> {
    Ok(sqlx::query_as!(
        CollectorTable,
        "select * from collectors where id = $1 order by id",
        id
    )
    .fetch_one(pool)
    .await?)
}

pub async fn rename_collector(pool: &Pool, id: i32, name: String) -> Result<(), shared::Error> {
    // TODO should i return the name?
    let result = sqlx::query_scalar!("update collectors set name = $1 where id = $2", name, id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        Err(shared::Error::DbNothingChanged)
    } else {
        Ok(())
    }
}

pub async fn get_collector_drives(pool: &Pool, id: i32) -> Result<Vec<DriveTable>, shared::Error> {
    Ok(sqlx::query_as!(
        DriveTable,
        "select * from drives where collector_id = $1",
        id
    )
    .fetch_all(pool)
    .await?)
}

pub async fn get_collector_network_interfaces(
    pool: &Pool,
    id: i32,
) -> Result<Vec<NetworkInterfaceTable>, shared::Error> {
    Ok(sqlx::query_as!(
        NetworkInterfaceTable,
        "select * from network_interfaces where collector_id = $1",
        id
    )
    .fetch_all(pool)
    .await?)
}
