# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
