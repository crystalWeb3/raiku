use futures_util::{stream::StreamExt, SinkExt};
use serde_json::json;
use tokio::time::{sleep, Duration};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;
use crate::transactions::process_transaction;

pub async fn start_websocket() {
    let solana_ws_url = "wss://api.devnet.solana.com"; // Change as needed

    let url = Url::parse(solana_ws_url).expect("Invalid WebSocket URL");
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    println!("Connected to Solana WebSocket");

    let (mut write, mut read) = ws_stream.split();

    let log_subscribe_msg = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "logsSubscribe",
        "params": ["all"]
    });

    write.send(Message::Text(log_subscribe_msg.to_string())).await.expect("Failed to send subscription");
    println!("Subscribed to log events");


    while let Some(message) = read.next().await {
        match message {
            Ok(Message::Text(text)) => {
                process_transaction(text).await;
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
                sleep(Duration::from_secs(5)).await;
            }
            _ => {}
        }
    }
}
