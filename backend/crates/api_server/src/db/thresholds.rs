use shared::structs::{
    db::{
        DriveTable, EndpointsTable, EndpointsThresholdsJoin, MetricsThresholdsTable,
        NetworkInterfaceTable,
    },
    thresholds::{EndpointsThreshold, MetricsThreshold},
};

use crate::db::{Builder, Pool};

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
) -> Result<Vec<EndpointsThreshold>, shared::Error> {
    let result = sqlx::query_as!(
        EndpointsThreshold,
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

pub async fn get_endpoints_thresholds_join(
    pool: &Pool,
    collector_id: i32,
) -> Result<Vec<EndpointsThresholdsJoin>, shared::Error> {

    let result = sqlx::query_as!(EndpointsThresholdsJoin, 
        "
        select t.id threshold_id, endpoint_id, collector_id, t.count threshold_value, url, expected_codes
        from endpoints e
        join endpoints_thresholds t on e.id = t.endpoint_id
        where collector_id = $1", collector_id
    ).fetch_all(pool).await?;

    Ok(result)
}

pub async fn insert_metrics_thresholds(
    pool: &Pool,
    thresholds: MetricsThreshold,
) -> Result<MetricsThreshold, shared::Error> {
    let result = sqlx::query_as!(
        MetricsThreshold,
        "insert into metrics_thresholds
        (collector_id, metric_type, component_name, value, count)
        values ($1, $2, $3, $4, $5)
        returning *",
        thresholds.collector_id,
        thresholds.metric_type,
        thresholds.component_name,
        thresholds.value, 
        thresholds.count
    )
    .fetch_one(pool)
    .await?;

    Ok(result)
}

pub async fn insert_endpoints_thresholds(
    pool: &Pool,
    thresholds: EndpointsThreshold,
) -> Result<EndpointsThreshold, shared::Error> {
    let result = sqlx::query_as!(
        EndpointsThreshold,
        "insert into endpoints_thresholds
        (endpoint_id, count)
        values ($1, $2)
        returning *",
        thresholds.endpoint_id,
        thresholds.count
    )
    .fetch_one(pool)
    .await?;

    Ok(result)
}

pub async fn delete_metrics_thresholds(
    pool: &Pool,
    threshold_id: i32,
) -> Result<(), shared::Error> {
    sqlx::query!("delete from metrics_thresholds where id = $1", threshold_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn delete_endpoints_thresholds(
    pool: &Pool,
    threshold_id: i32,
) -> Result<(), shared::Error> {
    sqlx::query!(
        "delete from endpoints_thresholds where id = $1",
        threshold_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_available_metric_types(
    pool: &Pool,
    collector_id: i32,
) -> Result<Vec<String>, shared::Error> {
    let result = sqlx::query_scalar!(
        "select ty.name 
        from metric_type ty
        where ty.name not in (
            select th.metric_type
            from metrics_thresholds th
            where th.collector_id = $1
        )
        order by ty.name",
        collector_id
    )
    .fetch_all(pool)
    .await?;

    // special case, TODO
    let result = result
        .into_iter()
        .filter(|val| *val != "endpoint")
        .collect();

    Ok(result)
}

pub async fn get_available_drives(
    pool: &Pool,
    collector_id: i32,
) -> Result<Vec<DriveTable>, shared::Error> {
    let result = sqlx::query_as!(
        DriveTable,
        "
        select *
        from drives
        where collector_id = $1
        and mountpoint not in (
            select component_name
            from metrics_thresholds
            where collector_id = $1
            and metric_type = 'drive_used_space'
        ) order by mountpoint",
        collector_id
    )
    .fetch_all(pool)
    .await?;

    Ok(result)
}

pub enum NetworkMetricType {
    Upload,
    Download,
}

pub async fn get_available_network_interfaces(
    pool: &Pool,
    collector_id: i32,
    network_metric_type: NetworkMetricType,
) -> Result<Vec<NetworkInterfaceTable>, shared::Error> {
    let mut builder = Builder::new(
        "select * 
        from network_interfaces 
        where collector_id = ",
    );
    builder.push_bind(collector_id);
    builder.push(
        " and name not in (
        select component_name
        from metrics_thresholds 
        where collector_id = ",
    );
    builder.push_bind(collector_id);
    builder.push(" and metric_type = ");

    match network_metric_type {
        NetworkMetricType::Upload => {
            builder.push(" 'network_upload'");
        }
        NetworkMetricType::Download => {
            builder.push(" 'network_download'");
        }
    }

    builder.push(" ) order by name");

    let result = builder
        .build_query_as::<NetworkInterfaceTable>()
        .fetch_all(pool)
        .await?;

    Ok(result)
}

pub async fn get_available_endpoints(
    pool: &Pool,
    collector_id: i32,
) -> Result<Vec<EndpointsTable>, shared::Error> {
    let result = sqlx::query_as!(
        EndpointsTable,
        "select *
        from endpoints
        where id not in (
            select endpoint_id
            from endpoints_thresholds t
            join endpoints e on t.endpoint_id = e.id
            where collector_id = $1
        );",
        collector_id
    )
    .fetch_all(pool)
    .await?;

    Ok(result)
}
