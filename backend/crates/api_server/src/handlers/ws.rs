use actix_web::{Error, HttpRequest, HttpResponse, get, rt, web};
use actix_ws::Message;
use futures_util::StreamExt as _;

use crate::AppState;

#[get("/ws/collector/{id}")]
pub async fn ws_metrics(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let (resp, mut send, mut receive) = actix_ws::handle(&req, stream)?;
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

                // messages from broadcast
                broadcast_recv = rx.recv() => {
                    if let Ok((websocket_type, collector_id)) = broadcast_recv && collector_id == id {
                        let val = serde_json::to_string(&websocket_type).unwrap();
                        send.text(val).await.unwrap();
                    }
                }
            }
        }
    });

    Ok(resp)
}
