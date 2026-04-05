use actix_web::{Responder, delete, get, web};

use crate::{
    AppState, db,
    handlers::{ResponseBodyType, handle_query_error},
};

use shared::structs::notifications::Notification;

#[utoipa::path(
    responses(
        (status = 200, description="Notifications of Collector with ID", body=Vec<Notification>),
        (status = 500, description="Interval Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[get("/collector/{id}/notifications")]
pub async fn get_collector_notifications(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    let result = db::get_notifications(&state.pool, id.into_inner()).await;
    handle_query_error(result, ResponseBodyType::Json)
}

#[utoipa::path(
    responses(
        (status = 200, description="Sucessfully deleted all Collector's Notifications"),
        (status = 500, description="Interval Server Error", body=String)
    ), 
    params (
        ("id", Path, description="ID of Collector"),
    )
)]
#[delete("/collector/{id}/notifications")]
pub async fn delete_collector_notifications_all(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    let result = db::delete_notification(&state.pool, id.into_inner(), None).await;
    handle_query_error(result, ResponseBodyType::None)
}

#[utoipa::path(
    responses(
        (status = 200, description="Sucessfully deleted Collector's Notification with ID"),
        (status = 500, description="Interval Server Error", body=String)
    ), 
    params (
        ("collector_id", Path, description="ID of Collector"),
        ("notification_id", Path, description="ID of Collector"),
    )
)]
#[delete("/collector/{collector_id}/notifications/{notification_id}")]
pub async fn delete_collector_notifications(
    state: web::Data<AppState>,
    id: web::Path<(i32, i32)>,
) -> impl Responder {
    let result = db::delete_notification(&state.pool, id.0, Some(id.1)).await;
    handle_query_error(result, ResponseBodyType::None)
}
