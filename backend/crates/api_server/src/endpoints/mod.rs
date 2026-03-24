use actix_web::{HttpResponse, Responder, get};

mod collectors;
mod metrics;
mod ws;

pub use collectors::*;
pub use metrics::*;
use shared::DatabaseError;
pub use ws::*;

fn handle_db_error<T: serde::Serialize>(result: Result<T, DatabaseError>) -> impl Responder {
    match result {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(val) => HttpResponse::InternalServerError().body(val.to_string()),
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}
