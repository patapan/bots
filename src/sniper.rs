
use websocket::WebSocket;

use crate::websocket;

// A raydium pool sniper bot
// It currently costs $300 to create a Raydium pool. 
// If you are able to snipe the creation, you can purchase tokens before the dev.

const RAYDIUM_POOL_ACCOUNT: String = "";

pub struct Sniper {
    pub websocket: WebSocket 
}


impl Sniper {
    pub async fn new(rpc: String) -> Self {
        Sniper {
            websocket: WebSocket::new(rpc).await
        }
    }

    pub fn run(&mut self) {

        self.websocket.open_channel("accountSubscribe".to_string(), RAYDIUM_POOL_ACCOUNT);
    }
}