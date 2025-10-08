// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod mod_manager;
mod settings;

use mod_manager::{ModManager, ModInfo};
use settings::{Settings, AppSettings};
use std::sync::Mutex;
use tauri::State;

struct AppState {
    mod_manager: Mutex<ModManager>,
    settings: Mutex<AppSettings>,
}

#[tauri::command]
fn get_installed_mods(state: State<AppState>) -> Result<Vec<ModInfo>, String> {
    let manager = state.mod_manager.lock().map_err(|e| e.to_string())?;
    Ok(manager.get_installed_mods())
}

#[tauri::command]
async fn install_mod(mod_data: serde_json::Value, state: State<'_, AppState>) -> Result<(), String> {
    let mut manager = state.mod_manager.lock().map_err(|e| e.to_string())?;
    let settings = state.settings.lock().map_err(|e| e.to_string())?;
    
    manager.install_mod(mod_data, &settings.get_settings()).await
}

#[tauri::command]
fn remove_mod(mod_id: String, state: State<AppState>) -> Result<(), String> {
    let mut manager = state.mod_manager.lock().map_err(|e| e.to_string())?;
    manager.remove_mod(&mod_id)
}

#[tauri::command]
fn get_settings(state: State<AppState>) -> Result<Settings, String> {
    let settings = state.settings.lock().map_err(|e| e.to_string())?;
    Ok(settings.get_settings())
}

#[tauri::command]
fn save_settings(settings: Settings, state: State<AppState>) -> Result<(), String> {
    let mut app_settings = state.settings.lock().map_err(|e| e.to_string())?;
    app_settings.save_settings(settings)
}

fn main() {
    let mod_manager = ModManager::new();
    let app_settings = AppSettings::new();

    tauri::Builder::default()
        .manage(AppState {
            mod_manager: Mutex::new(mod_manager),
            settings: Mutex::new(app_settings),
        })
        .invoke_handler(tauri::generate_handler![
            get_installed_mods,
            install_mod,
            remove_mod,
            get_settings,
            save_settings
        ])
        .register_uri_scheme_protocol("nxm", |_app, request| {
            // Handle NexusMods protocol
            let url = request.uri();
            println!("Received NexusMods URL: {}", url);
            
            // Parse the nxm:// URL and queue the download
            // Format: nxm://cyberpunk2077/mods/{mod_id}/files/{file_id}
            
            tauri::http::ResponseBuilder::new()
                .status(200)
                .body(Vec::new())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
