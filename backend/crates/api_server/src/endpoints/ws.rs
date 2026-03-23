use actix_web::{Error, HttpRequest, HttpResponse, get, rt, web};
use actix_ws::Message;
use futures_util::StreamExt as _;

use crate::AppState;

#[get("/ws/metrics/{id}")]
pub async fn ws_metrics(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let (res, mut send, mut receive) = actix_ws::handle(&req, stream)?;
    let id = path.into_inner();

    rt::spawn(async move {
        let mut rx = state.tx.subscribe();

        loop {
            tokio::select! {
                // messages from ws
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

                // TODO maybe if needed in the future, make it able to receive multiple different
                // messages with some kind of enum

                // metrics from broadcast
                metrics = rx.recv() => {
                    if let Ok(metrics) = metrics && metrics.collector_id == id {
                            send.text(metrics.json()).await.unwrap();
                        }
                }
            }
        }
    });

    Ok(res)
}
