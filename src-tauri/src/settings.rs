use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub game_path: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            game_path: String::new(),
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
                if let Ok(settings) = serde_json::from_str::<Settings>(&content) {
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
