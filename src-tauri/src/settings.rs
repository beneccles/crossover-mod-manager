use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub game_path: String,
    pub game_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    // Legacy fields for migration
    #[serde(default)]
    pub game_path: String,
    #[serde(default)]
    pub mod_storage_path: String,
    
    // New multi-game fields
    #[serde(default)]
    pub current_game: String,
    #[serde(default)]
    pub games: HashMap<String, GameConfig>,
    
    #[serde(default = "default_first_run")]
    pub first_run: bool,
    #[serde(default)]
    pub nexusmods_api_key: String,
}

fn default_first_run() -> bool {
    true
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            game_path: String::new(),
            mod_storage_path: String::new(),
            current_game: String::new(),
            games: HashMap::new(),
            first_run: true,
            nexusmods_api_key: String::new(),
        }
    }
}

pub struct AppSettings {
    settings_path: PathBuf,
    settings: Settings,
}

impl AppSettings {
    pub fn new() -> Self {
        let settings_path = Self::get_settings_path();
        let settings = Self::load_settings(&settings_path);

        Self {
            settings_path,
            settings,
        }
    }

    fn get_settings_path() -> PathBuf {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let app_dir = home.join(".crossover-mod-manager");

        if !app_dir.exists() {
            fs::create_dir_all(&app_dir).ok();
        }

        app_dir.join("settings.json")
    }

    fn load_settings(path: &PathBuf) -> Settings {
        if path.exists() {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(mut settings) = serde_json::from_str::<Settings>(&content) {
                    // Migrate old settings format to new multi-game format
                    if !settings.game_path.is_empty() && settings.games.is_empty() {
                        let game_config = GameConfig {
                            game_path: settings.game_path.clone(),
                            game_id: "cyberpunk2077".to_string(),
                        };
                        settings.games.insert("cyberpunk2077".to_string(), game_config);
                        settings.current_game = "cyberpunk2077".to_string();
                        settings.game_path = String::new(); // Clear legacy field
                    }
                    
                    // Ensure current_game is set if games exist
                    if settings.current_game.is_empty() && !settings.games.is_empty() {
                        if let Some(first_game_id) = settings.games.keys().next() {
                            settings.current_game = first_game_id.clone();
                        }
                    }
                    
                    return settings;
                }
            }
        }
        Settings::default()
    }

    pub fn get_settings(&self) -> Settings {
        self.settings.clone()
    }

    pub fn save_settings(&mut self, settings: Settings) -> Result<(), String> {
        let json = serde_json::to_string_pretty(&settings)
            .map_err(|e| format!("Failed to serialize settings: {}", e))?;

        fs::write(&self.settings_path, json)
            .map_err(|e| format!("Failed to write settings: {}", e))?;

        self.settings = settings;
        Ok(())
    }
}
