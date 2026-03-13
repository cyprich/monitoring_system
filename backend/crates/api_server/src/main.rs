use actix_cors::Cors;
use actix_web::{App, HttpServer};

use crate::endpoints::*;
use crate::ws::*;

mod endpoints;
mod ws;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = shared::get_env("API_PORT").unwrap();

    let port: u16 = port
        .parse()
        .expect("Couldn't convert API_PORT environment variable to u16 type ");

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(hello)
            .service(ws)
            .service(metrics_post)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
