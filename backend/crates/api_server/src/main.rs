use std::time::Duration;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use actix_web::{middleware, web};
use serde::Serialize;
use shared::structs::endpoints::EndpointResult;
use shared::structs::metrics::Metrics;
use shared::structs::notifications::Notification;
use tokio::sync::broadcast;
use tokio::time::sleep;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::handlers::*;
use db::Pool;

mod db;
mod handlers;
mod notifications;

const DELETE_RECORDS_AFTER_HOURS: f64 = 24.0;
const DELETE_DELAY_MINUTES: u64 = 5;

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", content = "data")]
enum WebSocketType {
    #[serde(rename = "metrics")]
    Metrics(Metrics),
    #[serde(rename = "endpoints_results")]
    EndpointResult(Vec<EndpointResult>),
    #[serde(rename = "notifications")]
    Notifications(Vec<Notification>),
}

#[derive(Clone)]
pub struct AppState {
    pool: Pool,
    // type of data transferred, collector id
    tx: broadcast::Sender<(WebSocketType, i32)>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // get port
    let port =
        shared::env::get("API_PORT").map_err(|val| std::io::Error::other(val.to_string()))?;
    let port: u16 = match port.parse() {
        Ok(val) => val,
        Err(val) => {
            eprintln!("Couldn't convert API_PORT environment variable to u16 type");
            return Err(std::io::Error::other(val));
        }
    };

    // create pool
    let pool = match db::get_pool().await {
        Ok(val) => val,
        Err(val) => {
            eprintln!("Couldn't create database pool: {}", val);
            return Err(std::io::Error::other(val.to_string()));
        }
    };

    // delete old metrics and endpoitns_results
    let pool_clone = pool.clone();
    tokio::spawn(async move {
        loop {
            let result = db::delete_old_records(&pool_clone).await;
            if let Err(val) = result {
                eprintln!("Error deleting old records: {}", val)
            }
            sleep(Duration::from_mins(DELETE_DELAY_MINUTES)).await;
        }
    });

    // prepare app state and apidocs
    let (tx, _) = broadcast::channel::<(WebSocketType, i32)>(128);
    let state = AppState { pool, tx };
    let openapi = ApiDoc::openapi();

    // run web server
    HttpServer::new(move || {
        App::new()
            // TODO
            .wrap(Cors::permissive())
            .app_data(web::Data::new(state.clone()))
            .service(
                web::scope("/api/v1")
                    .wrap(middleware::NormalizePath::trim())
                    .service(hello)
                    .service(ws)
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
                    .service(get_collector_notifications)
                    .service(delete_collector_notifications)
                    .service(delete_collector_notifications_all)
                    .service(rename_collector)
                    .service(get_collector_metrics_thresholds)
                    .service(get_collector_metrics_thresholds_available_metric_types)
                    .service(get_collector_metrics_thresholds_available_drives)
                    .service(get_collector_metrics_thresholds_available_networks_upload)
                    .service(get_collector_metrics_thresholds_available_networks_download)
                    .service(get_collector_endpoints_thresholds)
                    .service(get_collector_endpoints_thresholds_join)
                    .service(get_collector_endpoints_available_endpoints)
                    .service(post_metrics_thresholds)
                    .service(post_endpoints_thresholds)
                    .service(delete_metrics_thresholds)
                    .service(delete_endpoints_thresholds),
            )
            .service(
                SwaggerUi::new("/swagger_ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
