use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub mod_id: Option<String>,
    pub file_id: Option<String>,
    pub enabled: bool,
    pub files: Vec<String>,
    
    // File ownership tracking for conflict detection
    // Map of relative file path -> conflict info
    #[serde(default)]
    pub file_conflicts: HashMap<String, FileConflictInfo>,
    
    // Install timestamp for determining which mod was installed first
    #[serde(default)]
    pub installed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileConflictInfo {
    // The mod ID that originally owned this file (if any)
    pub previous_owner: Option<String>,
    // The mod name for user-friendly display
    pub previous_owner_name: Option<String>,
    // Whether this is an archive file (important for load order)
    pub is_archive: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct ModDatabase {
    mods: Vec<ModInfo>,
}

pub struct ModManager {
    database_path: PathBuf,
    mods: Vec<ModInfo>,
}

impl ModManager {
    pub fn new() -> Self {
        let database_path = Self::get_database_path();
        let mods = Self::load_database(&database_path);

        Self {
            database_path,
            mods,
        }
    }

    fn get_database_path() -> PathBuf {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let app_dir = home.join(".crossover-mod-manager");

        if !app_dir.exists() {
            fs::create_dir_all(&app_dir).ok();
        }

        app_dir.join("mods.json")
    }

    fn load_database(path: &Path) -> Vec<ModInfo> {
        if path.exists() {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(db) = serde_json::from_str::<ModDatabase>(&content) {
                    return db.mods;
                }
            }
        }
        Vec::new()
    }

    pub fn save_database(&self) -> Result<(), String> {
        let db = ModDatabase {
            mods: self.mods.clone(),
        };

        let json = serde_json::to_string_pretty(&db)
            .map_err(|e| format!("Failed to serialize database: {}", e))?;

        fs::write(&self.database_path, json)
            .map_err(|e| format!("Failed to write database: {}", e))?;

        Ok(())
    }

    pub fn get_installed_mods(&self) -> Vec<ModInfo> {
        self.mods.clone()
    }

    pub fn add_mod(&mut self, mod_info: ModInfo) {
        self.mods.push(mod_info);
    }

    /// Check if a mod is already installed based on mod_id and file_id
    pub fn find_existing_mod(&self, mod_id: &str, file_id: &str) -> Option<&ModInfo> {
        self.mods.iter().find(|mod_info| {
            if let (Some(existing_mod_id), Some(existing_file_id)) =
                (&mod_info.mod_id, &mod_info.file_id)
            {
                existing_mod_id == mod_id && existing_file_id == file_id
            } else {
                false
            }
        })
    }

    /// Check if a mod with the same name and version is already installed
    pub fn find_existing_mod_by_name(&self, name: &str, version: &str) -> Option<&ModInfo> {
        self.mods
            .iter()
            .find(|mod_info| mod_info.name == name && mod_info.version == version)
    }

    /// Check if any version of a mod is already installed (by mod_id only)
    pub fn find_existing_mod_by_id(&self, mod_id: &str) -> Option<&ModInfo> {
        self.mods.iter().find(|mod_info| {
            if let Some(existing_mod_id) = &mod_info.mod_id {
                existing_mod_id == mod_id
            } else {
                false
            }
        })
    }

    #[allow(dead_code)]
    pub async fn install_mod(
        &mut self,
        mod_data: serde_json::Value,
        settings: &crate::settings::Settings,
    ) -> Result<(), String> {
        // Extract mod information from the data
        let name = mod_data
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown Mod")
            .to_string();

        let version = mod_data
            .get("version")
            .and_then(|v| v.as_str())
            .unwrap_or("1.0.0")
            .to_string();

        let download_url = mod_data
            .get("download_url")
            .and_then(|v| v.as_str())
            .ok_or("No download URL provided")?;

        // Download the mod file
        let mod_file = self.download_mod(download_url).await?;

        // Extract the archive
        let extracted_files = self.extract_mod(&mod_file, &settings.game_path)?;

        // Install files to game directory
        let installed_files = self.install_files(&extracted_files, &settings.game_path)?;

        // Create mod entry
        let mod_id = uuid::Uuid::new_v4().to_string();
        let mod_info = ModInfo {
            id: mod_id.clone(),
            name,
            version,
            author: mod_data
                .get("author")
                .and_then(|v| v.as_str())
                .map(String::from),
            description: mod_data
                .get("description")
                .and_then(|v| v.as_str())
                .map(String::from),
            mod_id: mod_data
                .get("mod_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            file_id: mod_data
                .get("file_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            enabled: true,
            files: installed_files,
            file_conflicts: HashMap::new(),
            installed_at: Some(chrono::Utc::now().to_rfc3339()),
        };

        self.mods.push(mod_info);
        self.save_database()?;

        // Clean up temporary files
        fs::remove_file(mod_file).ok();

        Ok(())
    }

    pub fn remove_mod(
        &mut self,
        mod_id: &str,
    ) -> Result<(String, Vec<String>, Vec<String>), String> {
        let mod_index = self
            .mods
            .iter()
            .position(|m| m.id == mod_id)
            .ok_or("Mod not found")?;

        let mod_info = self.mods[mod_index].clone();
        let mod_name = mod_info.name.clone();
        let _total_files = mod_info.files.len();

        let mut removed_files = Vec::new();
        let mut failed_files = Vec::new();

        // Remove all installed files
        for file_path in &mod_info.files {
            match fs::remove_file(file_path) {
                Ok(_) => {
                    removed_files.push(file_path.clone());
                }
                Err(e) => {
                    eprintln!("Failed to remove file {}: {}", file_path, e);
                    failed_files.push(format!("{}: {}", file_path, e));
                }
            }
        }

        // Remove the mod from the database
        self.mods.remove(mod_index);
        self.save_database()?;

        Ok((mod_name, removed_files, failed_files))
    }

    #[allow(dead_code)]
    async fn download_mod(&self, url: &str) -> Result<PathBuf, String> {
        let temp_dir = std::env::temp_dir();
        let filename = format!("mod_{}.zip", uuid::Uuid::new_v4());
        let file_path = temp_dir.join(filename);

        let response = reqwest::get(url)
            .await
            .map_err(|e| format!("Failed to download mod: {}", e))?;

        let bytes = response
            .bytes()
            .await
            .map_err(|e| format!("Failed to read download: {}", e))?;

        fs::write(&file_path, bytes).map_err(|e| format!("Failed to save download: {}", e))?;

        Ok(file_path)
    }

    #[allow(dead_code)]
    fn extract_mod(&self, archive_path: &Path, _game_path: &str) -> Result<PathBuf, String> {
        let temp_dir = std::env::temp_dir();
        let extract_dir = temp_dir.join(format!("mod_extract_{}", uuid::Uuid::new_v4()));

        fs::create_dir_all(&extract_dir)
            .map_err(|e| format!("Failed to create extraction directory: {}", e))?;

        let file =
            fs::File::open(archive_path).map_err(|e| format!("Failed to open archive: {}", e))?;

        let mut archive =
            zip::ZipArchive::new(file).map_err(|e| format!("Failed to read archive: {}", e))?;

        for i in 0..archive.len() {
            let mut file = archive
                .by_index(i)
                .map_err(|e| format!("Failed to read archive entry: {}", e))?;

            let outpath = extract_dir.join(file.name());

            if file.name().ends_with('/') {
                fs::create_dir_all(&outpath).ok();
            } else {
                if let Some(p) = outpath.parent() {
                    fs::create_dir_all(p).ok();
                }
                let mut outfile = fs::File::create(&outpath)
                    .map_err(|e| format!("Failed to create file: {}", e))?;
                io::copy(&mut file, &mut outfile)
                    .map_err(|e| format!("Failed to extract file: {}", e))?;
            }
        }

        Ok(extract_dir)
    }

    #[allow(dead_code)]
    fn install_files(&self, extracted_dir: &Path, game_path: &str) -> Result<Vec<String>, String> {
        let game_dir = Path::new(game_path);
        if !game_dir.exists() {
            return Err("Game directory does not exist".to_string());
        }

        let mut installed_files = Vec::new();

        // Walk through extracted files and install them
        for entry in WalkDir::new(extracted_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                let relative_path = entry
                    .path()
                    .strip_prefix(extracted_dir)
                    .map_err(|e| e.to_string())?;

                // Determine installation path based on file structure
                let install_path = self.determine_install_path(game_dir, relative_path)?;

                // Create parent directories
                if let Some(parent) = install_path.parent() {
                    fs::create_dir_all(parent)
                        .map_err(|e| format!("Failed to create directory: {}", e))?;
                }

                // Copy file
                fs::copy(entry.path(), &install_path)
                    .map_err(|e| format!("Failed to copy file: {}", e))?;

                installed_files.push(install_path.to_string_lossy().to_string());
            }
        }

        // Clean up extraction directory
        fs::remove_dir_all(extracted_dir).ok();

        Ok(installed_files)
    }

    #[allow(dead_code)]
    fn determine_install_path(
        &self,
        game_dir: &Path,
        relative_path: &Path,
    ) -> Result<PathBuf, String> {
        // Try to detect common mod structure patterns
        let path_str = relative_path.to_string_lossy().to_lowercase();

        // Check for common Cyberpunk 2077 mod directories
        if path_str.contains("archive") || path_str.contains("archives") {
            Ok(game_dir
                .join("archive")
                .join("pc")
                .join("mod")
                .join(relative_path.file_name().unwrap()))
        } else if path_str.contains("bin") {
            Ok(game_dir
                .join("bin")
                .join("x64")
                .join(relative_path.file_name().unwrap()))
        } else if path_str.contains("r6") {
            Ok(game_dir
                .join("r6")
                .join("scripts")
                .join(relative_path.file_name().unwrap()))
        } else {
            // Default to archive/pc/mod for unknown files
            Ok(game_dir
                .join("archive")
                .join("pc")
                .join("mod")
                .join(relative_path.file_name().unwrap()))
        }
    }

    /// Check for file conflicts with already installed mods
    /// Returns a map of file paths to conflicting mod info
    pub fn check_file_conflicts(&self, files_to_install: &[String]) -> HashMap<String, Vec<ConflictDetails>> {
        let mut conflicts: HashMap<String, Vec<ConflictDetails>> = HashMap::new();

        for file_path in files_to_install {
            // Check if this file is already installed by another mod
            for existing_mod in &self.mods {
                if existing_mod.files.contains(file_path) {
                    conflicts
                        .entry(file_path.clone())
                        .or_insert_with(Vec::new)
                        .push(ConflictDetails {
                            mod_id: existing_mod.id.clone(),
                            mod_name: existing_mod.name.clone(),
                            mod_version: existing_mod.version.clone(),
                            is_archive: file_path.ends_with(".archive"),
                        });
                }
            }
        }

        conflicts
    }

    /// Analyze archive file load order conflicts
    /// Returns warnings about which archive will override which
    pub fn analyze_archive_load_order(&self, archive_files: &[String]) -> Vec<LoadOrderWarning> {
        let mut warnings = Vec::new();
        
        // Get all installed archive files from all mods
        let mut all_archives: Vec<(String, String, String)> = Vec::new(); // (filename, mod_name, mod_id)
        
        for existing_mod in &self.mods {
            for file in &existing_mod.files {
                if file.ends_with(".archive") {
                    if let Some(filename) = Path::new(file).file_name() {
                        all_archives.push((
                            filename.to_string_lossy().to_string(),
                            existing_mod.name.clone(),
                            existing_mod.id.clone(),
                        ));
                    }
                }
            }
        }
        
        // Add new archives being installed
        for file in archive_files {
            if let Some(filename) = Path::new(file).file_name() {
                all_archives.push((
                    filename.to_string_lossy().to_string(),
                    "NEW MOD".to_string(),
                    "new".to_string(),
                ));
            }
        }
        
        // Sort archives alphabetically (this is how CP2077 loads them)
        all_archives.sort_by(|a, b| a.0.cmp(&b.0));
        
        // Check for archives that might conflict
        // Group by basegame_ prefix or other common patterns
        let mut basegame_archives = Vec::new();
        let mut patch_archives = Vec::new();
        
        for (filename, mod_name, mod_id) in &all_archives {
            if filename.starts_with("basegame_") || filename.starts_with("basegame-") {
                basegame_archives.push((filename.clone(), mod_name.clone(), mod_id.clone()));
            } else if filename.starts_with("patch_") || filename.starts_with("patch-") {
                patch_archives.push((filename.clone(), mod_name.clone(), mod_id.clone()));
            }
        }
        
        // Warn if multiple mods modify basegame
        if basegame_archives.len() > 1 {
            let last_loaded = basegame_archives.last().unwrap();
            warnings.push(LoadOrderWarning {
                warning_type: LoadOrderWarningType::MultipleBasegameArchives,
                message: format!(
                    "Multiple mods modify basegame archives. '{}' will load last and override others.",
                    last_loaded.0
                ),
                affected_archives: basegame_archives.iter().map(|a| a.0.clone()).collect(),
                suggestion: Some(
                    "Consider renaming archives to control load order:\n\
                     - Prefix with '0-' to load first (e.g., '0-basegame_textures.archive')\n\
                     - Prefix with 'z-' to load last (e.g., 'z-basegame_final.archive')"
                        .to_string(),
                ),
            });
        }
        
        warnings
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictDetails {
    pub mod_id: String,
    pub mod_name: String,
    pub mod_version: String,
    pub is_archive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadOrderWarning {
    pub warning_type: LoadOrderWarningType,
    pub message: String,
    pub affected_archives: Vec<String>,
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LoadOrderWarningType {
    MultipleBasegameArchives,
    MultiplePatchArchives,
    ConflictingMods,
}
