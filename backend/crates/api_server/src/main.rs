use actix_cors::Cors;
use actix_web::{App, HttpServer};
use actix_web::{middleware, web};
use serde::Serialize;
use shared::structs::endpoints::EndpointResult;
use shared::structs::metrics::Metrics;
use tokio::sync::broadcast;

use crate::endpoints::*;
use db::Pool;

mod endpoints;
mod db;

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", content = "data")]
enum WebSocketType {
    #[serde(rename = "metrics")]
    Metrics(Metrics),
    #[serde(rename = "endpoints_results")]
    EndpointResult(Vec<EndpointResult>),
}

#[derive(Clone)]
pub struct AppState {
    pool: Pool,
    // type of data transferred, collector id
    tx: broadcast::Sender<(WebSocketType, i32)>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port =
    // TODO temp
        shared::env::get("API_PORT").map_err(|val| std::io::Error::other(format!("{:?}", val)))?;

    let port: u16 = port
        .parse()
        .expect("Couldn't convert API_PORT environment variable to u16 type ");

    // TODO why couldnt
    let pool = db::get_pool().await.expect("Couldn't create database pool");

    let (tx, _) = broadcast::channel::<(WebSocketType, i32)>(128);

    let state = AppState { pool, tx };

    HttpServer::new(move || {
        App::new()
            // TODO
            .wrap(Cors::permissive())
            .wrap(middleware::NormalizePath::trim())
            .app_data(web::Data::new(state.clone()))
            .service(hello)
            .service(ws_metrics)
            .service(metrics_post)
            .service(collector_register)
            .service(collectors)
            .service(get_collector)
            .service(get_collector_metrics)
            .service(get_collector_drives)
            .service(get_collector_network_interfaces)
            .service(get_collector_endpoints)
            .service(post_collector_endpoints)
            .service(put_collector_endpoints)
            .service(delete_collector_endpoints)
            .service(get_collector_endpoint_results)
            .service(get_collector_endpoint_results_last)
            .service(post_collector_endpoint_results)
            .service(rename_collector)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
