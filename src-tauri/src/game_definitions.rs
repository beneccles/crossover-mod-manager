use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDefinition {
    pub id: String,
    pub name: String,
    pub nexus_domain: String,
    pub detection_files: Vec<String>,
    pub mod_directories: Vec<String>,
    pub supports_load_order: bool,
}

impl GameDefinition {
    pub fn cyberpunk2077() -> Self {
        Self {
            id: "cyberpunk2077".to_string(),
            name: "Cyberpunk 2077".to_string(),
            nexus_domain: "cyberpunk2077".to_string(),
            detection_files: vec![
                "bin/x64/Cyberpunk2077.exe".to_string(),
                "Cyberpunk2077.exe".to_string(),
                "engine/config/base/engine.ini".to_string(),
            ],
            mod_directories: vec![
                "archive/pc/mod".to_string(),
                "bin/x64/plugins".to_string(),
                "r6/scripts".to_string(),
            ],
            supports_load_order: true,
        }
    }

    pub fn skyrim() -> Self {
        Self {
            id: "skyrim".to_string(),
            name: "Skyrim".to_string(),
            nexus_domain: "skyrim".to_string(),
            detection_files: vec![
                "TESV.exe".to_string(),
                "SkyrimLauncher.exe".to_string(),
            ],
            mod_directories: vec![
                "Data".to_string(),
            ],
            supports_load_order: true,
        }
    }

    pub fn skyrim_se() -> Self {
        Self {
            id: "skyrimse".to_string(),
            name: "Skyrim Special Edition".to_string(),
            nexus_domain: "skyrimspecialedition".to_string(),
            detection_files: vec![
                "SkyrimSE.exe".to_string(),
                "SkyrimSELauncher.exe".to_string(),
            ],
            mod_directories: vec![
                "Data".to_string(),
            ],
            supports_load_order: true,
        }
    }
}

pub fn get_supported_games() -> HashMap<String, GameDefinition> {
    let mut games = HashMap::new();
    
    let cp2077 = GameDefinition::cyberpunk2077();
    games.insert(cp2077.id.clone(), cp2077);
    
    let skyrim = GameDefinition::skyrim();
    games.insert(skyrim.id.clone(), skyrim);
    
    let skyrim_se = GameDefinition::skyrim_se();
    games.insert(skyrim_se.id.clone(), skyrim_se);
    
    games
}

pub fn detect_game_from_path(path: &Path) -> Option<GameDefinition> {
    if !path.exists() || !path.is_dir() {
        return None;
    }

    let games = get_supported_games();
    
    for (_, game) in games {
        for detection_file in &game.detection_files {
            if path.join(detection_file).exists() {
                return Some(game);
            }
        }
    }
    
    None
}

pub fn get_game_by_id(game_id: &str) -> Option<GameDefinition> {
    get_supported_games().get(game_id).cloned()
}
