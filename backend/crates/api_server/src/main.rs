use actix_cors::Cors;
use actix_web::{App, HttpServer};
use actix_web::{middleware, web};
use shared::structs::metrics::Metrics;
use tokio::sync::broadcast;

use crate::endpoints::*;
use db::Pool;

mod endpoints;

#[derive(Clone)]
pub struct AppState {
    pool: Pool,
    tx: broadcast::Sender<Metrics>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = shared::get_env("API_PORT").unwrap();

    let port: u16 = port
        .parse()
        .expect("Couldn't convert API_PORT environment variable to u16 type ");

    // TODO why couldnt
    let pool = db::get_pool().await.expect("Couldn't create database pool");

    let (tx, _) = broadcast::channel::<Metrics>(128);

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
            .service(get_collector_by_id)
            .service(get_collector_metrics)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
