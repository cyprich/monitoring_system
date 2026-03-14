use actix_cors::Cors;
use actix_web::{App, HttpServer};
use actix_web::{middleware, web};

use crate::endpoints::*;
use crate::ws::*;

mod db;
mod endpoints;
mod ws;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = shared::get_env("API_PORT").unwrap();

    let port: u16 = port
        .parse()
        .expect("Couldn't convert API_PORT environment variable to u16 type ");

    // TODO why couldnt
    let pool = crate::db::get_pool()
        .await
        .expect("Couldn't create database pool");

    HttpServer::new(move || {
        App::new()
            // TODO
            .wrap(Cors::permissive())
            .wrap(middleware::NormalizePath::trim())
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
            .service(ws)
            .service(metrics_post)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
