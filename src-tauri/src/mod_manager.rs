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
    
    // NEW: Game association
    #[serde(default)]
    pub game_id: String,

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

/// Represents a mod profile that can be exported/imported
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModProfile {
    pub version: String, // Profile format version
    pub created_at: String,
    pub game: String, // Game name (e.g., "Cyberpunk 2077")
    pub mod_manager_version: String,
    pub mods: Vec<ModProfileEntry>,
}

/// Individual mod entry in a profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModProfileEntry {
    pub name: String,
    pub version: String,
    pub author: Option<String>,
    pub mod_id: String,  // NexusMods mod ID
    pub file_id: String, // NexusMods file ID
    pub description: Option<String>,
    // Store relative file paths for verification
    pub files: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ModDatabase {
    mods: Vec<ModInfo>,
}

pub struct ModManager {
    database_path: PathBuf,
    mods: Vec<ModInfo>,
    current_game: String,
}

impl ModManager {
    pub fn new() -> Self {
        Self::new_for_game("cyberpunk2077")
    }
    
    pub fn new_for_game(game_id: &str) -> Self {
        let database_path = Self::get_database_path_for_game(game_id);
        
        // Migrate legacy mods.json to mods_cyberpunk2077.json
        if game_id == "cyberpunk2077" && !database_path.exists() {
            Self::migrate_legacy_database(&database_path);
        }
        
        let mods = Self::load_database(&database_path);

        Self {
            database_path,
            mods,
            current_game: game_id.to_string(),
        }
    }
    
    pub fn switch_game(&mut self, game_id: &str) -> Result<(), String> {
        self.database_path = Self::get_database_path_for_game(game_id);
        self.mods = Self::load_database(&self.database_path);
        self.current_game = game_id.to_string();
        Ok(())
    }
    
    #[allow(dead_code)]
    pub fn get_current_game(&self) -> &str {
        &self.current_game
    }

    #[allow(dead_code)]
    fn get_database_path() -> PathBuf {
        Self::get_database_path_for_game("cyberpunk2077")
    }
    
    fn get_database_path_for_game(game_id: &str) -> PathBuf {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let app_dir = home.join(".crossover-mod-manager");

        if !app_dir.exists() {
            fs::create_dir_all(&app_dir).ok();
        }

        app_dir.join(format!("mods_{}.json", game_id))
    }
    
    /// Migrate legacy mods.json to game-specific database
    fn migrate_legacy_database(new_path: &Path) {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let legacy_path = home.join(".crossover-mod-manager").join("mods.json");
        
        if legacy_path.exists() {
            // Read legacy database
            if let Ok(content) = fs::read_to_string(&legacy_path) {
                if let Ok(mut db) = serde_json::from_str::<ModDatabase>(&content) {
                    // Set game_id for all mods if not already set
                    for mod_info in &mut db.mods {
                        if mod_info.game_id.is_empty() {
                            mod_info.game_id = "cyberpunk2077".to_string();
                        }
                    }
                    
                    // Write to new game-specific database
                    if let Ok(json) = serde_json::to_string_pretty(&db) {
                        fs::write(new_path, json).ok();
                        println!("Migrated legacy mods.json to mods_cyberpunk2077.json");
                    }
                }
            }
        }
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

    /// Remove duplicate mod entries based on mod_id and file_id
    /// Keeps the most recent installation (last in the list)
    pub fn deduplicate_mods(&mut self) -> usize {
        use std::collections::HashSet;
        
        let mut seen = HashSet::new();
        let mut deduplicated = Vec::new();
        let original_count = self.mods.len();
        
        // Process in reverse order to keep the most recent entries
        for mod_info in self.mods.iter().rev() {
            if let (Some(mod_id), Some(file_id)) = (&mod_info.mod_id, &mod_info.file_id) {
                let key = (mod_id.clone(), file_id.clone());
                if !seen.contains(&key) {
                    seen.insert(key);
                    deduplicated.push(mod_info.clone());
                }
            } else {
                // Keep mods without mod_id/file_id (shouldn't happen, but be safe)
                deduplicated.push(mod_info.clone());
            }
        }
        
        // Reverse back to restore chronological order
        deduplicated.reverse();
        
        let removed_count = original_count - deduplicated.len();
        self.mods = deduplicated;
        
        removed_count
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
        game_id: String,
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
            game_id,
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

    /// Export all installed mods to a profile JSON file
    pub fn export_profile(&self, game_name: &str) -> Result<ModProfile, String> {
        let mut profile_entries = Vec::new();

        for mod_info in &self.mods {
            // Only export mods that have NexusMods IDs
            if let (Some(mod_id), Some(file_id)) = (&mod_info.mod_id, &mod_info.file_id) {
                profile_entries.push(ModProfileEntry {
                    name: mod_info.name.clone(),
                    version: mod_info.version.clone(),
                    author: mod_info.author.clone(),
                    mod_id: mod_id.clone(),
                    file_id: file_id.clone(),
                    description: mod_info.description.clone(),
                    files: mod_info.files.clone(),
                });
            }
        }

        let profile = ModProfile {
            version: "1.0".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            game: game_name.to_string(),
            mod_manager_version: env!("CARGO_PKG_VERSION").to_string(),
            mods: profile_entries,
        };

        Ok(profile)
    }

    /// Verify if mod files exist on disk
    pub fn verify_mod_files(&self, files: &[String]) -> (bool, Vec<String>, Vec<String>) {
        let mut existing_files = Vec::new();
        let mut missing_files = Vec::new();

        for file_path in files {
            if Path::new(file_path).exists() {
                existing_files.push(file_path.clone());
            } else {
                missing_files.push(file_path.clone());
            }
        }

        let all_exist = missing_files.is_empty();
        (all_exist, existing_files, missing_files)
    }

    /// Import a mod profile and register existing mods or mark for re-download
    pub fn import_profile(
        &mut self,
        profile: ModProfile,
        game_id: String,
    ) -> Result<(Vec<ModProfileEntry>, Vec<ModProfileEntry>), String> {
        let mut registered_mods = Vec::new();
        let mut to_download_mods = Vec::new();

        for profile_entry in profile.mods {
            // Check if this mod is already installed
            if let Some(_existing) =
                self.find_existing_mod(&profile_entry.mod_id, &profile_entry.file_id)
            {
                // Mod already exists, skip it
                registered_mods.push(profile_entry);
                continue;
            }

            // Verify if files exist on disk
            let (all_exist, existing_files, _missing_files) =
                self.verify_mod_files(&profile_entry.files);

            if all_exist && !existing_files.is_empty() {
                // Files exist, register the mod without downloading
                let mod_info = ModInfo {
                    id: uuid::Uuid::new_v4().to_string(),
                    name: profile_entry.name.clone(),
                    version: profile_entry.version.clone(),
                    author: profile_entry.author.clone(),
                    description: profile_entry.description.clone(),
                    mod_id: Some(profile_entry.mod_id.clone()),
                    file_id: Some(profile_entry.file_id.clone()),
                    enabled: true,
                    files: existing_files,
                    game_id: game_id.clone(),
                    file_conflicts: HashMap::new(),
                    installed_at: Some(chrono::Utc::now().to_rfc3339()),
                };

                self.add_mod(mod_info);
                registered_mods.push(profile_entry);
            } else {
                // Files don't exist, add to download queue
                to_download_mods.push(profile_entry);
            }
        }

        // Save the database with newly registered mods
        self.save_database()?;

        Ok((registered_mods, to_download_mods))
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

        // Get the resolver for the current game
        let game_id = &self.current_game;
        let resolver = crate::game_definitions::get_game_by_id(game_id)
            .ok_or_else(|| format!("Unknown game: {}", game_id))?
            .resolver
            .as_ref();

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

                let normalized_path = relative_path;
                let path_str = relative_path.to_string_lossy().to_lowercase();
                let file_name = entry.file_name().to_string_lossy();

                // Use the game resolver to determine install path
                let install_path = resolver.resolve_install_path(
                    game_dir,
                    relative_path,
                    normalized_path,
                    &path_str,
                    &file_name,
                )?;

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
    pub fn check_file_conflicts(
        &self,
        files_to_install: &[String],
    ) -> HashMap<String, Vec<ConflictDetails>> {
        let mut conflicts: HashMap<String, Vec<ConflictDetails>> = HashMap::new();

        for file_path in files_to_install {
            // Check if this file is already installed by another mod
            for existing_mod in &self.mods {
                if existing_mod.files.contains(file_path) {
                    conflicts
                        .entry(file_path.clone())
                        .or_default()
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

    // TODO: Implement active load order management UI
    // Currently unused - load order detection is done inline during installation
    /*
    /// Analyze archive file load order conflicts
    /// Returns warnings about which archive will override which
    #[allow(dead_code)]
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
    */
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictDetails {
    pub mod_id: String,
    pub mod_name: String,
    pub mod_version: String,
    pub is_archive: bool,
}

// TODO: Implement active load order management UI
// Currently unused - kept for future feature implementation
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadOrderWarning {
    pub warning_type: LoadOrderWarningType,
    pub message: String,
    pub affected_archives: Vec<String>,
    pub suggestion: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LoadOrderWarningType {
    MultipleBasegameArchives,
    MultiplePatchArchives,
    ConflictingMods,
}
