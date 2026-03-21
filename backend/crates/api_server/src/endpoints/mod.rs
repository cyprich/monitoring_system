use actix_web::{HttpResponse, Responder, get, post, web};
use shared::structs::unidentified_collector::UnidentifiedCollector;

mod metrics;
mod ws;

pub use metrics::*;
pub use ws::*;

use crate::{AppState, db};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[get("/collectors")]
async fn collectors(state: web::Data<AppState>) -> impl Responder {
    let result = db::get_collectors(&state.pool).await;
    HttpResponse::Ok().json(result)
}

#[get("/collector/{id}")]
async fn get_collector_by_id(state: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let result = db::get_collector_by_id(&state.pool, id.into_inner()).await;

    match result {
        Some(val) => HttpResponse::Ok().json(val),
        None => HttpResponse::NotFound().finish(),
    }
}

#[post("/collector/register")]
async fn collector_register(
    state: web::Data<AppState>,
    new_collector: web::Json<UnidentifiedCollector>,
) -> impl Responder {
    let result = db::register_collector(&state.pool, &new_collector.into_inner()).await;

    match result {
        Some(val) => HttpResponse::Created().body(val.to_string()),
        None => HttpResponse::InternalServerError().finish(),
    }
}
