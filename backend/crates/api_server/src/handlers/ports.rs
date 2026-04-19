use actix_web::{Responder, get, put, web};
use serde::Deserialize;
use shared::structs::ports::Port;

use crate::{
    AppState, WebSocketType, db,
    handlers::{ResponseBodyType, handle_query_error},
    notifications,
};

#[get("/collector/{id}/ports")]
pub async fn get_collector_ports(state: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let result = db::get_ports(&state.pool, id.into_inner()).await;
    handle_query_error(result, super::ResponseBodyType::Json)
}

async fn handle_port_open_or_close(
    state: web::Data<AppState>,
    collector_id: i32,
    ports: Vec<Port>,
    action_is_open: bool,
) -> impl Responder {
    let result = match action_is_open {
        true => db::put_opened_ports(&state.pool, collector_id, ports.clone()).await,
        false => db::put_closed_ports(&state.pool, collector_id, ports.clone()).await,
    };

    let result = match result {
        Ok(val) => val,
        Err(_) => {
            return handle_query_error(result, ResponseBodyType::None);
        }
    };

    if result.is_empty() {
        return handle_query_error(Ok(()), ResponseBodyType::None);
    }

    let websocket_type = match action_is_open {
        true => WebSocketType::PortsOpened(result),
        false => WebSocketType::PortsCloseed(result),
    };

    let result =
        notifications::handle_ports(&state.clone(), collector_id, ports, action_is_open).await;
    if let Err(val) = result {
        eprintln!("Error with evaluating ports notifications: {}", val);
    }

    let _ = state.tx.send((websocket_type, collector_id));

    handle_query_error(Ok(()), ResponseBodyType::None)
}

#[put("/collector/{id}/ports/opened")]
pub async fn put_collector_ports_opened(
    state: web::Data<AppState>,
    ports: web::Json<Vec<Port>>,
    id: web::Path<i32>,
) -> impl Responder {
    handle_port_open_or_close(state.clone(), id.into_inner(), ports.into_inner(), true).await
}

#[put("/collector/{id}/ports/closed")]
pub async fn put_collector_ports_closed(
    state: web::Data<AppState>,
    ports: web::Json<Vec<Port>>,
    id: web::Path<i32>,
) -> impl Responder {
    handle_port_open_or_close(state.clone(), id.into_inner(), ports.into_inner(), false).await
}

#[get("/collector/{id}/ports/notifications_settings")]
pub async fn get_collector_ports_notifications_settings(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    let result = db::get_collector_ports_notifications_settings(&state.pool, id.into_inner()).await;
    handle_query_error(result, ResponseBodyType::Json)
}

#[derive(Deserialize)]
struct Body {
    value: bool,
}

#[put("/collector/{id}/ports/notifications_settings/opened")]
pub async fn put_collector_ports_notifications_settings_opened(
    state: web::Data<AppState>,
    value: web::Json<Body>,
    id: web::Path<i32>,
) -> impl Responder {
    let result = db::update_collector_ports_notifications_settings(
        &state.pool,
        id.into_inner(),
        true,
        value.value,
    )
    .await;
    handle_query_error(result, ResponseBodyType::None)
}

#[put("/collector/{id}/ports/notifications_settings/closed")]
pub async fn put_collector_ports_notifications_settings_closed(
    state: web::Data<AppState>,
    value: web::Json<Body>,
    id: web::Path<i32>,
) -> impl Responder {
    let result = db::update_collector_ports_notifications_settings(
        &state.pool,
        id.into_inner(),
        false,
        value.value,
    )
    .await;
    handle_query_error(result, ResponseBodyType::None)
}
