use actix_web::{HttpResponse, Responder, post, web};
use shared::structs::Metrics;

use crate::db::{self, Pool};

#[post("/metrics")]
pub async fn metrics_post(pool: web::Data<Pool>, metrics: web::Json<Metrics>) -> impl Responder {
    // TODO
    // println!("New metrics:\n{:?}", metrics);
    db::insert_metrics(pool.get_ref(), metrics.into_inner()).await;
    HttpResponse::NotImplemented()
}
