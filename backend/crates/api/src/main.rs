use actix_cors::Cors;
use actix_web::{App, HttpServer};

use crate::endpoints::*;
use crate::ws::*;

mod endpoints;
mod ws;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if let Err(val) = dotenvy::dotenv() {
        log::error!("{}", val);
        return Err(std::io::Error::other(val));
    }

    let port = std::env::var("API_PORT").expect("Couldn't find API_PORT environment variable");
    let port: u16 = port
        .parse()
        .expect("Couldn't convert API_PORT environment variable to u16 type ");

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(hello)
            .service(ws)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
