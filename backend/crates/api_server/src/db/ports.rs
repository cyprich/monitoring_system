use shared::structs::{
    db::{PortsNotificationSettingsTable, PortsTable},
    ports::Port,
};

use crate::db::{Builder, Pool};

pub async fn get_ports(pool: &Pool, collector_id: i32) -> Result<Vec<PortsTable>, shared::Error> {
    let result = sqlx::query_as!(
        PortsTable,
        "select * from ports where collector_id = $1 order by port",
        collector_id
    )
    .fetch_all(pool)
    .await?;

    Ok(result)
}

pub async fn put_opened_ports(
    pool: &Pool,
    collector_id: i32,
    ports: Vec<Port>,
) -> Result<Vec<PortsTable>, shared::Error> {
    if ports.is_empty() {
        return Ok(vec![]);
    }

    let mut builder =
        Builder::new("insert into ports (collector_id, address, port, protocol, last_update) ");
    builder.push_values(ports, |mut b, val| {
        b.push_bind(collector_id)
            .push_bind(val.address.to_string())
            .push_bind(val.port as i32)
            .push_bind(val.protocol)
            .push_bind(val.last_update);
    });
    builder.push(" on conflict do nothing returning *");

    let result = builder
        .build_query_as::<PortsTable>()
        .fetch_all(pool)
        .await?;

    Ok(result)
}

pub async fn put_closed_ports(
    pool: &Pool,
    collector_id: i32,
    ports: Vec<Port>,
) -> Result<Vec<PortsTable>, shared::Error> {
    if ports.is_empty() {
        return Ok(vec![]);
    }

    let mut builder = Builder::new("delete from ports where collector_id = ");
    builder.push_bind(collector_id);
    builder.push(" and (address, port, protocol) in ( ");

    ports.into_iter().for_each(|val| {
        builder.push("( ");
        builder.push_bind(val.address);
        builder.push(" , ");
        builder.push_bind(val.port as i32);
        builder.push(" , ");
        builder.push_bind(val.protocol);
        builder.push(" ),  ");
    });

    builder.push(" ('',-1,'') ) returning *");

    let result = builder
        .build_query_as::<PortsTable>()
        .fetch_all(pool)
        .await?;

    Ok(result)
}

pub async fn get_collector_ports_notifications_settings(
    pool: &Pool,
    collector_id: i32,
) -> Result<PortsNotificationSettingsTable, shared::Error> {
    let result = sqlx::query_as!(
        PortsNotificationSettingsTable,
        "select * from ports_notifications_settings where collector_id = $1",
        collector_id
    )
    .fetch_one(pool)
    .await?;

    Ok(result)
}

pub async fn update_collector_ports_notifications_settings(
    pool: &Pool,
    collector_id: i32,
    is_opened_updating: bool,
    value: bool,
) -> Result<(), shared::Error> {
    let mut builder = Builder::new("update ports_notifications_settings set ");
    match is_opened_updating {
        true => {
            builder.push("show_for_opened ");
        }
        false => {
            builder.push("show_for_closed ");
        }
    };
    builder.push(" = ");
    builder.push_bind(value);

    builder.push(" where collector_id = ");
    builder.push_bind(collector_id);

    builder.build().execute(pool).await?;

    Ok(())
}
