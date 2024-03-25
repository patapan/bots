use std::thread::spawn;
use bots::websocket;
use websocket::WebSocket;
use tokio;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let rpc_url = std::env::var("RPC_ENDPOINT").expect("RPC_ENDPOINT must be set");

    // Spawn the WebSocket handler in a separate asynchronous task
    tokio::spawn(async move {
        let mut sniper = Sniper::new(format!("wss://{}", rpc_url)).await;
    });

    // Keep the main thread running, waiting for the spawned task(s)
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}
