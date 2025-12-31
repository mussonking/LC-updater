pub mod logic;
pub mod ws_server;
pub mod commands;

use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AppState {
    pub clients: ws_server::WsClients,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
