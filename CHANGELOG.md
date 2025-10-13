# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.5.0] - 2025-10-12

### Added

- **Phase 3 Crossover Compatibility Complete** 🎉

  - **File Permissions Management** (Priority #8)

    - Automatic Wine-compatible permission setting after file installation
    - Files set to 0o644 (rw-r--r--) for proper Wine DLL loading
    - Directories set to 0o755 (rwxr-xr-x) for traversability
    - Improves Wine DLL loading and config file writability
    - Unix-only feature (no-op on Windows)

  - **Long Path Validation** (Priority #9)

    - Checks PATH_MAX (1024 characters) before installation
    - Warns if path approaches 900 characters (safety margin)
    - Hard error if path exceeds maximum allowed length
    - Suggests shorter Crossover bottle names or paths
    - Prevents cryptic macOS filesystem failures

  - **Windows Version Detection** (Priority #11)
    - Automatic detection of Wine's configured Windows version
    - Reads Wine registry (system.reg) from bottle
    - Validates against recommended version (Windows 10)
    - Warns if older versions (Win 7/8) are detected
    - Provides step-by-step instructions to change Wine version
    - macOS-specific feature with graceful fallback

### Changed

- Enhanced installation flow with pre-installation validation checks
- Improved logging for Wine/Crossover compatibility issues
- Better error messages for path and permission issues

### Technical

- Added Wine compatibility functions in `main.rs`:
  - `set_wine_compatible_permissions()` - Sets Unix permissions on files/directories
  - `check_path_length()` - Validates path length against macOS limits
  - `detect_wine_windows_version()` - Parses Wine registry for version info
- Integrated checks into installation flow:
  - Path length validation before file installation begins
  - Windows version detection and warnings
  - Permission setting after each file copy
  - Directory permission setting after creation
- Cross-platform compatibility with conditional compilation

## [1.4.0] - 2025-10-12

### Added

- **Disk Space Checking** (Priority #7 - Phase 3)
  - Automatic disk space verification before mod download
  - Checks available space before extraction (prevents mid-install failures)
  - Requires 3x mod size for download + extraction + buffer
  - Human-readable size formatting (KB, MB, GB)
  - Clear error messages with available vs required space
  - Platform-specific implementation using statvfs on Unix
  - Separate checks for temp directory and game directory
  - Helpful tips for freeing up space

### Changed

- Enhanced download logging with formatted byte sizes
- Improved error messages for insufficient disk space scenarios

### Technical

- Added `nix` crate dependency (v0.29) for statvfs filesystem statistics
- Created disk space utility functions in `main.rs`:
  - `get_available_disk_space()` - Uses statvfs to get filesystem info
  - `format_bytes()` - Converts bytes to human-readable format
  - `check_sufficient_disk_space()` - Validates space before operations
- Integrated checks at two critical points:
  - Before downloading (temp directory check)
  - Before extraction (game directory check)

## [1.3.0] - 2025-10-11

### Added

- **Multi-Format Archive Support** (Enhanced Compatibility)
  - 7-Zip (.7z) format support with hybrid extraction
  - RAR (.rar) format support with hybrid extraction
  - Intelligent system tool detection (p7zip, unrar)
  - Automatic fallback to built-in Rust extractors
  - Installation hints for optimal performance
  - Extraction method logging and reporting
  - Magic byte detection for accurate format identification
  - Now supports 99% of NexusMods archive formats (ZIP/7z/RAR)
- **Archive Load Order Management & Conflict Detection** (Priority #3 - Phase 2)
  - Intelligent file conflict detection during installation
  - Warns when multiple mods modify the same files
  - Special handling for .archive files (load order awareness)
  - Explains which mod will override which based on alphabetical load order
  - Suggests file renaming strategies to control load order (0-, z- prefixes)
  - Tracks file ownership across all installed mods
  - Categorizes conflicts (archive vs non-archive files)
  - Prevents silent file overwrites between mods
- **Symlink Detection & Warning System** (Priority #4 - Phase 2)
  - Automatic detection of symbolic links during mod installation
  - Warns users about Wine/Crossover symlink compatibility issues
  - Lists all detected symlinks with their targets
  - Symlinks are skipped during installation for compatibility
  - Platform-specific advice for macOS/Crossover users
  - Prevents symlink-related mod failures
- **Unicode Filename Sanitization** (Priority #6 - Phase 2)
  - Automatic detection of non-ASCII characters in filenames
  - Transliteration to ASCII-safe equivalents (café→cafe, Zürich→Zurich)
  - Prevents Wine/Crossover file encoding issues
  - Comprehensive warning summary with before/after mapping
  - Platform-specific compatibility advice
  - Improves mod reliability in Wine environments
- **Archive Extraction Documentation**
  - New ARCHIVE_SUPPORT.md with comprehensive technical details
  - Performance comparisons between extraction methods
  - Installation guides for system tools
  - Troubleshooting section

### Changed

- Replaced ZIP-only extraction with unified archive extractor
- Mod installation now detects and handles multiple archive formats
- Enhanced logging with extraction method information

### Technical

- Created `archive_extractor.rs` module (361 lines)
  - Added `ArchiveType` enum (Zip, SevenZ, Rar, Unsupported)
  - Added `ExtractionMethod` enum for tracking extraction methods
  - Added `detect_archive_type()` - magic byte detection with extension fallback
  - Added `extract_7z_hybrid()` - system p7zip with sevenz-rust fallback
  - Added `extract_rar_hybrid()` - system unrar with unrar crate fallback
  - Added `check_command_exists()` - system tool availability checking
  - Added `get_installation_hints()` - user guidance for missing tools
  - Dependencies: sevenz-rust 0.6, unrar 0.5
- Enhanced `mod_manager.rs` with conflict detection
  - Added `FileConflictInfo` struct for tracking file ownership
  - Added `ConflictDetails` struct for reporting conflicts
  - Added `LoadOrderWarning` struct for archive load order issues
  - Added `check_file_conflicts()` - detect file overlaps between mods
  - Added `analyze_archive_load_order()` - detect archive conflicts
  - Extended `ModInfo` with `file_conflicts` and `installed_at` fields
  - Added timestamp tracking for installation order
- Updated `main.rs` installation flow
  - Integrated conflict detection before mod registration
  - Added detailed conflict warnings for users
  - Separate reporting for archive vs non-archive conflicts
  - Load order education and renaming suggestions
  - Added symlink detection during file iteration
  - Symlink tracking with path and target information
  - Comprehensive symlink warnings with platform-specific advice
  - Automatic symlink skipping for Wine/Crossover compatibility
  - Added Unicode filename detection during file iteration
  - Unicode transliteration using unidecode library
  - ASCII sanitization with comprehensive character filtering
  - Before/after filename mapping in warnings
  - Platform-specific Unicode compatibility advice
  - Dependencies: unidecode 0.3

### Performance

- System p7zip: ~45% faster than built-in 7z extractor
- System unrar: ~50% faster than built-in RAR extractor
- Zero installation required (Rust libraries always available)
- Optimal performance when system tools installed

### Documentation

- Added ARCHIVE_SUPPORT.md with architecture and usage details
- Updated mod installation flow documentation

## [1.2.0] - 2025-10-11

### Added

- **Comprehensive Case Sensitivity Handling** (Priority #2 - Phase 1 Complete)
  - Automatic path normalization for all Cyberpunk 2077 game directories
  - Case mismatch detection during mod installation
  - Detailed warnings and auto-correction notifications
  - Helper functions for case-insensitive file operations
  - Summary statistics for corrected files
  - Platform-specific tips for macOS/Crossover users
  - Prevents "file not found" errors on case-sensitive Wine filesystems

### Changed

- Updated `determine_install_path_for_file()` to use normalized paths
- Enhanced installation logging with case sensitivity warnings
- Improved Wine/Crossover compatibility documentation

### Technical

- Added `normalize_game_path_component()` - normalizes directory names
- Added `normalize_game_path()` - normalizes full file paths
- Added `check_case_mismatch()` - detects incorrect casing
- Added `find_path_case_insensitive()` - case-insensitive path lookup
- Tracks case mismatch count during installation
- Detects existing files with different casing before overwriting

### Documentation

- Updated CROSSOVER_COMPATIBILITY.md with implementation details
- Marked Phase 1 (Critical Fixes) as COMPLETED
- Added comprehensive examples of user experience

## [1.1.0] - 2025-10-10

### Added

- **REDmod Launch Parameter Detection** (Priority #1 - Critical)

  - Automatic detection of REDmod mods during installation
  - Prominent warnings about `-modded` parameter requirement
  - Platform-specific launcher instructions (GOG/Steam/Epic)
  - Clear guidance to prevent silent mod failures

- **Duplicate Mod Detection**

  - Check for exact same mod and file version
  - Detect different versions of same mod
  - Warn about potential name conflicts
  - Prevent wasted downloads and installations

- **RED4ext Support Improvements**
  - Fixed version.dll placement (game root, not bin/x64/)
  - Enhanced file detection and logging
  - Comprehensive Crossover setup documentation
  - Wine DLL configuration guidance

### Documentation

- Created CROSSOVER_COMPATIBILITY.md guide
- Created RED4EXT_COMPATIBILITY.md guide
- Documented 12 potential Crossover/Wine issues
- Added 3-phase implementation roadmap

## [1.0.0] - 2025-01-XX

### Added

- Initial release of Crossover Mod Manager
- React + Vite frontend with modern, dark-themed UI
- Tauri + Rust backend for file system operations
- Mod list view showing all installed mods
- Mod details panel with file information
- Settings panel for configuring game path
- Automatic mod download from NexusMods
- ZIP archive extraction
- Smart file placement based on file types
- Mod database (JSON) for tracking installed mods
- Safe mod removal without affecting vanilla files
- NexusMods protocol handler (nxm://) registration
- Support for Cyberpunk 2077 mods
- Comprehensive README documentation
- Development guide for contributors
- Contributing guidelines

### Features

- **Mod Management**
  - Install mods directly from NexusMods
  - View all installed mods in sidebar
  - See detailed mod information
  - Track installed files per mod
  - Remove mods safely
- **Installation Logic**
  - Downloads mods from URLs
  - Extracts ZIP archives automatically
  - Determines correct installation paths
  - Supports archive files (→ `archive/pc/mod/`)
  - Supports bin files (→ `bin/x64/`)
  - Supports R6 scripts (→ `r6/scripts/`)
- **Settings**
  - Configure game installation path
  - Persistent settings storage
  - Directory picker for easy path selection
- **UI/UX**
  - Clean, modern interface
  - Dark theme optimized for gaming
  - Loading indicators for operations
  - Responsive layout
  - Tabbed navigation (Mods/Settings)
- **Data Persistence**
  - JSON database at `~/.crossover-mod-manager/mods.json`
  - Settings file at `~/.crossover-mod-manager/settings.json`

### Technical Stack

- React 19
- Vite 7
- Tauri 1.5
- Rust 1.70+
- Dependencies: reqwest, zip, walkdir, serde, uuid, dirs

### Platform Support

- macOS (primary target)
- Designed for games running via Crossover
- Specifically configured for Cyberpunk 2077

[1.0.0]: https://github.com/beneccles/crossover-mod-manager/releases/tag/v1.0.0
