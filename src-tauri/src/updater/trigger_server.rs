use tauri::{AppHandle, Emitter, Manager};
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn start_trigger_server(app: AppHandle) {
    let addr = "127.0.0.1:8889";
    let listener = match TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to bind trigger server to {}: {}", addr, e);
            return;
        }
    };
    
    println!("Trigger server started on http://{}", addr);
    while let Ok((mut stream, _)) = listener.accept().await {
        let app_handle = app.clone();
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            if let Ok(n) = stream.read(&mut buf).await {
                if n > 0 {
                    let request = String::from_utf8_lossy(&buf[..n]);
                    // Accept GET /trigger-update or POST /trigger-update
                    if request.contains(" /trigger-update ") {
                        let _ = app_handle.emit("trigger-manual-update", ());
                        
                        let state = app_handle.state::<crate::updater::AppState>();
                        crate::updater::ws_server::broadcast_reload(&state.clients).await;
                        
                        let response = "HTTP/1.1 200 OK\r\n\
                                        Content-Type: application/json\r\n\
                                        Access-Control-Allow-Origin: *\r\n\
                                        \r\n\
                                        {\"status\":\"update_triggered\"}";
                        let _ = stream.write_all(response.as_bytes()).await;
                    } else {
                        let response = "HTTP/1.1 404 Not Found\r\n\r\n404 Not Found";
                        let _ = stream.write_all(response.as_bytes()).await;
                    }
                }
            }
        });
    }
}
