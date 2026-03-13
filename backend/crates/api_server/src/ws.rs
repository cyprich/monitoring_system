use std::time::Duration;

use actix_web::{Error, HttpRequest, HttpResponse, get, rt, web};
use actix_ws::Message;
use futures_util::StreamExt as _;
use tokio::time::interval;

#[get("/ws")]
pub async fn ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, mut send, mut receive) = actix_ws::handle(&req, stream)?;

    let mut collector = collector::Collector::new().unwrap();

    rt::spawn(async move {
        println!("New connection");

        let mut interval = interval(Duration::from_secs(1));

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    let data = collector.get_metrics().json();
                    send.text(data).await.unwrap();
                }

                msg = receive.next() => {
                    if let Some(msg) = msg {
                        match msg {
                            Ok(Message::Text(text)) => {
                                println!("Message: {}", text);
                                send.text(text).await.unwrap();
                            },
                            Ok(Message::Binary(bin)) => {
                                println!("Binary: {:?}", bin);
                                send.binary(bin).await.unwrap();
                            },
                            Ok(Message::Ping(bin)) => {
                                println!("Ping: {:?}", bin);
                                send.pong(&bin).await.unwrap();
                            },
                            Ok(Message::Close(reason)) => {
                                println!("Close: {:?}", reason);
                                send.close(reason).await.unwrap();
                                break;
                            }
                            Err(e) => {
                                println!("Error: {}", e);
                                break;
                            },
                            _ => {}
                        }
                    }
                }
            }
        }
    });

    Ok(res)
}
