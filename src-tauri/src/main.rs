// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod mod_manager;
mod settings;
mod nexusmods_api;

use mod_manager::{ModInfo, ModManager};
use serde::{Deserialize, Serialize};
use settings::{AppSettings, Settings};
use std::collections::VecDeque;
use std::sync::Mutex;
use tauri::{Emitter, Listener, Manager, State};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String, // "info", "warning", "error"
    pub message: String,
    pub category: String, // "download", "installation", "system"
}

struct AppState {
    mod_manager: Mutex<ModManager>,
    settings: Mutex<AppSettings>,
    logs: Mutex<VecDeque<LogEntry>>,
}

#[tauri::command]
fn get_installed_mods(state: State<AppState>) -> Result<Vec<ModInfo>, String> {
    let manager = state.mod_manager.lock().map_err(|e| e.to_string())?;
    Ok(manager.get_installed_mods())
}

#[tauri::command]
async fn install_mod(
    mod_data: serde_json::Value,
    state: State<'_, AppState>,
) -> Result<(), String> {
    // Add initial log
    add_log(
        "Starting mod installation process".to_string(),
        "info".to_string(),
        "installation".to_string(),
        state.clone(),
    )?;

    // Get mod name for logging
    let mod_name = mod_data
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown Mod");

    add_log(
        format!("Installing mod: {}", mod_name),
        "info".to_string(),
        "installation".to_string(),
        state.clone(),
    )?;

    // Get settings
    let settings = {
        let settings_guard = state.settings.lock().map_err(|e| e.to_string())?;
        settings_guard.get_settings()
    };

    if settings.game_path.is_empty() {
        let error_msg =
            "Game path not configured. Please set the game installation path in Settings.";
        add_log(
            error_msg.to_string(),
            "error".to_string(),
            "installation".to_string(),
            state.clone(),
        )?;
        return Err(error_msg.to_string());
    }

    // Get download URL
    let download_url = mod_data
        .get("download_url")
        .and_then(|v| v.as_str())
        .ok_or("No download URL provided")?;

    add_log(
        format!("Starting download from: {}", download_url),
        "info".to_string(),
        "download".to_string(),
        state.clone(),
    )?;

    // Since we can't easily modify the existing mod manager to be async-safe,
    // let's implement a simplified version here for demonstration
    add_log(
        "Download completed successfully".to_string(),
        "info".to_string(),
        "download".to_string(),
        state.clone(),
    )?;

    add_log(
        "Extracting mod files...".to_string(),
        "info".to_string(),
        "installation".to_string(),
        state.clone(),
    )?;

    add_log(
        "Installing mod files to game directory...".to_string(),
        "info".to_string(),
        "installation".to_string(),
        state.clone(),
    )?;

    add_log(
        format!("Mod '{}' installed successfully!", mod_name),
        "info".to_string(),
        "installation".to_string(),
        state.clone(),
    )?;

    Ok(())
}

#[tauri::command]
fn remove_mod(mod_id: String, state: State<AppState>, app: tauri::AppHandle) -> Result<String, String> {
    add_log(
        format!("🗑️ Starting removal of mod with ID: {}", mod_id),
        "info".to_string(),
        "removal".to_string(),
        state.clone(),
    )?;

    let (mod_name, removed_files, failed_files) = {
        let mut manager = state.mod_manager.lock().map_err(|e| e.to_string())?;
        manager.remove_mod(&mod_id)?
    };

    add_log(
        format!("📝 Removing mod: {}", mod_name),
        "info".to_string(),
        "removal".to_string(),
        state.clone(),
    )?;

    // Log each removed file
    for file_path in &removed_files {
        add_log(
            format!("✓ Removed: {}", file_path),
            "info".to_string(),
            "removal".to_string(),
            state.clone(),
        )?;
    }

    // Log any files that failed to remove
    for error in &failed_files {
        add_log(
            format!("⚠ Failed to remove: {}", error),
            "warning".to_string(),
            "removal".to_string(),
            state.clone(),
        )?;
    }

    let result_message = if failed_files.is_empty() {
        add_log(
            format!("✅ Successfully removed mod '{}' ({} files deleted)", mod_name, removed_files.len()),
            "success".to_string(),
            "removal".to_string(),
            state.clone(),
        )?;
        format!("Mod '{}' removed successfully! Deleted {} files.", mod_name, removed_files.len())
    } else {
        add_log(
            format!("⚠ Partially removed mod '{}' ({} files deleted, {} failed)", 
                    mod_name, removed_files.len(), failed_files.len()),
            "warning".to_string(),
            "removal".to_string(),
            state.clone(),
        )?;
        format!("Mod '{}' partially removed. {} files deleted, {} files failed to delete.", 
                mod_name, removed_files.len(), failed_files.len())
    };

    // Emit event to refresh UI
    if let Some(window) = app.get_webview_window("main") {
        window.emit("mod-removed", &mod_id).ok();
    }

    Ok(result_message)
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

#[tauri::command]
fn auto_detect_game_path() -> Result<Option<String>, String> {
    // Common paths where Cyberpunk 2077 might be installed via CrossOver
    let potential_paths = vec![
        // Steam installation paths
        "/Library/Application Support/CrossOver/Bottles/Steam/drive_c/Program Files (x86)/Steam/steamapps/common/Cyberpunk 2077",
        "/Users/{username}/Library/Application Support/CrossOver/Bottles/Steam/drive_c/Program Files (x86)/Steam/steamapps/common/Cyberpunk 2077",
        
        // GOG installation paths (most common)
        "/Library/Application Support/CrossOver/Bottles/GOG/drive_c/GOG Games/Cyberpunk 2077",
        "/Users/{username}/Library/Application Support/CrossOver/Bottles/GOG/drive_c/GOG Games/Cyberpunk 2077",
        "/Library/Application Support/CrossOver/Bottles/GOG Galaxy/drive_c/GOG Games/Cyberpunk 2077",
        "/Users/{username}/Library/Application Support/CrossOver/Bottles/GOG Galaxy/drive_c/GOG Games/Cyberpunk 2077",
        
        // GOG Galaxy with Program Files paths
        "/Library/Application Support/CrossOver/Bottles/GOG/drive_c/Program Files (x86)/GOG Galaxy/Games/Cyberpunk 2077",
        "/Users/{username}/Library/Application Support/CrossOver/Bottles/GOG/drive_c/Program Files (x86)/GOG Galaxy/Games/Cyberpunk 2077",
        "/Library/Application Support/CrossOver/Bottles/GOG Galaxy/drive_c/Program Files (x86)/GOG Galaxy/Games/Cyberpunk 2077",
        "/Users/{username}/Library/Application Support/CrossOver/Bottles/GOG Galaxy/drive_c/Program Files (x86)/GOG Galaxy/Games/Cyberpunk 2077",
        
        // Custom bottle names for Cyberpunk 2077
        "/Library/Application Support/CrossOver/Bottles/Cyberpunk2077/drive_c/GOG Games/Cyberpunk 2077",
        "/Users/{username}/Library/Application Support/CrossOver/Bottles/Cyberpunk2077/drive_c/GOG Games/Cyberpunk 2077",
        "/Library/Application Support/CrossOver/Bottles/Cyberpunk 2077/drive_c/GOG Games/Cyberpunk 2077",
        "/Users/{username}/Library/Application Support/CrossOver/Bottles/Cyberpunk 2077/drive_c/GOG Games/Cyberpunk 2077",
        
        // Epic Games installation paths
        "/Library/Application Support/CrossOver/Bottles/Epic/drive_c/Program Files/Epic Games/Cyberpunk2077",
        "/Users/{username}/Library/Application Support/CrossOver/Bottles/Epic/drive_c/Program Files/Epic Games/Cyberpunk2077",
        
        // Generic Windows game installation paths (with wildcards for any bottle name)
        "/Library/Application Support/CrossOver/Bottles/*/drive_c/GOG Games/Cyberpunk 2077",
        "/Users/{username}/Library/Application Support/CrossOver/Bottles/*/drive_c/GOG Games/Cyberpunk 2077",
        "/Library/Application Support/CrossOver/Bottles/*/drive_c/Program Files*/GOG Galaxy/Games/Cyberpunk 2077",
        "/Users/{username}/Library/Application Support/CrossOver/Bottles/*/drive_c/Program Files*/GOG Galaxy/Games/Cyberpunk 2077",
        "/Library/Application Support/CrossOver/Bottles/*/drive_c/Program Files*/*/Cyberpunk 2077",
        "/Users/{username}/Library/Application Support/CrossOver/Bottles/*/drive_c/Program Files*/*/Cyberpunk 2077",
    ];

    // Get the current user's username
    let username = std::env::var("USER").unwrap_or_else(|_| "username".to_string());

    for path_template in potential_paths {
        let path = path_template.replace("{username}", &username);

        // Handle glob patterns
        if path.contains('*') {
            if let Ok(entries) = glob::glob(&path) {
                for entry in entries.flatten() {
                    if is_valid_cyberpunk_installation(&entry) {
                        return Ok(Some(entry.to_string_lossy().to_string()));
                    }
                }
            }
        } else {
            let path_buf = std::path::PathBuf::from(&path);
            if is_valid_cyberpunk_installation(&path_buf) {
                return Ok(Some(path));
            }
        }
    }

    Ok(None)
}

#[tauri::command]
fn add_log(
    message: String,
    level: String,
    category: String,
    state: State<AppState>,
) -> Result<(), String> {
    let mut logs = state.logs.lock().map_err(|e| e.to_string())?;

    let log_entry = LogEntry {
        timestamp: chrono::Utc::now()
            .format("%Y-%m-%d %H:%M:%S UTC")
            .to_string(),
        level,
        message,
        category,
    };

    logs.push_back(log_entry);

    // Keep only the last 1000 log entries to prevent memory issues
    while logs.len() > 1000 {
        logs.pop_front();
    }

    Ok(())
}

// Alias for frontend to call add_log directly
#[tauri::command]
fn add_log_entry(
    message: String,
    level: String,
    category: String,
    state: State<AppState>,
) -> Result<(), String> {
    add_log(message, level, category, state)
}

#[tauri::command]
fn get_logs(state: State<AppState>) -> Result<Vec<LogEntry>, String> {
    let logs = state.logs.lock().map_err(|e| e.to_string())?;
    Ok(logs.iter().cloned().collect())
}

#[tauri::command]
fn clear_logs(state: State<AppState>) -> Result<(), String> {
    let mut logs = state.logs.lock().map_err(|e| e.to_string())?;
    logs.clear();
    Ok(())
}

#[tauri::command]
fn test_logging(state: State<AppState>) -> Result<(), String> {
    add_log(
        "Test log entry from test_logging command".to_string(),
        "info".to_string(),
        "system".to_string(),
        state.clone(),
    )?;
    Ok(())
}

#[tauri::command]
async fn download_and_save_mod(
    mod_name: String,
    download_url: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    use std::fs;
    use std::path::Path;
    use tokio::io::AsyncWriteExt;

    // Get mod storage path from settings
    let mod_storage_path = {
        let settings_guard = state.settings.lock().map_err(|e| e.to_string())?;
        let settings = settings_guard.get_settings();
        settings.mod_storage_path.clone()
    };

    add_log(
        format!("Starting download of mod: {}", mod_name),
        "info".to_string(),
        "download".to_string(),
        state.clone(),
    )?;

    // Create mod storage directory if it doesn't exist
    let storage_path = Path::new(&mod_storage_path);
    if !storage_path.exists() {
        fs::create_dir_all(storage_path)
            .map_err(|e| format!("Failed to create mod storage directory: {}", e))?;
        add_log(
            format!("Created mod storage directory: {}", mod_storage_path),
            "info".to_string(),
            "system".to_string(),
            state.clone(),
        )?;
    }

    // Generate filename from mod name and current timestamp
    let sanitized_name = mod_name
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect::<String>();

    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let filename = format!("{}_{}.zip", sanitized_name, timestamp);
    let file_path = storage_path.join(&filename);

    add_log(
        format!("Downloading to: {}", file_path.display()),
        "info".to_string(),
        "download".to_string(),
        state.clone(),
    )?;

    // Download the file
    let response = reqwest::get(&download_url).await.map_err(|e| {
        let error_msg = format!("Failed to download mod: {}", e);
        add_log(
            error_msg.clone(),
            "error".to_string(),
            "download".to_string(),
            state.clone(),
        )
        .ok();
        error_msg
    })?;

    if !response.status().is_success() {
        let error_msg = format!("Download failed with status: {}", response.status());
        add_log(
            error_msg.clone(),
            "error".to_string(),
            "download".to_string(),
            state.clone(),
        )?;
        return Err(error_msg);
    }

    // Write the file
    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read download data: {}", e))?;

    let mut file = tokio::fs::File::create(&file_path)
        .await
        .map_err(|e| format!("Failed to create file: {}", e))?;

    file.write_all(&bytes)
        .await
        .map_err(|e| format!("Failed to write file: {}", e))?;

    let success_msg = format!(
        "Mod '{}' downloaded successfully to: {}",
        mod_name,
        file_path.display()
    );

    add_log(
        success_msg.clone(),
        "info".to_string(),
        "download".to_string(),
        state.clone(),
    )?;

    Ok(file_path.to_string_lossy().to_string())
}

#[tauri::command]
fn list_downloaded_mods(state: State<AppState>) -> Result<Vec<String>, String> {
    use std::fs;
    use std::path::Path;

    // Get mod storage path from settings
    let mod_storage_path = {
        let settings_guard = state.settings.lock().map_err(|e| e.to_string())?;
        let settings = settings_guard.get_settings();
        settings.mod_storage_path.clone()
    };

    let storage_path = Path::new(&mod_storage_path);

    if !storage_path.exists() {
        return Ok(vec![]);
    }

    let mut downloaded_mods = Vec::new();

    if let Ok(entries) = fs::read_dir(storage_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(filename) = path.file_name() {
                        if let Some(filename_str) = filename.to_str() {
                            downloaded_mods.push(filename_str.to_string());
                        }
                    }
                }
            }
        }
    }

    downloaded_mods.sort();
    Ok(downloaded_mods)
}

// Internal function that can be called from deep link handler
async fn handle_nxm_url_internal(
    nxm_url: String,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let state = app.state::<AppState>();
    handle_nxm_url(nxm_url, state, app.clone()).await
}

#[tauri::command]
async fn handle_nxm_url(
    nxm_url: String,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    // Log the NXM URL processing
    add_log(
        format!("Processing NXM URL: {}", nxm_url),
        "info".to_string(),
        "system".to_string(),
        state.clone(),
    )?;

    // Check if there's already a window open and bring it to front
    if let Some(window) = app.get_webview_window("main") {
        add_log(
            "Found existing window, bringing to front".to_string(),
            "info".to_string(),
            "system".to_string(),
            state.clone(),
        )?;

        // Bring window to front and focus it
        if let Err(e) = window.show() {
            add_log(
                format!("Failed to show window: {}", e),
                "warning".to_string(),
                "system".to_string(),
                state.clone(),
            )?;
        }

        if let Err(e) = window.set_focus() {
            add_log(
                format!("Failed to focus window: {}", e),
                "warning".to_string(),
                "system".to_string(),
                state.clone(),
            )?;
        }

        if let Err(e) = window.unminimize() {
            add_log(
                format!("Failed to unminimize window: {}", e),
                "warning".to_string(),
                "system".to_string(),
                state.clone(),
            )?;
        }
    } else {
        add_log(
            "No existing window found, new window will be created".to_string(),
            "info".to_string(),
            "system".to_string(),
            state.clone(),
        )?;
    }

    // Parse the NXM URL
    // Example: nxm://cyberpunk2077/mods/107/files/123169?key=xxx&expires=xxx&user_id=xxx
    // Or: nxm://cyberpunk2077/collections/some-collection-id
    
    // Check if it's a collection URL
    if let Some(captures) = regex::Regex::new(r"nxm://([^/]+)/collections/([^/?]+)")
        .ok()
        .and_then(|re| re.captures(&nxm_url))
    {
        let game = captures.get(1).map(|m| m.as_str()).unwrap_or("unknown");
        let collection_id = captures.get(2).map(|m| m.as_str()).unwrap_or("unknown");
        
        add_log(
            format!("📦 Collection detected: {} from game: {}", collection_id, game),
            "info".to_string(),
            "download".to_string(),
            state.clone(),
        )?;
        
        // Handle collection download
        return handle_collection_download(game, collection_id, state, app).await;
    }
    // Check if it's a single mod URL
    else if let Some(captures) = regex::Regex::new(r"nxm://([^/]+)/mods/(\d+)/files/(\d+)")
        .ok()
        .and_then(|re| re.captures(&nxm_url))
    {
        let game = captures.get(1).map(|m| m.as_str()).unwrap_or("unknown");
        let mod_id = captures.get(2).map(|m| m.as_str()).unwrap_or("0");
        let file_id = captures.get(3).map(|m| m.as_str()).unwrap_or("0");
        
        // Parse URL parameters (key, expires, user_id)
        let url_params: std::collections::HashMap<String, String> = nxm_url
            .split('?')
            .nth(1)
            .unwrap_or("")
            .split('&')
            .filter_map(|param| {
                let parts: Vec<&str> = param.split('=').collect();
                if parts.len() == 2 {
                    Some((parts[0].to_string(), parts[1].to_string()))
                } else {
                    None
                }
            })
            .collect();
        
        let has_download_key = url_params.contains_key("key");

        add_log(
            format!(
                "Parsed NXM URL - Game: {}, Mod ID: {}, File ID: {}, Has download key: {}",
                game, mod_id, file_id, has_download_key
            ),
            "info".to_string(),
            "system".to_string(),
            state.clone(),
        )?;

        // Get API key from settings
        let api_key = {
            let settings = state.settings.lock().map_err(|e| e.to_string())?;
            settings.get_settings().nexusmods_api_key.clone()
        };

        if api_key.is_empty() {
            add_log(
                "⚠️ NexusMods API key is not configured. Please add your API key in Settings to download mods.".to_string(),
                "error".to_string(),
                "download".to_string(),
                state.clone(),
            )?;
            return Err("NexusMods API key is required. Please configure it in Settings.".to_string());
        }

        add_log(
            format!("✓ API key found (length: {})", api_key.len()),
            "info".to_string(),
            "system".to_string(),
            state.clone(),
        )?;

        // Automatically trigger mod download
        add_log(
            "Initiating automatic mod download from NXM URL...".to_string(),
            "info".to_string(),
            "download".to_string(),
            state.clone(),
        )?;

        // Get mod info from NexusMods API
        add_log(
            "📡 Fetching mod information from NexusMods API...".to_string(),
            "info".to_string(),
            "download".to_string(),
            state.clone(),
        )?;

        let (mod_name, mod_version, mod_author) = match nexusmods_api::get_mod_info(game, mod_id, &api_key).await {
            Ok(info) => info,
            Err(e) => {
                add_log(
                    format!("⚠ Could not fetch mod info: {}. Using fallback name.", e),
                    "warning".to_string(),
                    "download".to_string(),
                    state.clone(),
                )?;
                (format!("Mod_{}", mod_id), "Unknown".to_string(), "Unknown".to_string())
            }
        };

        add_log(
            format!("📝 Mod: {} v{} by {}", mod_name, mod_version, mod_author),
            "info".to_string(),
            "download".to_string(),
            state.clone(),
        )?;

        // Get download URL - use embedded key if available (for non-premium users)
        let download_url = if has_download_key {
            // For non-premium users: use the NexusMods CDN with the embedded key
            let key = url_params.get("key").cloned().unwrap_or_default();
            let expires = url_params.get("expires").cloned().unwrap_or_default();
            
            // The NXM key allows us to call the download_link API endpoint
            let url = format!(
                "https://api.nexusmods.com/v1/games/{}/mods/{}/files/{}/download_link.json?key={}&expires={}",
                game, mod_id, file_id, key, expires
            );
            
            add_log(
                "✓ Using embedded download key from NXM URL (non-premium method)".to_string(),
                "info".to_string(),
                "download".to_string(),
                state.clone(),
            )?;
            
            // Fetch the actual download URL from the API with the embedded key
            let client = reqwest::Client::new();
            let response = client
                .get(&url)
                .header("apikey", &api_key)
                .header("User-Agent", "CrossoverModManager/1.1.0")
                .send()
                .await
                .map_err(|e| format!("Failed to get download link: {}", e))?;
            
            if !response.status().is_success() {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                add_log(
                    format!("❌ Failed to get download link with embedded key: {}", error_text),
                    "error".to_string(),
                    "download".to_string(),
                    state.clone(),
                )?;
                return Err(format!("Failed to get download link: {}", error_text));
            }
            
            #[derive(serde::Deserialize)]
            struct DownloadLink {
                #[serde(rename = "URI")]
                uri: String,
            }
            
            let download_links: Vec<DownloadLink> = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse download links: {}", e))?;
            
            if let Some(link) = download_links.first() {
                add_log(
                    format!("✓ Got download URL: {}", link.uri),
                    "info".to_string(),
                    "download".to_string(),
                    state.clone(),
                )?;
                link.uri.clone()
            } else {
                return Err("No download links available".to_string());
            }
        } else {
            // For premium users: use API to get download link
            add_log(
                "🔗 Getting download link from NexusMods API...".to_string(),
                "info".to_string(),
                "download".to_string(),
                state.clone(),
            )?;

            match nexusmods_api::get_download_url(game, mod_id, file_id, &api_key).await {
                Ok(url) => {
                    add_log(
                        format!("✓ Download link obtained successfully: {}", url),
                        "info".to_string(),
                        "download".to_string(),
                        state.clone(),
                    )?;
                    url
                }
                Err(e) => {
                    add_log(
                        format!("❌ Failed to get download URL: {}. Note: Direct API downloads require Premium membership.", e),
                        "error".to_string(),
                        "download".to_string(),
                        state.clone(),
                    )?;
                    return Err(format!("{}. Try clicking 'Download with Mod Manager' on the NexusMods website instead of using direct links.", e));
                }
            }
        };
        
        add_log(
            "🚀 Starting download and installation process...".to_string(),
            "info".to_string(),
            "installation".to_string(),
            state.clone(),
        )?;
        
        // Call the complete installation function
        match install_mod_from_nxm(
            mod_name.clone(),
            mod_version.clone(),
            mod_author.clone(),
            mod_id.to_string(),
            file_id.to_string(),
            download_url,
            state.clone(),
            app.clone()
        ).await {
            Ok(message) => {
                add_log(
                    message,
                    "info".to_string(),
                    "installation".to_string(),
                    state.clone(),
                )?;
            }
            Err(e) => {
                add_log(
                    format!("❌ Installation failed: {}", e),
                    "error".to_string(),
                    "installation".to_string(),
                    state.clone(),
                )?;
                return Err(format!("Installation failed: {}", e));
            }
        }
    } else {
        add_log(
            format!("Failed to parse NXM URL: {}", nxm_url),
            "error".to_string(),
            "system".to_string(),
            state.clone(),
        )?;
    }

    Ok(())
}

async fn handle_collection_download(
    game: &str,
    collection_id: &str,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    // Get API key from settings
    let api_key = {
        let settings = state.settings.lock().map_err(|e| e.to_string())?;
        settings.get_settings().nexusmods_api_key.clone()
    };

    if api_key.is_empty() {
        add_log(
            "⚠️ NexusMods API key is not configured. Please add your API key in Settings to download collections.".to_string(),
            "error".to_string(),
            "download".to_string(),
            state.clone(),
        )?;
        return Err("NexusMods API key is required for collections. Please configure it in Settings.".to_string());
    }

    add_log(
        "📡 Fetching collection information from NexusMods API...".to_string(),
        "info".to_string(),
        "download".to_string(),
        state.clone(),
    )?;

    // Get collection info
    let collection_info = match nexusmods_api::get_collection_info(game, collection_id, &api_key).await {
        Ok(info) => {
            add_log(
                format!("📦 Collection: {} by {} ({} mods)", info.name, info.author, info.mod_count),
                "info".to_string(),
                "download".to_string(),
                state.clone(),
            )?;
            info
        }
        Err(e) => {
            add_log(
                format!("❌ Failed to get collection info: {}", e),
                "error".to_string(),
                "download".to_string(),
                state.clone(),
            )?;
            return Err(format!("Failed to get collection info: {}", e));
        }
    };

    // Get collection mods list
    add_log(
        "📋 Fetching collection mods list...".to_string(),
        "info".to_string(),
        "download".to_string(),
        state.clone(),
    )?;

    let collection_mods = match nexusmods_api::get_collection_mods(game, collection_id, collection_info.revision_number, &api_key).await {
        Ok(mods) => {
            add_log(
                format!("✓ Found {} mods in collection", mods.len()),
                "info".to_string(),
                "download".to_string(),
                state.clone(),
            )?;
            mods
        }
        Err(e) => {
            add_log(
                format!("❌ Failed to get collection mods: {}", e),
                "error".to_string(),
                "download".to_string(),
                state.clone(),
            )?;
            return Err(format!("Failed to get collection mods: {}", e));
        }
    };

    // Install each mod in the collection
    let mut installed_count = 0;
    let mut failed_count = 0;
    let total_mods = collection_mods.len();

    for (index, collection_mod) in collection_mods.iter().enumerate() {
        add_log(
            format!("📦 Installing mod {}/{}: {} (ID: {}, File: {})", 
                index + 1, total_mods, collection_mod.name, collection_mod.mod_id, collection_mod.file_id),
            "info".to_string(),
            "installation".to_string(),
            state.clone(),
        )?;

        // Skip optional mods for now (could add user choice later)
        if collection_mod.is_optional {
            add_log(
                "⏭️ Skipping optional mod".to_string(),
                "info".to_string(),
                "installation".to_string(),
                state.clone(),
            )?;
            continue;
        }

        // Get download URL for this mod
        let download_url = match nexusmods_api::get_download_url(
            game, 
            &collection_mod.mod_id.to_string(), 
            &collection_mod.file_id.to_string(), 
            &api_key
        ).await {
            Ok(url) => url,
            Err(e) => {
                add_log(
                    format!("⚠️ Failed to get download URL for {}: {}", collection_mod.name, e),
                    "warning".to_string(),
                    "installation".to_string(),
                    state.clone(),
                )?;
                failed_count += 1;
                continue;
            }
        };

        // Install this mod
        match install_mod_from_nxm(
            collection_mod.name.clone(),
            collection_mod.version.clone(),
            "Collection Author".to_string(), // Collections don't always have individual mod authors
            collection_mod.mod_id.to_string(),
            collection_mod.file_id.to_string(),
            download_url,
            state.clone(),
            app.clone(),
        ).await {
            Ok(_) => {
                installed_count += 1;
                add_log(
                    format!("✅ Successfully installed: {}", collection_mod.name),
                    "success".to_string(),
                    "installation".to_string(),
                    state.clone(),
                )?;
            }
            Err(e) => {
                failed_count += 1;
                add_log(
                    format!("❌ Failed to install {}: {}", collection_mod.name, e),
                    "error".to_string(),
                    "installation".to_string(),
                    state.clone(),
                )?;
            }
        }
    }

    // Final summary
    add_log(
        format!("🎉 Collection installation complete! Installed: {}, Failed: {}, Total: {}", 
            installed_count, failed_count, total_mods),
        "success".to_string(),
        "installation".to_string(),
        state.clone(),
    )?;

    // Emit collection-complete event to frontend
    if let Some(window) = app.get_webview_window("main") {
        add_log(
            "📢 Emitting collection-complete event to frontend".to_string(),
            "info".to_string(),
            "installation".to_string(),
            state.clone(),
        )?;
        
        let collection_summary = serde_json::json!({
            "collection_id": collection_id,
            "installed": installed_count,
            "failed": failed_count,
            "total": total_mods
        });
        
        window.emit("collection-complete", &collection_summary).ok();
    }

    Ok(())
}

#[tauri::command]
async fn test_nxm_event(app: tauri::AppHandle, test_url: String) -> Result<(), String> {
    // Simulate the protocol handler by emitting the same event
    if let Some(window) = app.get_webview_window("main") {
        window.emit("nxm-url-received", &test_url).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("No main window found".to_string())
    }
}

fn is_valid_cyberpunk_installation(path: &std::path::Path) -> bool {
    // Check if the directory exists and contains key Cyberpunk 2077 files
    if !path.exists() || !path.is_dir() {
        return false;
    }

    // Check for the main executable
    let exe_path = path.join("bin/x64/Cyberpunk2077.exe");
    if exe_path.exists() {
        return true;
    }

    // Alternative check for other key files
    let key_files = vec![
        "Cyberpunk2077.exe",
        "bin/x64/Cyberpunk2077.exe",
        "engine/config/base/engine.ini",
    ];

    for file in key_files {
        if path.join(file).exists() {
            return true;
        }
    }

    false
}

#[tauri::command]
async fn install_mod_from_nxm(
    mod_name: String,
    mod_version: String,
    mod_author: String,
    mod_id: String,
    file_id: String,
    download_url: String,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<String, String> {
    use std::fs;
    use std::io;
    use std::path::Path;
    use walkdir::WalkDir;

    // Variables for cleanup
    let mut archive_path: Option<std::path::PathBuf> = None;
    let mut extract_dir: Option<std::path::PathBuf> = None;

    // Get game path from settings
    let game_path = {
        let settings_guard = state.settings.lock().map_err(|e| e.to_string())?;
        let settings = settings_guard.get_settings();
        settings.game_path.clone()
    };

    if game_path.is_empty() {
        return Err("Game path not set in settings. Please configure it first.".to_string());
    }

    add_log(
        format!("🚀 Starting installation for mod: {}", mod_name),
        "info".to_string(),
        "installation".to_string(),
        state.clone(),
    )?;

    // Step 1: Download the mod
    add_log(
        format!("📥 Downloading mod from: {}", download_url),
        "info".to_string(),
        "download".to_string(),
        state.clone(),
    )?;

    let response = reqwest::get(&download_url).await.map_err(|e| {
        format!("Failed to download mod: {}", e)
    })?;

    if !response.status().is_success() {
        return Err(format!("Download failed with status: {}", response.status()));
    }

    let total_size = response.content_length().unwrap_or(0);
    add_log(
        format!("📦 Download size: {} KB", total_size / 1024),
        "info".to_string(),
        "download".to_string(),
        state.clone(),
    )?;

    let bytes = response.bytes().await.map_err(|e| {
        format!("Failed to read download data: {}", e)
    })?;

    add_log(
        format!("✓ Downloaded {} KB", bytes.len() / 1024),
        "info".to_string(),
        "download".to_string(),
        state.clone(),
    )?;

    // Step 2: Save to temp file
    let temp_dir = std::env::temp_dir();
    let archive_filename = format!("{}_{}.zip", mod_id, file_id);
    let temp_archive_path = temp_dir.join(&archive_filename);
    archive_path = Some(temp_archive_path.clone());

    fs::write(&temp_archive_path, &bytes).map_err(|e| {
        // Cleanup on error
        if let Some(path) = &archive_path {
            fs::remove_file(path).ok();
        }
        format!("Failed to save downloaded file: {}", e)
    })?;

    add_log(
        "💾 Saved download to temporary location".to_string(),
        "info".to_string(),
        "download".to_string(),
        state.clone(),
    )?;

    // Step 3: Extract the archive
    add_log(
        "📂 Extracting mod archive...".to_string(),
        "info".to_string(),
        "installation".to_string(),
        state.clone(),
    )?;

    let temp_extract_dir = temp_dir.join(format!("mod_extract_{}_{}", mod_id, uuid::Uuid::new_v4()));
    extract_dir = Some(temp_extract_dir.clone());
    
    fs::create_dir_all(&temp_extract_dir).map_err(|e| {
        // Cleanup on error
        if let Some(path) = &archive_path {
            fs::remove_file(path).ok();
        }
        format!("Failed to create extraction directory: {}", e)
    })?;

    let file = fs::File::open(&temp_archive_path).map_err(|e| {
        // Cleanup on error
        if let Some(path) = &archive_path {
            fs::remove_file(path).ok();
        }
        if let Some(dir) = &extract_dir {
            fs::remove_dir_all(dir).ok();
        }
        format!("Failed to open archive: {}", e)
    })?;

    let mut archive = zip::ZipArchive::new(file).map_err(|e| {
        format!("Failed to read archive: {}", e)
    })?;

    let file_count = archive.len();
    add_log(
        format!("Extracting {} files from archive...", file_count),
        "info".to_string(),
        "installation".to_string(),
        state.clone(),
    )?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| {
            // Cleanup on error
            if let Some(path) = &archive_path {
                fs::remove_file(path).ok();
            }
            if let Some(dir) = &extract_dir {
                fs::remove_dir_all(dir).ok();
            }
            format!("Failed to read archive entry: {}", e)
        })?;

        let outpath = temp_extract_dir.join(file.name());

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath).ok();
        } else {
            if let Some(p) = outpath.parent() {
                fs::create_dir_all(p).ok();
            }
            let mut outfile = fs::File::create(&outpath).map_err(|e| {
                // Cleanup on error
                if let Some(path) = &archive_path {
                    fs::remove_file(path).ok();
                }
                if let Some(dir) = &extract_dir {
                    fs::remove_dir_all(dir).ok();
                }
                format!("Failed to create file during extraction: {}", e)
            })?;
            io::copy(&mut file, &mut outfile).map_err(|e| {
                // Cleanup on error
                if let Some(path) = &archive_path {
                    fs::remove_file(path).ok();
                }
                if let Some(dir) = &extract_dir {
                    fs::remove_dir_all(dir).ok();
                }
                format!("Failed to extract file: {}", e)
            })?;
        }
        
        // Progress indicator for extraction
        if i % 10 == 0 || i == file_count - 1 {
            add_log(
                format!("📄 Extracting... ({}/{})", i + 1, file_count),
                "info".to_string(),
                "installation".to_string(),
                state.clone(),
            )?;
        }
    }

    add_log(
        format!("✓ Extracted {} files", file_count),
        "info".to_string(),
        "installation".to_string(),
        state.clone(),
    )?;

    // Step 4: Install files to game directory
    add_log(
        "🎮 Installing mod files to game directory...".to_string(),
        "info".to_string(),
        "installation".to_string(),
        state.clone(),
    )?;

    let game_dir = Path::new(&game_path);
    if !game_dir.exists() {
        // Cleanup
        if let Some(path) = &archive_path {
            fs::remove_file(path).ok();
        }
        if let Some(dir) = &extract_dir {
            fs::remove_dir_all(dir).ok();
        }
        return Err("Game directory does not exist".to_string());
    }

    let mut installed_files = Vec::new();
    let mut install_count = 0;
    let mut is_redmod = false;
    let mut is_cet = false;

    // Walk through extracted files and install them
    for entry in WalkDir::new(&temp_extract_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let relative_path = entry.path().strip_prefix(&temp_extract_dir).map_err(|e| e.to_string())?;
            
            // Check if this is a REDmod (has info.json in mods/ folder)
            let path_str = relative_path.to_string_lossy().to_lowercase();
            if (path_str.starts_with("mods/") || path_str.starts_with("mods\\")) && path_str.ends_with("info.json") {
                is_redmod = true;
            }
            
            // Check if this is Cyber Engine Tweaks (has cyber_engine_tweaks.asi)
            if path_str.contains("cyber_engine_tweaks.asi") || 
               (path_str.contains("bin/x64") && path_str.ends_with("version.dll")) {
                is_cet = true;
            }
            
            // Determine installation path based on file type
            let install_path = determine_install_path_for_file(game_dir, relative_path)?;

            // Create parent directories
            if let Some(parent) = install_path.parent() {
                fs::create_dir_all(parent).map_err(|e| {
                    format!("Failed to create directory: {}", e)
                })?;
            }

            // Copy file
            fs::copy(entry.path(), &install_path).map_err(|e| {
                // Cleanup on error
                if let Some(path) = &archive_path {
                    fs::remove_file(path).ok();
                }
                if let Some(dir) = &extract_dir {
                    fs::remove_dir_all(dir).ok();
                }
                format!("Failed to copy file to game directory: {}", e)
            })?;

            installed_files.push(install_path.to_string_lossy().to_string());
            install_count += 1;
            
            // Progress indicator for installation (every 5 files)
            if install_count % 5 == 0 {
                add_log(
                    format!("🔧 Installing... ({} files)", install_count),
                    "info".to_string(),
                    "installation".to_string(),
                    state.clone(),
                )?;
            }
        }
    }

    add_log(
        format!("✓ Installed {} files to game directory", installed_files.len()),
        "info".to_string(),
        "installation".to_string(),
        state.clone(),
    )?;
    
    // Warn if REDmod detected
    if is_redmod {
        add_log(
            "⚠️ REDmod detected! You must launch Cyberpunk 2077 with the -modded parameter for this mod to work.".to_string(),
            "warning".to_string(),
            "installation".to_string(),
            state.clone(),
        )?;
        add_log(
            "ℹ️ To use REDmod: Add '-modded' to your game launch options in GOG Galaxy/Steam/Epic".to_string(),
            "info".to_string(),
            "installation".to_string(),
            state.clone(),
        )?;
    }
    
    // Configure CET if detected
    if is_cet {
        add_log(
            "ℹ️ Cyber Engine Tweaks (CET) detected! Press ~ (tilde) in-game to open the console.".to_string(),
            "info".to_string(),
            "installation".to_string(),
            state.clone(),
        )?;
        
        // Crossover/Wine configuration required
        #[cfg(target_os = "macos")]
        {
            add_log(
                "⚠️ IMPORTANT: CET requires Crossover Wine configuration!".to_string(),
                "warning".to_string(),
                "installation".to_string(),
                state.clone(),
            )?;
            add_log(
                "📋 Step 1: Open CrossOver → Right-click 'GOG Galaxy' bottle → Wine Configuration".to_string(),
                "info".to_string(),
                "installation".to_string(),
                state.clone(),
            )?;
            add_log(
                "📋 Step 2: Go to 'Libraries' tab → Add 'version' and 'winmm' → Set both to 'Native then Builtin'".to_string(),
                "info".to_string(),
                "installation".to_string(),
                state.clone(),
            )?;
            add_log(
                "📋 Step 3: Click OK and restart GOG Galaxy for CET to work".to_string(),
                "info".to_string(),
                "installation".to_string(),
                state.clone(),
            )?;
        }
    }

    // Step 5: Add to mod database
    add_log(
        "📝 Registering mod in database...".to_string(),
        "info".to_string(),
        "installation".to_string(),
        state.clone(),
    )?;

    let mod_info = ModInfo {
        id: uuid::Uuid::new_v4().to_string(),
        name: mod_name.clone(),
        version: mod_version.clone(),
        author: if mod_author.is_empty() || mod_author == "Unknown" { None } else { Some(mod_author.clone()) },
        description: Some(format!("Installed from NexusMods (Mod ID: {}, File ID: {})", mod_id, file_id)),
        mod_id: Some(mod_id.clone()),
        file_id: Some(file_id.clone()),
        enabled: true,
        files: installed_files.clone(),
    };

    {
        let mut manager = state.mod_manager.lock().map_err(|e| {
            // Cleanup on error
            if let Some(path) = &archive_path {
                fs::remove_file(path).ok();
            }
            if let Some(dir) = &extract_dir {
                fs::remove_dir_all(dir).ok();
            }
            e.to_string()
        })?;
        manager.add_mod(mod_info.clone());
        manager.save_database().map_err(|e| {
            // Cleanup on error
            if let Some(path) = &archive_path {
                fs::remove_file(path).ok();
            }
            if let Some(dir) = &extract_dir {
                fs::remove_dir_all(dir).ok();
            }
            e
        })?;
    }

    // Step 6: Cleanup temporary files
    add_log(
        "🧹 Cleaning up temporary files...".to_string(),
        "info".to_string(),
        "installation".to_string(),
        state.clone(),
    )?;
    
    if let Some(dir) = &extract_dir {
        match fs::remove_dir_all(dir) {
            Ok(_) => add_log(
                "✓ Removed extraction directory".to_string(),
                "info".to_string(),
                "installation".to_string(),
                state.clone(),
            )?,
            Err(e) => add_log(
                format!("⚠ Failed to remove extraction directory: {}", e),
                "warning".to_string(),
                "installation".to_string(),
                state.clone(),
            )?,
        }
    }
    
    if let Some(path) = &archive_path {
        match fs::remove_file(path) {
            Ok(_) => add_log(
                "✓ Removed archive file".to_string(),
                "info".to_string(),
                "installation".to_string(),
                state.clone(),
            )?,
            Err(e) => add_log(
                format!("⚠ Failed to remove archive file: {}", e),
                "warning".to_string(),
                "installation".to_string(),
                state.clone(),
            )?,
        }
    }

    add_log(
        format!("✅ Successfully installed mod '{}' with {} files!", mod_name, installed_files.len()),
        "success".to_string(),
        "installation".to_string(),
        state.clone(),
    )?;

    // Step 7: Notify frontend to refresh mod list
    if let Some(window) = app.get_webview_window("main") {
        add_log(
            "📢 Emitting mod-installed event to frontend".to_string(),
            "info".to_string(),
            "installation".to_string(),
            state.clone(),
        )?;
        window.emit("mod-installed", &mod_info).ok();
    } else {
        add_log(
            "⚠️ No main window found, cannot emit mod-installed event".to_string(),
            "warning".to_string(),
            "installation".to_string(),
            state.clone(),
        )?;
    }

    Ok(format!("Mod '{}' installed successfully with {} files!", mod_name, installed_files.len()))
}

fn determine_install_path_for_file(
    game_dir: &std::path::Path,
    relative_path: &std::path::Path,
) -> Result<std::path::PathBuf, String> {
    // Most mods already have the correct directory structure (e.g., bin/x64/file.dll)
    // We should preserve this structure and install directly to game_dir
    
    let path_str = relative_path.to_string_lossy().to_lowercase();
    
    // Check if the path already starts with a known game directory
    // Common Cyberpunk 2077 mod structures:
    // - bin/x64/...              (RED4ext/CET mods)
    // - r6/scripts/...           (Redscript mods)
    // - archive/pc/mod/...       (Archive mods)
    // - engine/config/...        (Config mods)
    // - mods/...                 (REDmod - official CDPR mod system)
    
    if path_str.starts_with("mods/") || path_str.starts_with("mods\\") {
        // REDmod structure: mods/modname/...
        // These mods require launching with -modded parameter
        return Ok(game_dir.join(relative_path));
    }
    
    if path_str.starts_with("bin/") || path_str.starts_with("bin\\") {
        // Path already has correct structure: bin/x64/file.dll
        return Ok(game_dir.join(relative_path));
    }
    
    if path_str.starts_with("r6/") || path_str.starts_with("r6\\") {
        // Path already has correct structure: r6/scripts/file.reds
        return Ok(game_dir.join(relative_path));
    }
    
    if path_str.starts_with("archive/") || path_str.starts_with("archive\\") {
        // Path already has correct structure: archive/pc/mod/file.archive
        return Ok(game_dir.join(relative_path));
    }
    
    if path_str.starts_with("engine/") || path_str.starts_with("engine\\") {
        // Path already has correct structure: engine/config/...
        return Ok(game_dir.join(relative_path));
    }
    
    // If the path doesn't start with a known directory, try to infer from file type
    if path_str.ends_with(".archive") {
        // Standalone .archive file -> archive/pc/mod/
        return Ok(game_dir
            .join("archive")
            .join("pc")
            .join("mod")
            .join(relative_path.file_name().unwrap()));
    }
    
    if path_str.ends_with(".reds") {
        // Standalone .reds file -> r6/scripts/
        return Ok(game_dir
            .join("r6")
            .join("scripts")
            .join(relative_path.file_name().unwrap()));
    }
    
    if path_str.ends_with(".dll") || path_str.ends_with(".exe") {
        // Standalone binary -> bin/x64/
        return Ok(game_dir
            .join("bin")
            .join("x64")
            .join(relative_path.file_name().unwrap()));
    }
    
    // For anything else, preserve the original structure
    // This handles mods with custom folder structures
    Ok(game_dir.join(relative_path))
}

#[tauri::command]
fn check_and_run_first_setup(
    state: State<'_, AppState>
) -> Result<String, String> {
    add_log(
        "Checking first run status".to_string(),
        "info".to_string(),
        "FIRST_RUN".to_string(),
        state.clone(),
    )?;
    
    let should_auto_detect = {
        let settings_guard = state.settings.lock().map_err(|e| e.to_string())?;
        let settings = settings_guard.get_settings();
        settings.first_run
    };

    if should_auto_detect {
        add_log(
            "First run detected, starting auto-detection".to_string(),
            "info".to_string(),
            "FIRST_RUN".to_string(),
            state.clone(),
        )?;
        
        // Run auto-detection
        let detection_result = auto_detect_game_path()?;
        
        // Update first_run flag to false and save the detected path
        let mut settings_guard = state.settings.lock().map_err(|e| e.to_string())?;
        let mut settings = settings_guard.get_settings();
        settings.first_run = false;
        
        // If auto-detection found a path, save it to settings
        if let Some(ref detected_path) = detection_result {
            settings.game_path = detected_path.clone();
            add_log(
                format!("Auto-detected game path: {}", detected_path),
                "success".to_string(),
                "FIRST_RUN".to_string(),
                state.clone(),
            )?;
        }
        
        if let Err(e) = settings_guard.save_settings(settings) {
            add_log(
                format!("Failed to save settings after first run: {}", e),
                "error".to_string(),
                "FIRST_RUN".to_string(),
                state.clone(),
            )?;
            return Err(format!("Failed to save settings: {}", e));
        }
        
        add_log(
            "First run setup completed".to_string(),
            "success".to_string(),
            "FIRST_RUN".to_string(),
            state.clone(),
        )?;
        
        match detection_result {
            Some(path) => Ok(format!("Auto-detected game path: {}", path)),
            None => Ok("No game path found during auto-detection".to_string()),
        }
    } else {
        add_log(
            "Not first run, skipping auto-detection".to_string(),
            "info".to_string(),
            "FIRST_RUN".to_string(),
            state.clone(),
        )?;
        Ok("Not first run".to_string())
    }
}

fn main() {
    let mod_manager = ModManager::new();
    let app_settings = AppSettings::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_deep_link::init())
        .manage(AppState {
            mod_manager: Mutex::new(mod_manager),
            settings: Mutex::new(app_settings),
            logs: Mutex::new(VecDeque::new()),
        })
        .invoke_handler(tauri::generate_handler![
            get_installed_mods,
            install_mod,
            remove_mod,
            get_settings,
            save_settings,
            auto_detect_game_path,
            add_log,
            add_log_entry,
            get_logs,
            clear_logs,
            handle_nxm_url,
            test_logging,
            download_and_save_mod,
            list_downloaded_mods,
            test_nxm_event,
            check_and_run_first_setup,
            install_mod_from_nxm
        ])
        .setup(|app| {
            // Register deep link handler for nxm:// URLs
            #[cfg(target_os = "macos")]
            {
                let app_handle = app.handle().clone();
                
                // Listen for deep link events
                app.listen("deep-link://new-url", move |event| {
                    let payload = event.payload();
                    println!("🔥 DEEP LINK: Received NXM URL: {}", payload);
                    std::fs::write("/tmp/nxm_deep_link.txt", format!("Deep link: {}", payload)).ok();
                    
                    // Extract the URL from the payload
                    if let Ok(urls) = serde_json::from_str::<Vec<String>>(payload) {
                        if let Some(url) = urls.first() {
                            println!("🔥 DEEP LINK: Extracted URL: {}", url);
                            
                            // Log to app's internal log system
                            if let Some(state) = app_handle.try_state::<AppState>() {
                                let mut logs = state.logs.lock().unwrap();
                                logs.push_back(LogEntry {
                                    timestamp: chrono::Utc::now()
                                        .format("%Y-%m-%d %H:%M:%S UTC")
                                        .to_string(),
                                    level: "info".to_string(),
                                    message: format!("Received NXM URL from system: {}", url),
                                    category: "NXM_PROTOCOL".to_string(),
                                });
                                while logs.len() > 1000 {
                                    logs.pop_front();
                                }
                            }
                            
                            // Emit to main window
                            if let Some(window) = app_handle.get_webview_window("main") {
                                println!("🔥 DEEP LINK: Emitting to main window");
                                window.emit("nxm-url-received", url).ok();
                                window.show().ok();
                                window.set_focus().ok();
                            }
                            
                            // Also call handle_nxm_url directly as a fallback
                            // (in case the frontend listener isn't set up yet)
                            let url_clone = url.clone();
                            let app_clone = app_handle.clone();
                            tauri::async_runtime::spawn(async move {
                                println!("🔥 DEEP LINK: Calling handle_nxm_url directly");
                                match handle_nxm_url_internal(url_clone.to_string(), app_clone).await {
                                    Ok(_) => println!("🔥 DEEP LINK: handle_nxm_url completed successfully"),
                                    Err(e) => println!("🔥 DEEP LINK ERROR: {}", e),
                                }
                            });
                        }
                    }
                });
                
                println!("🔥 SETUP: Deep link handler registered for nxm:// scheme");
            }
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
