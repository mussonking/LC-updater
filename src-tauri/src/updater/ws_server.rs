use std::sync::Arc;
use tokio::sync::Mutex;
use futures_util::{StreamExt, SinkExt};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;

pub type WsClients = Arc<Mutex<Vec<futures_util::stream::SplitSink<tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>, Message>>>>;

pub async fn start_ws_server(clients: WsClients) {
    let listener = TcpListener::bind("127.0.0.1:8888").await.expect("Failed to bind");
    while let Ok((stream, _)) = listener.accept().await {
        let clients = Arc::clone(&clients);
        tokio::spawn(async move {
            if let Ok(ws_stream) = accept_async(stream).await {
                let (write, mut read) = ws_stream.split();
                clients.lock().await.push(write);
                while let Some(_) = read.next().await {}
            }
        });
    }
}

pub async fn broadcast_reload(clients: &WsClients) {
    let mut clients_lock = clients.lock().await;
    let message = serde_json::json!({ "action": "RELOAD" }).to_string();
    
    let mut to_remove = Vec::new();
    for (i, client) in clients_lock.iter_mut().enumerate() {
        if let Err(_) = client.send(Message::Text(message.clone())).await {
            to_remove.push(i);
        }
    }
    
    for i in to_remove.into_iter().rev() {
        clients_lock.remove(i);
    }
}
