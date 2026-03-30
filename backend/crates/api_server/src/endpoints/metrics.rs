use actix_web::{HttpResponse, Responder, post, web};
use shared::structs::metrics::Metrics;

use crate::{AppState, WebSocketType};

#[post("/metrics")]
pub async fn metrics_post(
    state: web::Data<AppState>,
    metrics: web::Json<Metrics>,
) -> impl Responder {
    let metrics = metrics.into_inner();
    let result = db::insert_metrics(&state.pool, &metrics).await;

    if let Err(val) = result {
        return match val {
            shared::Error::DbForeignKey(error) => {
                HttpResponse::Unauthorized().body(error.to_string())
            }
            _ => HttpResponse::InternalServerError().body(val.to_string()),
        };
    }

    let id = metrics.collector_id;
    let _ = state.tx.send((WebSocketType::Metrics(metrics), id));

    // todo
    HttpResponse::Ok().finish()
}
