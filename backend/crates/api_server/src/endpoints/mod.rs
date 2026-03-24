use actix_web::{HttpResponse, Responder, get};

mod collectors;
mod metrics;
mod ws;

pub use collectors::*;
pub use metrics::*;
pub use ws::*;

fn handle_db_error<T: serde::Serialize>(result: Result<T, shared::Error>) -> impl Responder {
    match result {
        Ok(val) => HttpResponse::Ok().json(val),
        // TODO temp
        Err(val) => HttpResponse::InternalServerError().body(format!("{:?}", val)),
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}
