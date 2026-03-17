use actix_web::{HttpResponse, Responder, get, post, web};
use shared::structs::UnidentifiedCollector;

mod metrics;

use crate::{AppState, db};
pub use metrics::*;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
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
