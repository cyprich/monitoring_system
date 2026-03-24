use actix_web::{HttpResponse, Responder, post, web};
use shared::{DatabaseError, structs::metrics::Metrics};

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
            DatabaseError::ForeignKey => HttpResponse::Unauthorized().finish(),
            DatabaseError::Database(val) => {
                HttpResponse::InternalServerError().body(val.to_string())
            }
            DatabaseError::Env(val) => HttpResponse::InternalServerError().body(val.to_string()),
        };
    }

    let _ = state.tx.send(metrics);

    // todo
    HttpResponse::Ok().finish()
}
