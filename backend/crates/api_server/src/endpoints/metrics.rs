use actix_web::{HttpResponse, Responder, post, web};
use shared::structs::metrics::Metrics;

use crate::AppState;

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

    let _ = state.tx.send(metrics);

    // todo
    HttpResponse::Ok().finish()
}
