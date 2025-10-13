use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub enum ArchiveType {
    Zip,
    SevenZ,
    Rar,
    Unsupported(String),
}

#[derive(Debug, Clone)]
pub enum ExtractionMethod {
    RustZip,
    RustSevenz,
    RustUnrar,
    SystemP7zip,
    SystemUnrar,
}

pub struct ArchiveExtractor;

impl ArchiveExtractor {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self
    }

    /// Detect archive type by reading file magic bytes (more reliable than extension)
    pub fn detect_archive_type(archive_path: &Path) -> ArchiveType {
        // Try to read the first few bytes to detect the actual format
        if let Ok(mut file) = fs::File::open(archive_path) {
            let mut magic = [0u8; 8];
            if std::io::Read::read(&mut file, &mut magic).is_ok() {
                // Check magic bytes for each format
                // ZIP: 50 4B 03 04 or 50 4B 05 06 (empty archive) or 50 4B 07 08 (spanned)
                if magic[0] == 0x50
                    && magic[1] == 0x4B
                    && (magic[2] == 0x03 || magic[2] == 0x05 || magic[2] == 0x07)
                {
                    return ArchiveType::Zip;
                }

                // 7z: 37 7A BC AF 27 1C
                if magic[0] == 0x37
                    && magic[1] == 0x7A
                    && magic[2] == 0xBC
                    && magic[3] == 0xAF
                    && magic[4] == 0x27
                    && magic[5] == 0x1C
                {
                    return ArchiveType::SevenZ;
                }

                // RAR: 52 61 72 21 1A 07 (RAR 1.5+) or 52 61 72 21 1A 07 01 00 (RAR 5.0+)
                if magic[0] == 0x52
                    && magic[1] == 0x61
                    && magic[2] == 0x72
                    && magic[3] == 0x21
                    && magic[4] == 0x1A
                    && magic[5] == 0x07
                {
                    return ArchiveType::Rar;
                }
            }
        }

        // Fallback to extension-based detection if magic bytes don't match
        match archive_path.extension().and_then(|s| s.to_str()) {
            Some("zip") => ArchiveType::Zip,
            Some("7z") => ArchiveType::SevenZ,
            Some("rar") => ArchiveType::Rar,
            Some(ext) => ArchiveType::Unsupported(ext.to_string()),
            None => ArchiveType::Unsupported("unknown".to_string()),
        }
    }

    /// Extract archive using hybrid approach (system tools + Rust fallbacks)
    pub fn extract(
        archive_path: &Path,
        extract_dir: &Path,
    ) -> Result<(usize, ExtractionMethod), String> {
        let archive_type = Self::detect_archive_type(archive_path);

        match archive_type {
            ArchiveType::Zip => Self::extract_zip(archive_path, extract_dir),
            ArchiveType::SevenZ => Self::extract_7z_hybrid(archive_path, extract_dir),
            ArchiveType::Rar => Self::extract_rar_hybrid(archive_path, extract_dir),
            ArchiveType::Unsupported(ext) => Err(format!("Unsupported archive format: .{}", ext)),
        }
    }

    /// Extract ZIP using Rust zip crate
    fn extract_zip(
        archive_path: &Path,
        extract_dir: &Path,
    ) -> Result<(usize, ExtractionMethod), String> {
        let file = fs::File::open(archive_path)
            .map_err(|e| format!("Failed to open ZIP archive: {}", e))?;

        let mut archive =
            zip::ZipArchive::new(file).map_err(|e| format!("Failed to read ZIP: {}", e))?;

        let mut count = 0;
        for i in 0..archive.len() {
            let mut file = archive
                .by_index(i)
                .map_err(|e| format!("Failed to read entry: {}", e))?;

            let outpath = extract_dir.join(file.name());

            if file.name().ends_with('/') {
                fs::create_dir_all(&outpath).ok();
            } else {
                if let Some(p) = outpath.parent() {
                    fs::create_dir_all(p)
                        .map_err(|e| format!("Failed to create directory: {}", e))?;
                }
                let mut outfile = fs::File::create(&outpath)
                    .map_err(|e| format!("Failed to create file: {}", e))?;
                io::copy(&mut file, &mut outfile)
                    .map_err(|e| format!("Failed to extract file: {}", e))?;
                count += 1;
            }
        }

        Ok((count, ExtractionMethod::RustZip))
    }

    /// Extract 7z using hybrid approach (system p7zip or Rust fallback)
    fn extract_7z_hybrid(
        archive_path: &Path,
        extract_dir: &Path,
    ) -> Result<(usize, ExtractionMethod), String> {
        // Try system p7zip first (faster, more compatible)
        if let Ok(count) = Self::try_system_7z(archive_path, extract_dir) {
            return Ok((count, ExtractionMethod::SystemP7zip));
        }

        // Fallback to Rust library
        println!("System p7zip not available, using built-in extractor...");
        Self::extract_7z_rust(archive_path, extract_dir)
    }

    /// Extract 7z using system p7zip command
    fn try_system_7z(archive_path: &Path, extract_dir: &Path) -> Result<usize, String> {
        // Check if 7z is installed
        if !Self::check_command_exists("7z") && !Self::check_command_exists("7za") {
            return Err("7z not installed".to_string());
        }

        let cmd = if Self::check_command_exists("7z") {
            "7z"
        } else {
            "7za"
        };

        // Create extraction directory
        fs::create_dir_all(extract_dir)
            .map_err(|e| format!("Failed to create extraction directory: {}", e))?;

        // Extract archive
        let output = Command::new(cmd)
            .arg("x") // Extract with full paths
            .arg("-y") // Yes to all prompts
            .arg(archive_path)
            .arg(format!("-o{}", extract_dir.display()))
            .output()
            .map_err(|e| format!("Failed to run 7z: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "7z extraction failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        // Count extracted files
        let count = WalkDir::new(extract_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .count();

        Ok(count)
    }

    /// Extract 7z using Rust sevenz-rust crate
    fn extract_7z_rust(
        archive_path: &Path,
        extract_dir: &Path,
    ) -> Result<(usize, ExtractionMethod), String> {
        use sevenz_rust::*;

        fs::create_dir_all(extract_dir)
            .map_err(|e| format!("Failed to create extraction directory: {}", e))?;

        let mut count = 0;
        let mut sz = SevenZReader::open(archive_path, Password::empty())
            .map_err(|e| format!("Failed to open 7z archive: {}", e))?;

        sz.for_each_entries(|entry, reader| {
            if !entry.is_directory() {
                let output_path = extract_dir.join(entry.name());
                if let Some(parent) = output_path.parent() {
                    fs::create_dir_all(parent).ok();
                }

                let mut outfile = fs::File::create(&output_path)?;
                io::copy(reader, &mut outfile)?;

                count += 1;
            }
            Ok(true) // Continue iteration
        })
        .map_err(|e| format!("7z extraction error: {}", e))?;

        Ok((count, ExtractionMethod::RustSevenz))
    }

    /// Extract RAR using hybrid approach (system unrar or Rust fallback)
    fn extract_rar_hybrid(
        archive_path: &Path,
        extract_dir: &Path,
    ) -> Result<(usize, ExtractionMethod), String> {
        // Try system unrar first (faster, more compatible)
        if let Ok(count) = Self::try_system_unrar(archive_path, extract_dir) {
            return Ok((count, ExtractionMethod::SystemUnrar));
        }

        // Fallback to Rust library
        println!("System unrar not available, using built-in extractor...");
        Self::extract_rar_rust(archive_path, extract_dir)
    }

    /// Extract RAR using system unrar command
    fn try_system_unrar(archive_path: &Path, extract_dir: &Path) -> Result<usize, String> {
        // Check if unrar is installed
        if !Self::check_command_exists("unrar") {
            return Err("unrar not installed".to_string());
        }

        // Create extraction directory
        fs::create_dir_all(extract_dir)
            .map_err(|e| format!("Failed to create extraction directory: {}", e))?;

        // Extract archive
        let output = Command::new("unrar")
            .arg("x") // Extract with full paths
            .arg("-y") // Yes to all prompts
            .arg("-o+") // Overwrite existing files
            .arg(archive_path)
            .arg(extract_dir)
            .output()
            .map_err(|e| format!("Failed to run unrar: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "unrar extraction failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        // Count extracted files
        let count = WalkDir::new(extract_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .count();

        Ok(count)
    }

    /// Extract RAR using Rust unrar crate
    fn extract_rar_rust(
        archive_path: &Path,
        extract_dir: &Path,
    ) -> Result<(usize, ExtractionMethod), String> {
        use unrar::Archive;

        fs::create_dir_all(extract_dir)
            .map_err(|e| format!("Failed to create extraction directory: {}", e))?;

        let mut count = 0;
        let mut archive = Archive::new(archive_path)
            .open_for_processing()
            .map_err(|e| format!("Failed to open RAR archive: {}", e))?;

        loop {
            match archive.read_header() {
                Ok(Some(header)) => {
                    let entry = header
                        .entry()
                        .filename
                        .to_str()
                        .ok_or("Invalid filename in RAR")?;

                    if !header.entry().is_directory() {
                        let output_path = extract_dir.join(entry);
                        if let Some(parent) = output_path.parent() {
                            fs::create_dir_all(parent).ok();
                        }

                        archive = header
                            .extract_to(&output_path)
                            .map_err(|e| format!("Failed to extract RAR file: {}", e))?;

                        count += 1;
                    } else {
                        archive = header
                            .skip()
                            .map_err(|e| format!("Failed to skip RAR entry: {}", e))?;
                    }
                }
                Ok(None) => break, // End of archive
                Err(e) => return Err(format!("Failed to read RAR header: {}", e)),
            }
        }

        Ok((count, ExtractionMethod::RustUnrar))
    }

    /// Check if a command exists in PATH
    fn check_command_exists(command: &str) -> bool {
        Command::new("which")
            .arg(command)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    /// Get user-friendly extraction method name
    pub fn method_name(method: &ExtractionMethod) -> &'static str {
        match method {
            ExtractionMethod::RustZip => "Built-in ZIP",
            ExtractionMethod::RustSevenz => "Built-in 7z",
            ExtractionMethod::RustUnrar => "Built-in RAR",
            ExtractionMethod::SystemP7zip => "System p7zip",
            ExtractionMethod::SystemUnrar => "System unrar",
        }
    }

    /// Check which system extractors are available
    pub fn check_system_tools() -> (bool, bool) {
        let p7zip_available = Self::check_command_exists("7z") || Self::check_command_exists("7za");
        let unrar_available = Self::check_command_exists("unrar");
        (p7zip_available, unrar_available)
    }

    /// Get installation hints for missing system tools
    pub fn get_installation_hints() -> Vec<String> {
        let mut hints = Vec::new();
        let (p7zip, unrar) = Self::check_system_tools();

        if !p7zip {
            hints.push("💡 Install p7zip for faster 7z extraction: brew install p7zip".to_string());
        }
        if !unrar {
            hints
                .push("💡 Install unrar for faster RAR extraction: brew install unrar".to_string());
        }

        hints
    }
}
