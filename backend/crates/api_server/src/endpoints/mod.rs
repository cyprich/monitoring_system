use actix_web::{HttpResponse, Responder, get};

mod metrics;

pub use metrics::*;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}
