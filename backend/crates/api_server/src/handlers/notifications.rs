use actix_web::{Responder, delete, get, web};

use crate::{
    AppState, db,
    handlers::{ResponseBodyType, handle_query_error},
};

#[get("/collector/{id}/notifications")]
pub async fn get_collector_notifications(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    let result = db::get_collector_notifications(&state.pool, id.into_inner()).await;
    handle_query_error(result, ResponseBodyType::Json)
}

#[delete("/collector/{id}/notifications")]
pub async fn delete_collector_notifications_all(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    let result = db::remove_collector_notifications(&state.pool, id.into_inner(), None).await;
    handle_query_error(result, ResponseBodyType::None)
}

#[delete("/collector/{collector_id}/notifications/{notification_id}")]
pub async fn delete_collector_notifications(
    state: web::Data<AppState>,
    id: web::Path<(i32, i32)>,
) -> impl Responder {
    let result = db::remove_collector_notifications(&state.pool, id.0, Some(id.1)).await;
    handle_query_error(result, ResponseBodyType::None)
}
