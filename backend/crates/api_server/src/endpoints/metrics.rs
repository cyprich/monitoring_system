use actix_web::{HttpResponse, Responder, post, web};
use shared::metrics::Metrics;

#[post("/metrics")]
pub async fn metrics_post(metrics: web::Json<Metrics>) -> impl Responder {
    // TODO
    println!("New metrics:\n{:?}", metrics);
    HttpResponse::NotImplemented()
}
