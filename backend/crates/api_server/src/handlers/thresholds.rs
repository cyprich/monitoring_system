use actix_web::{Responder, delete, get, web};

use crate::{
    AppState, db,
    handlers::{ResponseBodyType, handle_query_error},
};

#[get("/collector/{id}/metrics_thresholds")]
pub async fn get_collector_metrics_thresholds(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    let result = db::get_metrics_thresholds(&state.pool, id.into_inner()).await;
    handle_query_error(result, ResponseBodyType::Json)
}

#[get("/collector/{id}/endpoints_thresholds")]
pub async fn get_collector_endpoints_thresholds(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    let result = db::get_endpoints_thresholds(&state.pool, id.into_inner()).await;
    handle_query_error(result, ResponseBodyType::Json)
}

#[delete("/metrics_thresholds/{threshold_id}")]
pub async fn delete_metrics_thresholds(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    let result = db::delete_metrics_thresholds(&state.pool, id.into_inner()).await;
    handle_query_error(result, ResponseBodyType::None)
}

#[delete("/endpoints_thresholds/{threshold_id}")]
pub async fn delete_endpoints_thresholds(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    let result = db::delete_endpoints_thresholds(&state.pool, id.into_inner()).await;
    handle_query_error(result, ResponseBodyType::None)
}
