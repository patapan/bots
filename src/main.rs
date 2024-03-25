use std::thread::spawn;
use bots::websocket;
use websocket::WebSocket;
use tokio;

#[tokio::main]
async fn main() {
    // Load environment variables from a .env file (optional, for development)
    dotenv::dotenv().ok();

    // Retrieve the RPC URL from an environment variable
    let rpc_url = std::env::var("RPC_ENDPOINT").expect("RPC_ENDPOINT must be set");

    // Spawn the WebSocket handler in a separate asynchronous task
    tokio::spawn(async move {
        let mut websocket = WebSocket::new(format!("wss://{}", rpc_url)).await;
        // Assuming you have methods to open channels or subscribe to topics
        // websocket.open_channel("yourMethod", "yourAccount").await;

        // Call the read method to process incoming messages indefinitely
        websocket.read().await;
    });

    // Keep the main thread running, waiting for the spawned task(s)
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}
