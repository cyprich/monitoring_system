use actix_web::{HttpResponse, Responder, get, patch};
use serde::Deserialize;

use crate::{
    AppState,
    endpoints::{ResponseBodyType, handle_query_error},
};

use actix_web::{post, web};
use shared::structs::unidentified_collector::UnidentifiedCollector;

#[get("/collectors")]
async fn collectors(state: web::Data<AppState>) -> impl Responder {
    let result = db::get_collectors(&state.pool).await;
    handle_query_error(result, ResponseBodyType::Json)
}

#[get("/collector/{id}")]
async fn get_collector_by_id(state: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let result = db::get_collector_by_id(&state.pool, id.into_inner()).await;
    handle_query_error(result, ResponseBodyType::Json)
}

#[get("/collector/{id}/metrics")]
async fn get_collector_metrics(state: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let result = db::get_collector_metrics(&state.pool, id.into_inner()).await;
    handle_query_error(result, ResponseBodyType::Json)
}

#[post("/collector/register")]
async fn collector_register(
    state: web::Data<AppState>,
    new_collector: web::Json<UnidentifiedCollector>,
) -> impl Responder {
    let result = db::register_collector(&state.pool, &new_collector.into_inner()).await;
    match result {
        Ok(val) => HttpResponse::Created().body(val.to_string()),
        Err(val) => HttpResponse::InternalServerError().body(val.to_string()),
    }
}

#[derive(Deserialize)]
struct RenameCollectorStruct {
    name: String,
}

#[patch("/collector/{id}/rename")]
async fn rename_collector(
    state: web::Data<AppState>,
    id: web::Path<i32>,
    body: web::Json<RenameCollectorStruct>,
) -> impl Responder {
    let result = db::rename_collector(&state.pool, id.into_inner(), body.into_inner().name).await;
    handle_query_error(result, ResponseBodyType::None)
}
