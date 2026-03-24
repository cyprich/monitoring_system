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
        match val {
            DatabaseError::Database(error) => {
                return HttpResponse::InternalServerError().body(error.to_string());
            }
            DatabaseError::ForeignKey => return HttpResponse::Unauthorized().finish(),
        }
    }

    let _ = state.tx.send(metrics);

    // todo
    HttpResponse::Ok().finish()
}
