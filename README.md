# Crossover Mod Manager

A Nexus Mod Manager for PC games on Mac via Crossover, built with React, Vite, Tauri, and Rust.

## Features

- **NexusMods Integration**: Responds to 'Download with Mod Manager' links on NexusMods
- **Automatic Installation**: Downloads, unpacks, and installs mods to the correct game directories
- **Mod Tracking**: Keeps track of all installed files for each mod
- **Safe Removal**: Removes mods without affecting vanilla game files
- **Cyberpunk 2077 Support**: Specifically configured for CP2077 via Crossover

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
  - Archive extraction (ZIP support)
  - HTTP downloads from NexusMods
  - Persistent mod database (JSON)
  - Custom URL protocol handler (nxm://)

## Prerequisites

- Node.js 18+ and npm
- Rust 1.70+
- System dependencies for Tauri (macOS):
  - Xcode Command Line Tools: `xcode-select --install`

## Installation

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
3. Select your Cyberpunk 2077 installation directory in Crossover
   - Example: `/Users/username/Library/Application Support/CrossOver/Bottles/[BottleName]/drive_c/Program Files/Cyberpunk 2077/`

### Installing Mods

1. Visit NexusMods and find a mod you want to install
2. Click "Download with Mod Manager" (requires NexusMods account)
3. The application will automatically:
   - Download the mod archive
   - Extract the contents
   - Install files to the appropriate game directories
   - Track all installed files

### Managing Mods

- **View Mods**: See all installed mods in the left sidebar
- **Mod Details**: Click on a mod to view its details, version, and installed files
- **Remove Mod**: Click "Remove Mod" to uninstall and delete all mod files

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

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built for the Cyberpunk 2077 modding community
- Designed to work seamlessly with NexusMods
- Optimized for Mac users running games through Crossover
