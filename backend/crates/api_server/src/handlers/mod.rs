use actix_web::{HttpResponse, Responder, get};
use serde::Deserialize;

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

// response body - jsor or none
enum ResponseBodyType {
    Json,
    None,
}

// params structs
#[derive(Deserialize, utoipa::IntoParams)]
struct MetricsQueryParams {
    time_limit_hours: Option<i32>,
    resolution: Option<i32>,
}

#[derive(Deserialize, utoipa::ToSchema)]
struct RenameCollectorStruct {
    name: String,
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
                ResponseBodyType::None => r.finish(),
            }
        }
        Err(val) => match val {
            shared::Error::DbForeignKey(fk) => HttpResponse::Conflict().body(fk.to_string()),
            shared::Error::DbNothingChanged => {
                // TODO might not be ID in all cases
                HttpResponse::NotModified().body("Nothing changed, specified ID not found")
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
#[get("")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}
