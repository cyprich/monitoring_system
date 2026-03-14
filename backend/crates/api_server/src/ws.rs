use actix_web::{Error, HttpRequest, HttpResponse, get, rt, web};
use actix_ws::Message;
use futures_util::StreamExt as _;

use crate::AppState;

#[get("/ws")]
pub async fn ws(
    state: web::Data<AppState>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let (res, mut send, mut receive) = actix_ws::handle(&req, stream)?;

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

                // metrics from broadcast
                metrics = rx.recv() => {
                    if let Ok(metrics) = metrics {
                        send.text(metrics.json()).await.unwrap();
                    }
                }
            }
        }
    });

    Ok(res)
}
