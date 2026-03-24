use actix_web::{Responder, get};

use crate::{AppState, endpoints::handle_db_error};

use actix_web::{post, web};
use shared::structs::unidentified_collector::UnidentifiedCollector;

#[get("/collectors")]
async fn collectors(state: web::Data<AppState>) -> impl Responder {
    let result = db::get_collectors(&state.pool).await;
    handle_db_error(result)
}

#[get("/collector/{id}")]
async fn get_collector_by_id(state: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let result = db::get_collector_by_id(&state.pool, id.into_inner()).await;
    handle_db_error(result)
}

#[get("/collector/{id}/metrics")]
async fn get_collector_metrics(state: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let result = db::get_collector_metrics(&state.pool, id.into_inner()).await;
    handle_db_error(result)
}

#[post("/collector/register")]
async fn collector_register(
    state: web::Data<AppState>,
    new_collector: web::Json<UnidentifiedCollector>,
) -> impl Responder {
    let result = db::register_collector(&state.pool, &new_collector.into_inner()).await;
    handle_db_error(result)
}
