use shared::structs::{
    collector_info::CollectorInfo,
    db::{CollectorTable, DriveTable, NetworkInterfaceTable},
};

use crate::{Pool, db::Builder};

pub async fn register_collector(
    pool: &Pool,
    collector_info: &CollectorInfo,
) -> Result<i32, shared::Error> {
    let mut transaction = pool.begin().await?;
    let c = collector_info;

    // collector
    let id = sqlx::query_scalar!(
        "insert into collectors 
        (name, system_name, host_name, kernel_version, total_memory_mb, total_swap_mb, cpu_count) 
        values ($1, $2, $3, $4, $5, $6, $7) 
        returning id",
        c.name,
        c.system_name,
        c.host_name,
        c.kernel_version,
        c.total_memory_mb as i32,
        c.total_swap_mb as i32,
        c.cpu_count as i32
    )
    .fetch_one(&mut *transaction)
    .await?;

    // drives
    let mut builder =
        Builder::new("insert into drives (mountpoint, collector_id, capacity_gb, file_system) ");

    builder.push_values(c.drives.clone(), |mut b, drive| {
        b.push_bind(drive.mountpoint)
            .push_bind(id)
            .push_bind(drive.capacity_gb as i32)
            .push_bind(drive.file_system);
    });

    builder.build().execute(&mut *transaction).await?;

    // network interfaces
    let mut builder = Builder::new("insert into network_interfaces (name, collector_id, mac) ");

    builder.push_values(c.network_interfaces.clone(), |mut b, net| {
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

pub async fn get_collector_by_id(
    pool: &Pool,
    collector_id: i32,
) -> Result<CollectorTable, shared::Error> {
    Ok(sqlx::query_as!(
        CollectorTable,
        "select * from collectors where id = $1 order by id",
        collector_id
    )
    .fetch_one(pool)
    .await?)
}

pub async fn rename_collector(
    pool: &Pool,
    collector_id: i32,
    name: String,
) -> Result<(), shared::Error> {
    let result = sqlx::query_scalar!(
        "update collectors set name = $1 where id = $2",
        name,
        collector_id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        Err(shared::Error::DbNothingChanged)
    } else {
        Ok(())
    }
}

pub async fn get_collector_drives(
    pool: &Pool,
    collector_id: i32,
) -> Result<Vec<DriveTable>, shared::Error> {
    Ok(sqlx::query_as!(
        DriveTable,
        "select * from drives where collector_id = $1",
        collector_id
    )
    .fetch_all(pool)
    .await?)
}

pub async fn get_collector_network_interfaces(
    pool: &Pool,
    collector_id: i32,
) -> Result<Vec<NetworkInterfaceTable>, shared::Error> {
    Ok(sqlx::query_as!(
        NetworkInterfaceTable,
        "select * from network_interfaces where collector_id = $1",
        collector_id
    )
    .fetch_all(pool)
    .await?)
}

// pub async fn get_collector_name(pool: &Pool, collector_id: i32) -> Result<String, shared::Error> {
//     let result = sqlx::query_scalar!("select name from collectors where id = $1", collector_id)
//         .fetch_one(pool)
//         .await?;
//
//     Ok(result)
// }
