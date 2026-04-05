use actix_web::{HttpResponse, Responder, get};

mod collectors;
mod metrics;
mod notifications;
mod thresholds;
mod ws;
mod api_docs;

pub use collectors::*;
pub use metrics::*;
pub use notifications::*;
pub use thresholds::*;
pub use ws::*;
pub use api_docs::*;

enum ResponseBodyType {
    Json,
    // Body { value: String },
    None,
}

fn handle_query_error<T: serde::Serialize>(
    result: Result<T, shared::Error>,
    body_type: ResponseBodyType,
) -> HttpResponse {
    match result {
        Ok(val) => {
            let mut r = HttpResponse::Ok();
            match body_type {
                ResponseBodyType::Json => r.json(val),
                // ResponseBodyType::Body { value } => r.body(value),
                ResponseBodyType::None => r.finish(),
            }
        }
        Err(val) => match val {
            shared::Error::DbForeignKey(fk) => HttpResponse::Unauthorized().body(fk.to_string()),
            shared::Error::DbNothingChanged => {
                // TODO might not be ID in all cases
                HttpResponse::NotFound().body("Nothing changed, specified ID not found")
            }
            // TODO more cases if needed
            _ => HttpResponse::InternalServerError().body(val.to_string()),
        },
    }
}

#[utoipa::path(
    responses(
        (status = 200, description="API Server reachable", body=String),
    ), 
)]
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}
