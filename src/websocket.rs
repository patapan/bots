use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use url::Url;
use futures_util::{stream::{SplitSink, SplitStream, StreamExt}, SinkExt};
use serde_json::json;

pub struct Connection {
    pub stream_id: usize, // used to define the stream  within internal system
    pub unsub_id: u64,  // used to unsub from socket connection
    pub method: String, // rpc method sub
}

pub struct WebSocket {
    write: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    active_connections: Vec<Connection> // ordered by stream ID
}

impl WebSocket {
    pub async fn new(rpc: String) -> Self {
        let (ws_stream, _) = connect_async(Url::parse(&rpc).expect("Invalid rpc format")).await.expect("Failed to connect");
        let (write, read) = ws_stream.split();
        println!("WebSocket handshake has been successfully completed");
        WebSocket { write, read, active_connections: Vec::new()}
    }

    // subscribe to new method 
    pub async fn open_channel(&mut self, method: String, account: String) {
        let message = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": [
                account,
                {
                    "encoding": "jsonParsed",
                    "commitment": "finalized"
                }
            ]
        })
        .to_string();

        self.write
        .send(Message::Text(message))
        .await
        .expect("Failed to send subscribe message");
    }

    pub async fn close_channel(&mut self, method: String, id: usize) {
        let connection = self.active_connections.get(id).expect(&format!("Failed to retrieve connection with ID {}", id));
        let message = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": [
                connection.unsub_id
            ]
        })
        .to_string();

        self.write
        .send(Message::Text(message))
        .await
        .expect("Failed to send unsubscribe message");
    }

    // infinitely loop to process incoming messages
    pub async fn read(&mut self) {

        while let Some(message) = self.read.next().await {
            let text = match message {
                Ok(Message::Text(t)) => t,
                Err(e) => {
                    println!("Error reading message: {}", e);
                    break; // or handle the error as needed
                },
                _ => continue, // Skip non-text messages
            };

            let parsed_json = match serde_json::from_str::<serde_json::Value>(&text) {
                Ok(json) => json,
                Err(_) => {
                    println!("Received a message that's not valid JSON.");
                    continue;
                },
            };

            let id = match parsed_json.get("id").and_then(|id| id.as_i64()) {
                Some(id) => id,
                None => {
                    println!("Received a message without a numeric ID or not related to subscriptions.");
                    continue;
                },
            };

            println!("Received message with ID: {}", id);

            if let Some(sub_id) = parsed_json.get("result").and_then(|r| r.as_i64()) {
                // Store or process the subscription ID
                println!("Subscription ID received: {}", sub_id);
            } else {
                println!("Got here");
                // Handle other types of messages that have an ID but are not subscription confirmations
            }
        }
    }
}