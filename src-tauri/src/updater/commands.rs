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
    println!("[Updater] check_and_update triggered for URL: {}", manifest_url);

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| {
            println!("[Updater] Error building HTTP client: {}", e);
            e.to_string()
        })?;
        
    println!("[Updater] Fetching manifest...");
    let res = client.get(manifest_url).send().await.map_err(|e| {
        println!("[Updater] Error fetching manifest: {}", e);
        e.to_string()
    })?;
    
    println!("[Updater] Parsing manifest JSON...");
    let manifest: logic::UpdateManifest = res.json().await.map_err(|e| {
        println!("[Updater] Error parsing manifest JSON: {}", e);
        e.to_string()
    })?;
    println!("[Updater] Remote manifest version: {} (Download URL: {})", manifest.version, manifest.download_url);
    
    let app_dir = app.path().local_data_dir().map_err(|e| e.to_string())?.join("LeClasseurExtension");
    let current_version = logic::get_local_version(&app_dir);
    println!("[Updater] Local extension version: {}", current_version);
    
    let remote_ver = manifest.version.trim();
    let local_ver = current_version.trim();
    println!("[Updater] Comparing exactly: Remote='{}' vs Local='{}'", remote_ver, local_ver);
    
    if remote_ver != local_ver {
        println!("[Updater] Version mismatch detected! Starting download and install...");
        match logic::download_and_install(app_dir.clone(), manifest).await {
            Ok(_) => {
                println!("[Updater] Download and install successful! Broadcasting reload event.");
                ws_server::broadcast_reload(&state.clients).await;
                Ok(true)
            },
            Err(e) => {
                println!("[Updater] ERROR during download/install: {}", e);
                Err(e)
            }
        }
    } else {
        println!("[Updater] Versions match, no update needed.");
        Ok(false)
    }
}

#[tauri::command]
pub async fn get_local_version_command(app: AppHandle) -> Result<String, String> {
    let app_dir = app.path().local_data_dir()
        .map_err(|e| e.to_string())?
        .join("LeClasseurExtension");
    Ok(logic::get_local_version(&app_dir))
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
