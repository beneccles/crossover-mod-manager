# Mod Profile Export/Import Feature

## Overview

This feature allows users to export their complete mod profile to a JSON file and later import it to automatically register mods whose files already exist on disk. Perfect for backing up your mod setup or transferring to a new system.

## Features

### Export Functionality

- **Export Button**: Located in Settings tab under "� Mod Profile Export & Import"
- **Export File Format**: JSON file containing complete mod information including file paths and NexusMods IDs

### Import Functionality

- **Import Button**: Located in Settings tab under "� Mod Profile Export & Import"
- **Smart Import Process**:
  1. Parses the mod profile file
  2. Checks which mods' files already exist on disk
  3. Automatically registers mods whose files are present
  4. Flags missing mods for manual re-download from NexusMods
  5. Shows detailed results in the Logs tab

### File Verification

- CMM verifies if mod files exist on disk before requiring re-download
- Automatically registers mods if their files are already present
- Perfect for migrating to a new system where you've already copied game files

## Usage

### Exporting Mod List

1. Open the Settings tab
2. Scroll to "📦 Mod List Backup & Restore" section
3. Click "📤 Export Mod List"
4. Choose a location and filename (default: `cmm-modlist-export.json`)
5. Click Save

**Note**: Only mods downloaded from NexusMods (with mod_id and file_id) can be exported.

### Importing Mod List

1. Open the Settings tab
2. Scroll to "📦 Mod List Backup & Restore" section
3. Ensure you have a valid NexusMods API key configured
4. Click "📥 Import Mod List"
5. Select your previously exported JSON file
6. Confirm the import when prompted
7. CMM will:
   - Check which mods are already installed
   - Download and install any missing mods
   - Log all progress in the Logs tab

**Requirements**:

- Valid NexusMods API key
- Internet connection for downloading mods

## Technical Implementation

### Backend (Rust)

#### Data Structures (`mod_manager.rs`)

```rust
pub struct ModListExport {
    pub export_version: String,
    pub export_date: String,
    pub game_domain: String,
    pub mods: Vec<ExportedModInfo>,
}

pub struct ExportedModInfo {
    pub name: String,
    pub version: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub mod_id: String,
    pub file_id: String,
    pub game_domain: String,
}
```

#### Key Functions (`mod_manager.rs`)

- `export_mod_list()`: Exports installed mods to JSON file
- `parse_import_file()`: Parses import file and determines what needs to be done
- `check_mod_files_exist()`: Verifies if mod files still exist on disk

#### Tauri Commands (`main.rs`)

- `export_mod_list`: Handles export requests from frontend
- `import_mod_list`: Handles import requests and orchestrates the download/install process

### Frontend (React)

#### UI Components (`Settings.jsx`)

- Export button with save dialog
- Import button with open dialog and confirmation
- Help text explaining the feature
- Integration with NexusMods API key requirement

#### Functions

- `exportModList()`: Opens save dialog and calls backend export command
- `importModList()`: Opens file dialog, confirms with user, and calls backend import command

## File Format Example

```json
{
  "export_version": "1.0",
  "export_date": "2025-10-15T12:34:56.789Z",
  "game_domain": "cyberpunk2077",
  "mods": [
    {
      "name": "Cyber Engine Tweaks",
      "version": "1.32.0",
      "author": "yamashi",
      "description": "Framework for modding Cyberpunk 2077",
      "mod_id": "107",
      "file_id": "123169",
      "game_domain": "cyberpunk2077"
    },
    {
      "name": "RedMod DLC Fix",
      "version": "2.1",
      "author": "rfuzzo",
      "description": "Fixes DLC loading issues",
      "mod_id": "456",
      "file_id": "789012",
      "game_domain": "cyberpunk2077"
    }
  ]
}
```

## Logging

All export/import operations are logged with detailed information:

- Export: Number of mods exported, file path
- Import: Mods already installed, mods being downloaded, success/failure for each mod
- Errors: API failures, file not found, etc.

Check the **Logs** tab for detailed progress during import operations.

## Limitations

1. **NexusMods Only**: Only mods downloaded from NexusMods with valid mod_id and file_id can be exported/imported
2. **API Key Required**: Import requires a valid NexusMods API key
3. **Manual Mods**: Manually installed mods (not from NexusMods) cannot be exported
4. **File Availability**: Mods that have been removed from NexusMods cannot be re-downloaded
5. **Premium vs Non-Premium**: Premium users have faster download speeds; non-premium users may need to use "Download with Mod Manager" button

## Future Enhancements

- Support for multiple game domains
- Selective import (choose which mods to import)
- Import preview before downloading
- Batch download progress tracking
- Export/import mod settings and configurations
- Support for mod collections from NexusMods

## Error Handling

The import process handles various error scenarios:

- Invalid JSON format
- Missing NexusMods API key
- Network failures
- Mod no longer available on NexusMods
- Installation failures

All errors are logged and reported to the user with clear messages.
