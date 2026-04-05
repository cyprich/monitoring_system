use actix_web::{Responder, delete, get, post, web};
use shared::structs::{db::{DriveTable, EndpointsTable, EndpointsThresholdsJoin, NetworkInterfaceTable}, thresholds::{EndpointsThreshold, MetricsThreshold}};

use crate::{
    AppState, db,
    handlers::{ResponseBodyType, handle_query_error},
};

#[utoipa::path(
    responses(
        (status = 200, description="Metrics Thresholds from database", body=Vec<MetricsThreshold>),
        (status = 500, description="Interval Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[get("/collector/{id}/metrics_thresholds")]
pub async fn get_collector_metrics_thresholds(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    let result = db::get_metrics_thresholds(&state.pool, id.into_inner()).await;
    handle_query_error(result, ResponseBodyType::Json)
}

#[utoipa::path(
    responses(
        (status = 200, description="Endpoints Thresholds from database", body=Vec<EndpointsThreshold>),
        (status = 500, description="Interval Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[get("/collector/{id}/endpoints_thresholds")]
pub async fn get_collector_endpoints_thresholds(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    let result = db::get_endpoints_thresholds(&state.pool, id.into_inner()).await;
    handle_query_error(result, ResponseBodyType::Json)
}

#[utoipa::path(
    responses(
        (status = 200, description="Endpoints Thresholds with whole Endpoint from database", body=Vec<EndpointsThresholdsJoin>),
        (status = 500, description="Interval Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[get("/collector/{id}/endpoints_thresholds_join")]
pub async fn get_collector_endpoints_thresholds_join(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    let result = db::get_endpoints_thresholds_join(&state.pool, id.into_inner()).await;
    handle_query_error(result, ResponseBodyType::Json)
}

#[utoipa::path(
    responses(
        (status = 200, description="Sucessfully inserted, returning created Metrics Threshold", body=MetricsThreshold),
        (status = 500, description="Interval Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[post("/metrics_thresholds")]
pub async fn post_metrics_thresholds(
    state: web::Data<AppState>,
    json: web::Json<MetricsThreshold>,
) -> impl Responder {
    let result = db::insert_metrics_thresholds(&state.pool, json.into_inner()).await;
    handle_query_error(result, ResponseBodyType::Json)
}

#[utoipa::path(
    responses(
        (status = 200, description="Sucessfully inserted, returning created Endpoint Threshold", body=EndpointsThreshold),
        (status = 500, description="Interval Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[post("/endpoints_thresholds")]
pub async fn post_endpoints_thresholds(
    state: web::Data<AppState>,
    json: web::Json<EndpointsThreshold>,
) -> impl Responder {
    let result = db::insert_endpoints_thresholds(&state.pool, json.into_inner()).await;
    handle_query_error(result, ResponseBodyType::Json)
}

#[utoipa::path(
    responses(
        (status = 200, description="Sucessfully deleted"),
        (status = 500, description="Interval Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[delete("/metrics_thresholds/{threshold_id}")]
pub async fn delete_metrics_thresholds(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    let result = db::delete_metrics_thresholds(&state.pool, id.into_inner()).await;
    handle_query_error(result, ResponseBodyType::None)
}

#[utoipa::path(
    responses(
        (status = 200, description="Sucessfully deleted"),
        (status = 500, description="Interval Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[delete("/endpoints_thresholds/{threshold_id}")]
pub async fn delete_endpoints_thresholds(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    let result = db::delete_endpoints_thresholds(&state.pool, id.into_inner()).await;
    handle_query_error(result, ResponseBodyType::None)
}

#[utoipa::path(
    responses(
        (status = 200, description="Metrics Types, which are not monitored for failure, thus are available to monitor", body=Vec<String>),
        (status = 500, description="Interval Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[get("/collector/{id}/metrics_thresholds/available_metric_types")]
pub async fn get_collector_metrics_thresholds_available_metric_types(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    let result = db::get_available_metric_types(&state.pool, id.into_inner()).await;
    handle_query_error(result, ResponseBodyType::Json)
}

// TODO merge these three into one with {metric_type} in web::Path

#[utoipa::path(
    responses(
        (status = 200, description="Drives, which are not monitored for failure, thus are available to monitor", body=Vec<DriveTable>),
        (status = 500, description="Interval Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[get("/collector/{id}/metrics_thresholds/available_drives")]
pub async fn get_collector_metrics_thresholds_available_drives(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    let result = db::get_available_drives(&state.pool, id.into_inner()).await;
    handle_query_error(result, ResponseBodyType::Json)
}

#[utoipa::path(
    responses(
        (status = 200, description="Network Interfaces, which are not monitored for Upload failure, thus are available to monitor", body=Vec<NetworkInterfaceTable>),
        (status = 500, description="Interval Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[get("/collector/{id}/metrics_thresholds/available_network_interfaces_upload")]
pub async fn get_collector_metrics_thresholds_available_networks_upload(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    let result = db::get_available_network_interfaces(
        &state.pool,
        id.into_inner(),
        db::NetworkMetricType::Upload,
    )
    .await;
    handle_query_error(result, ResponseBodyType::Json)
}

#[utoipa::path(
    responses(
        (status = 200, description="Network Interfaces, which are not monitored for Download failure, thus are available to monitor", body=Vec<NetworkInterfaceTable>),
        (status = 500, description="Interval Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[get("/collector/{id}/metrics_thresholds/available_network_interfaces_download")]
pub async fn get_collector_metrics_thresholds_available_networks_download(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    let result = db::get_available_network_interfaces(
        &state.pool,
        id.into_inner(),
        db::NetworkMetricType::Download,
    )
    .await;
    handle_query_error(result, ResponseBodyType::Json)
}

#[utoipa::path(
    responses(
        (status = 200, description="Endpoints, which are not monitored for failure, thus are available to monitor", body=Vec<EndpointsTable>),
        (status = 500, description="Interval Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[get("/collector/{id}/endpoints_thresholds/available_endpoints")]
pub async fn get_collector_endpoints_available_endpoints(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    let result = db::get_available_endpoints(&state.pool, id.into_inner()).await;
    handle_query_error(result, ResponseBodyType::Json)
}
