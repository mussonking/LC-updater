pub mod updater;

use crate::updater::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = AppState::new();
    let clients_clone = Arc::clone(&state.clients);
    
    // Start WS server in background
    tokio::spawn(async move {
        updater::ws_server::start_ws_server(clients_clone).await;
    });

    tauri::Builder::default()
        .manage(state)
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![
            updater::commands::get_install_path, 
            updater::commands::open_chrome_extensions, 
            updater::commands::check_and_update,
            updater::commands::trigger_manual_reload
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use std::sync::Arc;
