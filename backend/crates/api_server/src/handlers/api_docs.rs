use shared::structs::{
    db::{CollectorTable, DriveTable, EndpointsTable, EndpointsThresholdsJoin, NetworkInterfaceTable}, 
    endpoints::{Endpoint, EndpointResult}, 
    notifications::Notification, 
    thresholds::{EndpointsThreshold, MetricsThreshold}
};

#[derive(utoipa::OpenApi)]
#[openapi(paths(
    hello,
    metrics_post,
    collector_register,
    collectors,
    get_collector,
    get_collector_metrics,
    get_collector_drives,
    get_collector_network_interfaces,
    get_collector_endpoints,
    post_collector_endpoints,
    put_collector_endpoints,
    delete_collector_endpoints,
    get_collector_endpoint_results,
    get_collector_endpoint_results_last,
    post_collector_endpoint_results,
    get_collector_notifications,
    delete_collector_notifications,
    delete_collector_notifications_all,
    rename_collector,
    get_collector_metrics_thresholds,
    get_collector_metrics_thresholds_available_metric_types,
    get_collector_metrics_thresholds_available_drives,
    get_collector_metrics_thresholds_available_networks_upload,
    get_collector_metrics_thresholds_available_networks_download,
    get_collector_endpoints_thresholds,
    get_collector_endpoints_thresholds_join,
    get_collector_endpoints_available_endpoints,
    post_metrics_thresholds,
    post_endpoints_thresholds,
    delete_metrics_thresholds,
    delete_endpoints_thresholds
))]
pub struct ApiDoc;

// handlers
/////////////////////////////////////////////////////////

#[utoipa::path(
    get, 
    path = "/api/v1",
    responses(
        (status = 200, description="API Server reachable", body=String),
    ), 
)]
#[allow(dead_code)]
fn hello() {}

#[utoipa::path(
    get, 
    path = "/api/v1/collectors",
    responses(
        (status = 200, description="Collectors from database", body=Vec<CollectorTable>),
        (status = 409, description="Problems with Foreign Key in Database", body=String),
        (status = 500, description="Internal Server Error", body=String)
    ), 
)]
#[allow(dead_code)]
fn collectors() {}

#[utoipa::path(
    get, 
    path = "/api/v1/collector/{id}",
    responses(
        (status = 200, description="Collector with ID from database", body=CollectorTable),
        (status = 409, description="Problems with Foreign Key in Database", body=String),
        (status = 500, description="Internal Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector")
    )
)]
#[allow(dead_code)]
fn get_collector() {}

#[utoipa::path(
    get, 
    path = "/api/v1/collector/{id}/metrics",
    responses(
        (status = 200, description="Collector with ID from database", body=CollectorTable),
        (status = 409, description="Problems with Foreign Key in Database", body=String),
        (status = 500, description="Internal Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
        ("time_limit_hours", Query, description="Maximum number of hours to include in the result"),
        ("resolution", Query, description="How many datapoints to return. Data will be grouped and averaged together"),
    )
)]
#[allow(dead_code)]
fn get_collector_metrics() {}

#[utoipa::path(
    get, 
    path = "/api/v1/collector/{id}/drives",
    responses(
        (status = 200, description="Drives of Collector with ID from database", body=Vec<DriveTable>),
        (status = 409, description="Problems with Foreign Key in Database", body=String),
        (status = 500, description="Internal Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[allow(dead_code)]
fn get_collector_drives() {}

#[utoipa::path(
    get, 
    path = "/api/v1/collector/{id}/network_interfaces",
    responses(
        (status = 200, description="Network Interfaces of Collector with ID from database", body=Vec<NetworkInterfaceTable>),
        (status = 409, description="Problems with Foreign Key in Database", body=String),
        (status = 500, description="Internal Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[allow(dead_code)]
fn get_collector_network_interfaces() {}

#[utoipa::path(
    get, 
    path = "/api/v1/collector/{id}/endpoints",
    responses(
        (status = 200, description="Endpoints of Collector with ID from database", body=Vec<Endpoint>),
        (status = 409, description="Problems with Foreign Key in Database", body=String),
        (status = 500, description="Internal Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[allow(dead_code)]
fn get_collector_endpoints() {}

#[utoipa::path(
    post, 
    path = "/api/v1/collector/{id}/endpoints",
    responses(
        (status = 200, description="New endpoint created", body=Endpoint),
        (status = 409, description="Problems with Foreign Key in Database", body=String),
        (status = 500, description="Internal Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[allow(dead_code)]
fn post_collector_endpoints() {}

#[utoipa::path(
    put, 
    path = "/api/v1/collector/{id}/endpoints",
    responses(
        (status = 200, description="Endpoint updated"),
        (status = 409, description="Problems with Foreign Key in Database", body=String),
        (status = 500, description="Internal Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[allow(dead_code)]
fn put_collector_endpoints() {}

#[utoipa::path(
    delete, 
    path = "/api/v1/collector/{collector_id}/endpoints/{endpoint_id}",
    responses(
        (status = 200, description="Endpoint Deleted"),
        (status = 409, description="Problems with Foreign Key in Database", body=String),
        (status = 500, description="Internal Server Error", body=String)
    ), 
    params (
        ("collector_id", Path, description="ID of Collector"),
        ("endpoint_id", Path, description="ID of Endpoint"),
    )
)]
#[allow(dead_code)]
fn delete_collector_endpoints() {}

#[utoipa::path(
    get, 
    path = "/api/v1/collector/{id}/endpoints_results",
    responses(
        (status = 200, description="Results of Endpoints measurements", body=Vec<EndpointResult>),
        (status = 409, description="Problems with Foreign Key in Database", body=String),
        (status = 500, description="Internal Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[allow(dead_code)]
fn get_collector_endpoint_results() {}

#[utoipa::path(
    get, 
    path = "/api/v1/collector/{id}/endpoints_results/last",
    responses(
        (status = 200, description="Results of Endpoints measurements from only the last measurements", body=Vec<EndpointResult>),
        (status = 409, description="Problems with Foreign Key in Database", body=String),
        (status = 500, description="Internal Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[allow(dead_code)]
fn get_collector_endpoint_results_last() {}

#[utoipa::path(
    post, 
    path = "/api/v1/collector/{id}/endpoints_results",
    responses(
        (status = 200, description="Sucessfully inserted into database"),
        (status = 409, description="Problems with Foreign Key in Database", body=String),
        (status = 500, description="Internal Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[allow(dead_code)]
fn post_collector_endpoint_results() {}

#[utoipa::path(
    post, 
    path = "/api/v1/collector/register",
    responses(
        (status = 201, description="Collector registered, returning ID of Collector", body=i32),
        (status = 409, description="Problems with Foreign Key in Database", body=String),
        (status = 500, description="Internal Server Error", body=String)
    ), 
)]
#[allow(dead_code)]
fn collector_register() {}

#[utoipa::path(
    patch, 
    path = "/api/v1/collector/{id}/rename",
    responses(
        (status = 200, description="Collector renamed"),
        (status = 304, description="Collector name not changed"),
        (status = 409, description="Problems with Foreign Key in Database", body=String),
        (status = 500, description="Internal Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[allow(dead_code)]
fn rename_collector() {}

/////////////////////////////////////////////////////////

#[utoipa::path(
    post, 
    path = "/api/v1/metrics",
    responses(
        (status = 200, description="Sucessfully inserted into database"),
        (status = 401, description="Collector ID is invalid or not found, new registration needed"),
        (status = 409, description="Problems with Foreign Key in Database", body=String),
        (status = 500, description="Internal Server Error", body=String)
    ), 
)]
#[allow(dead_code)]
fn metrics_post() {}

/////////////////////////////////////////////////////////

#[utoipa::path(
    get, 
    path = "/api/v1/collector/{id}/notifications",
    responses(
        (status = 200, description="Notifications of Collector with ID", body=Vec<Notification>),
        (status = 409, description="Problems with Foreign Key in Database", body=String),
        (status = 500, description="Internal Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[allow(dead_code)]
fn get_collector_notifications() {}

#[utoipa::path(
    delete, 
    path = "/api/v1/collector/{id}/notifications",
    responses(
        (status = 200, description="Sucessfully deleted all Collector's Notifications"),
        (status = 409, description="Problems with Foreign Key in Database", body=String),
        (status = 500, description="Internal Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[allow(dead_code)]
fn delete_collector_notifications_all() {}

#[utoipa::path(
    delete, 
    path = "/api/v1/collector/{collector_id}/notifications/{notification_id}",
    responses(
        (status = 200, description="Sucessfully deleted Collector's Notification with ID"),
        (status = 409, description="Problems with Foreign Key in Database", body=String),
        (status = 500, description="Internal Server Error", body=String)
    ), 
    params (
        ("collector_id", Path, description="ID of Collector"),
        ("notification_id", Path, description="ID of Collector"),
    )
)]
#[allow(dead_code)]
fn delete_collector_notifications() {}

/////////////////////////////////////////////////////////

#[utoipa::path(
    get, 
    path = "/api/v1/collector/{id}/metrics_thresholds",
    responses(
        (status = 200, description="Metrics Thresholds from database", body=Vec<MetricsThreshold>),
        (status = 409, description="Problems with Foreign Key in Database", body=String),
        (status = 500, description="Internal Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[allow(dead_code)]
fn get_collector_metrics_thresholds() {}

#[utoipa::path(
    get, 
    path = "/api/v1/collector/{id}/endpoints_thresholds",
    responses(
        (status = 200, description="Endpoints Thresholds from database", body=Vec<EndpointsThreshold>),
        (status = 409, description="Problems with Foreign Key in Database", body=String),
        (status = 500, description="Internal Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[allow(dead_code)]
fn get_collector_endpoints_thresholds() {}

#[utoipa::path(
    get, 
    path = "/api/v1/collector/{id}/endpoints_thresholds_join",
    responses(
        (status = 200, description="Endpoints Thresholds with whole Endpoint from database", body=Vec<EndpointsThresholdsJoin>),
        (status = 409, description="Problems with Foreign Key in Database", body=String),
        (status = 500, description="Internal Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[allow(dead_code)]
fn get_collector_endpoints_thresholds_join() {}

#[utoipa::path(
    post, 
    path = "/api/v1/metrics_thresholds",
    responses(
        (status = 200, description="Sucessfully inserted, returning created Metrics Threshold", body=MetricsThreshold),
        (status = 409, description="Problems with Foreign Key in Database", body=String),
        (status = 500, description="Internal Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[allow(dead_code)]
fn post_metrics_thresholds() {}

#[utoipa::path(
    post, 
    path = "/api/v1/endpoints_thresholds",
    responses(
        (status = 200, description="Sucessfully inserted, returning created Endpoint Threshold", body=EndpointsThreshold),
        (status = 409, description="Problems with Foreign Key in Database", body=String),
        (status = 500, description="Internal Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[allow(dead_code)]
fn post_endpoints_thresholds() {}

#[utoipa::path(
    delete, 
    path = "/api/v1/metrics_thresholds/{threshold_id}",
    responses(
        (status = 200, description="Sucessfully deleted"),
        (status = 409, description="Problems with Foreign Key in Database", body=String),
        (status = 500, description="Internal Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[allow(dead_code)]
fn delete_metrics_thresholds() {}

#[utoipa::path(
    delete, 
    path = "/api/v1/endpoints_thresholds/{threshold_id}",
    responses(
        (status = 200, description="Sucessfully deleted"),
        (status = 409, description="Problems with Foreign Key in Database", body=String),
        (status = 500, description="Internal Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[allow(dead_code)]
fn delete_endpoints_thresholds() {}

#[utoipa::path(
    get, 
    path = "/api/v1/collector/{id}/metrics_thresholds/available_metric_types",
    responses(
        (status = 200, description="Metrics Types, which are not monitored for failure, thus are available to monitor", body=Vec<String>),
        (status = 409, description="Problems with Foreign Key in Database", body=String),
        (status = 500, description="Internal Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[allow(dead_code)]
fn get_collector_metrics_thresholds_available_metric_types() {}

#[utoipa::path(
    get, 
    path = "/api/v1/collector/{id}/metrics_thresholds/available_drives",
    responses(
        (status = 200, description="Drives, which are not monitored for failure, thus are available to monitor", body=Vec<DriveTable>),
        (status = 409, description="Problems with Foreign Key in Database", body=String),
        (status = 500, description="Internal Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[allow(dead_code)]
fn get_collector_metrics_thresholds_available_drives() {}

#[utoipa::path(
    get, 
    path = "/api/v1/collector/{id}/metrics_thresholds/available_network_interfaces_upload",
    responses(
        (status = 200, description="Network Interfaces, which are not monitored for Upload failure, thus are available to monitor", body=Vec<NetworkInterfaceTable>),
        (status = 409, description="Problems with Foreign Key in Database", body=String),
        (status = 500, description="Internal Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[allow(dead_code)]
fn get_collector_metrics_thresholds_available_networks_upload() {}

#[utoipa::path(
    get, 
    path = "/api/v1/collector/{id}/metrics_thresholds/available_network_interfaces_download",
    responses(
        (status = 200, description="Network Interfaces, which are not monitored for Download failure, thus are available to monitor", body=Vec<NetworkInterfaceTable>),
        (status = 409, description="Problems with Foreign Key in Database", body=String),
        (status = 500, description="Internal Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[allow(dead_code)]
fn get_collector_metrics_thresholds_available_networks_download() {}

#[utoipa::path(
    get, 
    path = "/api/v1/collector/{id}/endpoints_thresholds/available_endpoints",
    responses(
        (status = 200, description="Endpoints, which are not monitored for failure, thus are available to monitor", body=Vec<EndpointsTable>),
        (status = 409, description="Problems with Foreign Key in Database", body=String),
        (status = 500, description="Internal Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[allow(dead_code)]
fn get_collector_endpoints_available_endpoints() {}
