use std::time::Duration;

use futures_util::SinkExt;
use tokio::{net::TcpListener, time::sleep};
use tokio_tungstenite::accept_async;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8000";
    let listener = TcpListener::bind(addr)
        .await
        .unwrap_or_else(|err| panic!("Failed to start server at {}; {}", addr, err));

    println!("Listening on {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let mut system = data_collection::System::new().unwrap();

            let addr = stream.peer_addr().unwrap();
            let mut ws_stream = accept_async(stream).await.unwrap();
            println!("New connection from {}", addr);

            loop {
                let data = system.get_data().cpu_usage[0];

                ws_stream.send(data.to_string().into()).await.unwrap();
                sleep(Duration::from_secs(1)).await;
            }
        });
    }

    println!("Hello, world!");
}
