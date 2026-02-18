use tauri::{AppHandle, Manager};
use std::fs;
use crate::updater::{logic, ws_server, AppState};

#[tauri::command]
pub async fn get_install_path(app: AppHandle) -> Result<String, String> {
    let app_dir = app.path().local_data_dir()
        .map_err(|e| e.to_string())?
        .join("LeClasseurExtension");
    
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    }
    
    Ok(app_dir.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn open_chrome_extensions() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(&["/C", "start", "chrome"])
            .spawn()
            .map_err(|e| e.to_string())?;
        Ok(())
    }
    #[cfg(not(target_os = "windows"))]
    {
        open::that("chrome://extensions/").map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub async fn check_and_update(app: AppHandle, state: tauri::State<'_, AppState>, manifest_url: &str) -> Result<bool, String> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| e.to_string())?;
        
    let res = client.get(manifest_url).send().await.map_err(|e| e.to_string())?;
    let manifest: logic::UpdateManifest = res.json().await.map_err(|e| e.to_string())?;
    
    let app_dir = app.path().local_data_dir().map_err(|e| e.to_string())?.join("LeClasseurExtension");
    let current_version = logic::get_local_version(&app_dir);
    
    if manifest.version != current_version {
        logic::download_and_install(app_dir, manifest).await?;
        ws_server::broadcast_reload(&state.clients).await;
        Ok(true)
    } else {
        Ok(false)
    }
}

#[tauri::command]
pub async fn trigger_manual_reload(state: tauri::State<'_, AppState>) -> Result<(), String> {
    ws_server::broadcast_reload(&state.clients).await;
    Ok(())
}

#[tauri::command]
pub async fn quit_app(app: AppHandle) {
    app.exit(0);
}
