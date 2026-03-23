use actix_web::{HttpResponse, Responder, post, web};
use shared::structs::metrics::Metrics;

use crate::AppState;

#[post("/metrics")]
pub async fn metrics_post(
    state: web::Data<AppState>,
    metrics: web::Json<Metrics>,
) -> impl Responder {
    let metrics = metrics.into_inner();

    db::insert_metrics(&state.pool, &metrics).await;
    let _ = state.tx.send(metrics);

    // todo
    HttpResponse::Ok()
}
