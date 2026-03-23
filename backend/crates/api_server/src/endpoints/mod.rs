use actix_web::{HttpResponse, Responder, get};

mod collectors;
mod metrics;
mod ws;

pub use collectors::*;
pub use metrics::*;
pub use ws::*;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}
