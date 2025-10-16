# Crossover Mod Manager

[![Build Status](https://github.com/beneccles/crossover-mod-manager/workflows/Build%20and%20Test/badge.svg)](https://github.com/beneccles/crossover-mod-manager/actions)
[![Release](https://img.shields.io/github/v/release/beneccles/crossover-mod-manager?include_prereleases)](https://github.com/beneccles/crossover-mod-manager/releases)
[![License](https://img.shields.io/github/license/beneccles/crossover-mod-manager)](https://github.com/beneccles/crossover-mod-manager/blob/main/LICENSE)

A Nexus Mod Manager for PC games on Mac via Crossover, built with React, Vite, Tauri, and Rust.

**Currently this only works with Cyberpunk 2077, but I have plans to support more games in the future.**

## Download / Installation

### Pre-built Releases

Download the latest beta release:

**[📦 Download Latest Release](https://github.com/beneccles/crossover-mod-manager/releases)**

Available packages:

- **macOS (Apple Silicon)**: `.dmg` file for M1/M2/M3/M4 Macs

**Platform Requirements**:

- macOS 11.0+ (Big Sur or later)
- Apple Silicon Mac (M1/M2/M3/M4)
- CrossOver 25 or later (recommended; v24 may work but is untested)

### Beta Releases

During beta testing, releases are marked with the 🧪 BETA tag. To download a beta:

1. Visit the [Releases page](https://github.com/beneccles/crossover-mod-manager/releases)
2. Look for releases tagged with **🧪 BETA Release** (e.g., v0.1.0-beta1)
3. Download the `Crossover.Mod.Manager_*_aarch64.dmg` file
4. Follow the installation instructions below

⚠️ **Beta Software Notice**: Beta releases may contain bugs and are intended for testing purposes. Please [report any issues](https://github.com/beneccles/crossover-mod-manager/issues) you encounter.

### Installation Instructions

1. **Download** the `.dmg` file from the releases page
2. **Open** the DMG file (double-click)
3. **Drag** the Crossover Mod Manager app to your Applications folder
4. **First Launch** - macOS Security Warning:
   - When you first try to open the app, macOS will block it with a security warning
   - This is normal for beta software that isn't notarized by Apple
   - **To open the app**:
     1. Right-click (or Control-click) the app in Applications
     2. Select "Open" from the menu
     3. Click "Open" again in the security dialog
   - After this first time, the app will open normally with a regular double-click

**Why the security warning?** Beta releases use ad-hoc signing to allow for rapid iteration. A fully signed and notarized version will be available with the stable v1.0.0 release.

### Building from Source

If you want to build from source, see the [Development](#development) section below.

## Features

- **NexusMods Integration**: Responds to 'Download with Mod Manager' links on NexusMods
- **Automatic Installation**: Downloads, unpacks, and installs mods to the correct game directories
- **Multi-Format Archive Support**: Handles ZIP, 7z, and RAR archives (99% of NexusMods mods)
- **Hybrid Extraction**: Uses system tools when available, falls back to built-in extractors
- **Mod Tracking**: Keeps track of all installed files for each mod
- **Safe Removal**: Removes mods without affecting vanilla game files
- **Cyberpunk 2077 Support**: Specifically configured for CP2077 via Crossover
- **Crossover Compatibility**: Case sensitivity handling and Wine-specific optimizations

📖 **[See detailed feature documentation →](FEATURES.md)**

## Project Structure

```
crossover-mod-manager/
├── src/                    # React frontend
│   ├── components/        # React components
│   │   ├── ModList.jsx   # List of installed mods
│   │   ├── ModDetails.jsx # Detailed mod information
│   │   └── Settings.jsx   # Application settings
│   ├── App.jsx            # Main application component
│   ├── main.jsx           # React entry point
│   └── index.css          # Global styles
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── main.rs        # Tauri application entry
│   │   ├── mod_manager.rs # Mod installation/removal logic
│   │   └── settings.rs    # Settings management
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Tauri configuration
└── package.json           # Node.js dependencies
```

## Technology Stack

- **Frontend**: React 19 + Vite 7
- **Backend**: Tauri 1.5 + Rust
- **Features**:
  - File system operations for mod installation
  - Multi-format archive extraction (ZIP, 7z, RAR)
  - Hybrid extraction with system tool detection
  - HTTP downloads from NexusMods
  - Persistent mod database (JSON)
  - Custom URL protocol handler (nxm://)
  - First-run auto-detection of game path
  - Case sensitivity handling for Wine/Crossover

## Prerequisites

### Required

- Node.js 18+ and npm
- Rust 1.70+
- System dependencies for Tauri (macOS):
  - Xcode Command Line Tools: `xcode-select --install`

### Optional (for better archive extraction performance)

```bash
# Install p7zip for faster 7z extraction
brew install p7zip

# Install unrar for faster RAR extraction
brew install unrar
```

**Note**: Archive extraction works without these tools using built-in Rust libraries, but system tools provide ~45% faster extraction. See [FEATURES.md](FEATURES.md#archive-support) for details.

## Installation (for Developers)

1. Clone the repository:

   ```bash
   git clone https://github.com/beneccles/crossover-mod-manager.git
   cd crossover-mod-manager
   ```

2. Install frontend dependencies:

   ```bash
   npm install
   ```

3. Install Rust dependencies (automatic during build)

## Development

Run the application in development mode:

```bash
npm run tauri:dev
```

This will start both the Vite development server and the Tauri application.

## Building

Build the application for production:

```bash
npm run tauri:build
```

The built application will be available in `src-tauri/target/release/bundle/`.

## Usage

### First-Time Setup

1. Launch the application
2. Go to Settings tab
3. **Configure NexusMods API Key** (required for downloading mods):
   - Visit [NexusMods API Key Settings](https://www.nexusmods.com/users/myaccount?tab=api%20access)
   - Log in to your NexusMods account (create one if you don't have it)
   - Scroll down to "Personal API Keys"
   - Click "Generate API Key"
   - Give it a name (e.g., "Crossover Mod Manager")
   - Copy the generated API key
   - Paste it into the "NexusMods API Key" field in the app's Settings tab
   - Click "Save Settings"
4. **Select your Cyberpunk 2077 installation directory** in Crossover
   - Click "Browse" or enter the path manually
   - Example: `/Users/username/Library/Application Support/CrossOver/Bottles/[BottleName]/drive_c/Program Files/Cyberpunk 2077/`
   - Click "Save Settings"

**Important Notes:**

- Your API key is stored locally on your Mac and never shared with anyone except NexusMods
- Premium NexusMods members get faster download speeds
- Free accounts work but may have slower downloads and require manual download button clicks

### Installing Mods

**Prerequisites:**

- NexusMods API key configured in Settings (see First-Time Setup above)
- Game path configured in Settings

**Steps:**

1. Visit [NexusMods Cyberpunk 2077](https://www.nexusmods.com/cyberpunk2077) and find a mod you want to install
2. Click "Download with Mod Manager" (the nxm:// link)
3. Your browser may ask for permission to open the link with Crossover Mod Manager - click "Allow"
4. The application will automatically:
   - Download the mod archive from NexusMods
   - Extract the contents
   - Install files to the appropriate game directories
   - Track all installed files for safe removal

### Managing Mods

- **View Mods**: See all installed mods in the left sidebar
- **Mod Details**: Click on a mod to view its details, version, and installed files
- **Remove Mod**: Click "Remove Mod" to uninstall and delete all mod files

## Uninstalling

To completely remove Crossover Mod Manager from your system:

### 1. Delete the Application

```bash
# Remove the app bundle
rm -rf "/Applications/Crossover Mod Manager.app"
```

Or simply drag `Crossover Mod Manager.app` from Applications to the Trash.

### 2. Remove Configuration Data (Optional)

The app stores settings and mod database separately from the application bundle. To completely remove all data:

```bash
# Remove configuration and mod database
rm -rf ~/.crossover-mod-manager
```

This directory contains:

- `settings.json`: Your game path, API key, and preferences
- `mods.json`: Database of installed mods and tracked files

**Note**: Deleting the app without removing `~/.crossover-mod-manager` will preserve your settings if you reinstall later.

### 3. Mods Remain in Game Directory

Uninstalling Crossover Mod Manager does **not** remove mods from your game installation. To remove installed mods:

- Use the "Remove Mod" button in the app **before** uninstalling, or
- Manually delete mod files from your Cyberpunk 2077 directory

## Architecture

### Frontend (React + Vite)

The frontend provides a clean, modern UI for managing mods:

- **ModList**: Displays all installed mods with their status
- **ModDetails**: Shows detailed information and installed files
- **Settings**: Configure game paths and application settings

### Backend (Rust + Tauri)

The Rust backend handles all file operations and mod management:

1. **ModManager**: Core mod installation and removal logic

   - Downloads mods from NexusMods
   - Extracts ZIP archives
   - Determines correct installation paths for different file types
   - Tracks installed files in a JSON database
   - Safely removes mods without affecting vanilla files

2. **Settings**: Persistent configuration storage

   - Saves game installation path
   - Stores user preferences

3. **Protocol Handler**: Responds to `nxm://` URLs
   - Registers as a handler for NexusMods downloads
   - Parses mod and file IDs from URLs

### Mod Installation Logic

The application automatically determines where to install files based on their type:

- **Archive files** (`*.archive`): `game/archive/pc/mod/`
- **Bin files**: `game/bin/x64/`
- **R6 scripts**: `game/r6/scripts/`
- **Unknown files**: Default to `game/archive/pc/mod/`

### Data Storage

All mod information is stored in `~/.crossover-mod-manager/`:

- `mods.json`: Database of installed mods and their files
- `settings.json`: Application settings

## Troubleshooting

### NexusMods API Key Issues

**Problem: Downloads fail with "Invalid API Key" or "Unauthorized"**

Solutions:

1. Verify your API key is correctly copied from [NexusMods API Settings](https://www.nexusmods.com/users/myaccount?tab=api%20access)
   - Make sure there are no extra spaces or characters
   - The key should be a long alphanumeric string
2. Check that your NexusMods account is active and in good standing
3. Try generating a new API key and updating it in Settings
4. Click "Save Settings" after entering the key

**Problem: "Download with Mod Manager" doesn't open the app**

Solutions:

1. Make sure you've launched the app at least once
2. Check that nxm:// protocol handler is registered:
   - Try clicking a mod manager download link again
   - Your browser should prompt you to open with Crossover Mod Manager
   - Select "Always allow" for future downloads
3. If still not working, try reinstalling the app

**Problem: Download speeds are very slow**

This is normal for free NexusMods accounts. Solutions:

- Consider [NexusMods Premium](https://www.nexusmods.com/register/premium) for faster downloads
- Free accounts are limited to ~1-2 MB/s
- Premium accounts get full speed downloads

### Game Path Issues

**Problem: "Game directory not found" error**

Solutions:

1. Verify your game path in Settings points to the correct location
2. The path should end with the game folder, e.g., `.../Cyberpunk 2077/`
3. Make sure CrossOver bottle is not moved or renamed
4. Try browsing to the path using the "Browse" button instead of typing manually

### Mod Installation Issues

**Problem: Mod installs but doesn't work in-game**

Solutions:

1. Verify the mod is compatible with your game version
2. Check if the mod requires other mods (dependencies)
3. Some mods need to be loaded in a specific order
4. Restart the game after installing mods
5. Check mod instructions on NexusMods for special requirements

**Problem: Can't remove a mod - files are locked**

Solutions:

1. Make sure the game is closed
2. Close CrossOver completely
3. Try removing the mod again
4. If still failing, the files can be manually deleted from your game directory

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Workflow

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Ensure all tests pass and code is formatted:
   ```bash
   cargo fmt --check  # Check Rust formatting
   cargo clippy       # Check Rust linting
   cargo test         # Run tests
   ```
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to your branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

### CI/CD Pipeline

This project uses GitHub Actions for continuous integration and deployment:

- **Automated Testing**: Every push and PR is tested on Linux
- **Code Quality**: Automatic linting with Clippy and formatting checks
- **Security Audits**: Dependency vulnerability scanning
- **Automated Releases**: Tag-based releases build for macOS and Linux

See [CI_CD.md](CI_CD.md) for detailed information about the build and release process.

### Creating a Release

Maintainers can create releases using the included script:

```bash
./scripts/release.sh 1.7.0
```

This will:

1. Update version in `tauri.conf.json`
2. Create/update CHANGELOG entry
3. Commit changes
4. Create and push a git tag
5. Trigger automated builds for all platforms

## License

MIT License - see [LICENSE](https://github.com/beneccles/crossover-mod-manager/blob/main/LICENSE) file for details.

## Acknowledgments

- Built for the Cyberpunk 2077 modding community
- Designed to work seamlessly with NexusMods
- Optimized for Mac users running games through Crossover 25.
