# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
