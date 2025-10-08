# Development Guide

This guide provides information for developers working on the Crossover Mod Manager.

## Development Setup

### Prerequisites

1. **Node.js and npm**
   - Node.js 18 or higher
   - npm 8 or higher
   - Check versions: `node --version && npm --version`

2. **Rust**
   - Rust 1.70 or higher
   - Install via [rustup](https://rustup.rs/)
   - Check version: `rustc --version`

3. **Tauri Dependencies**
   
   **macOS:**
   ```bash
   xcode-select --install
   ```
   
   **Linux (Ubuntu/Debian):**
   ```bash
   sudo apt update
   sudo apt install libwebkit2gtk-4.0-dev \
     build-essential \
     curl \
     wget \
     file \
     libssl-dev \
     libgtk-3-dev \
     libayatana-appindicator3-dev \
     librsvg2-dev
   ```

### Initial Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/beneccles/crossover-mod-manager.git
   cd crossover-mod-manager
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

## Development Workflow

### Running the Development Server

Start both the Vite dev server and Tauri app:

```bash
npm run tauri:dev
```

This will:
- Start Vite dev server on `http://localhost:1420`
- Launch the Tauri application
- Enable hot-reload for frontend changes
- Rebuild Rust code on changes

### Frontend-Only Development

If you want to work on the frontend without Tauri:

```bash
npm run dev
```

Note: Tauri API calls will fail in this mode.

### Building for Production

Build the complete application:

```bash
npm run tauri:build
```

Build outputs:
- **macOS**: `src-tauri/target/release/bundle/macos/`
- **Linux**: `src-tauri/target/release/bundle/appimage/`
- **Windows**: `src-tauri/target/release/bundle/msi/`

## Project Structure

```
crossover-mod-manager/
├── src/                       # React frontend source
│   ├── components/           # React components
│   │   ├── ModList.jsx      # Installed mods list
│   │   ├── ModList.css
│   │   ├── ModDetails.jsx   # Mod details view
│   │   ├── ModDetails.css
│   │   ├── Settings.jsx     # Settings panel
│   │   └── Settings.css
│   ├── App.jsx              # Main app component
│   ├── App.css
│   ├── main.jsx             # React entry point
│   └── index.css            # Global styles
├── src-tauri/               # Rust backend
│   ├── src/
│   │   ├── main.rs          # Tauri app entry, command handlers
│   │   ├── mod_manager.rs   # Mod installation/removal logic
│   │   └── settings.rs      # Settings persistence
│   ├── Cargo.toml           # Rust dependencies
│   ├── tauri.conf.json      # Tauri configuration
│   └── build.rs             # Build script
├── dist/                    # Vite build output (generated)
├── node_modules/            # npm dependencies (generated)
├── package.json             # npm configuration
├── vite.config.js           # Vite configuration
├── index.html               # HTML entry point
└── README.md                # User documentation
```

## Key Technologies

### Frontend Stack
- **React 19**: UI library
- **Vite 7**: Build tool and dev server
- **CSS**: Styling (no preprocessor)
- **@tauri-apps/api**: Tauri JavaScript API

### Backend Stack
- **Tauri 1.5**: Desktop app framework
- **Rust**: Backend language
- **serde/serde_json**: JSON serialization
- **reqwest**: HTTP client for downloads
- **zip**: Archive extraction
- **walkdir**: File system traversal

## Architecture

### Communication Flow

```
React UI ──invoke()──> Tauri Commands ──> Rust Backend
                                            ├── ModManager
                                            ├── Settings
                                            └── File System
```

### Tauri Commands

Commands exposed to the frontend (defined in `src-tauri/src/main.rs`):

- `get_installed_mods()`: Returns list of installed mods
- `install_mod(mod_data)`: Installs a new mod
- `remove_mod(mod_id)`: Removes an installed mod
- `get_settings()`: Retrieves app settings
- `save_settings(settings)`: Saves app settings

### Data Storage

All data is stored in the user's home directory:

```
~/.crossover-mod-manager/
├── mods.json       # Installed mods database
└── settings.json   # Application settings
```

## Adding Features

### Adding a New Frontend Component

1. Create component file: `src/components/NewComponent.jsx`
2. Create styles: `src/components/NewComponent.css`
3. Import in `App.jsx`: `import NewComponent from './components/NewComponent'`
4. Use in render: `<NewComponent />`

### Adding a New Tauri Command

1. Define function in Rust (`src-tauri/src/main.rs` or separate module):
   ```rust
   #[tauri::command]
   fn my_command(param: String) -> Result<String, String> {
       Ok(format!("Processed: {}", param))
   }
   ```

2. Register in `main()`:
   ```rust
   .invoke_handler(tauri::generate_handler![
       // ... existing commands
       my_command
   ])
   ```

3. Call from frontend:
   ```javascript
   import { invoke } from '@tauri-apps/api/tauri'
   
   const result = await invoke('my_command', { param: 'value' })
   ```

### Modifying Mod Installation Logic

The mod installation logic is in `src-tauri/src/mod_manager.rs`:

- `install_mod()`: Main installation flow
- `download_mod()`: Downloads from URL
- `extract_mod()`: Extracts ZIP archive
- `install_files()`: Copies files to game directory
- `determine_install_path()`: Determines where files should go

## Testing

### Manual Testing

1. Start dev server: `npm run tauri:dev`
2. Test mod list display
3. Test settings save/load
4. Test mod installation (requires mock data)
5. Test mod removal

### Testing Mod Installation

Create a test mod data object:

```javascript
const testMod = {
  name: "Test Mod",
  version: "1.0.0",
  author: "Test Author",
  description: "A test mod",
  download_url: "https://example.com/mod.zip",
  mod_id: "123",
  file_id: "456"
}

await invoke('install_mod', { modData: testMod })
```

## Debugging

### Frontend Debugging

- Open DevTools in Tauri window: Right-click → Inspect Element
- Console logs appear in DevTools
- React DevTools extension works

### Backend Debugging

- Add print statements: `println!("Debug: {}", value);`
- Run with console output:
  ```bash
  RUST_LOG=debug npm run tauri:dev
  ```
- Logs appear in terminal

### Common Issues

1. **"Failed to resolve import"**: Check npm dependencies
2. **"Build failed" (Rust)**: Check Cargo.toml versions
3. **"Permission denied"**: Check file system permissions
4. **"Protocol handler not working"**: Re-register the app

## Code Style

### JavaScript/React
- Use functional components with hooks
- Use arrow functions for callbacks
- Use async/await for promises
- Keep components focused and small
- Use CSS modules for component styles

### Rust
- Follow standard Rust formatting (`cargo fmt`)
- Use `Result<T, String>` for errors
- Handle errors with `map_err()`
- Keep functions focused
- Add comments for complex logic

## Contributing

1. Create a feature branch
2. Make changes
3. Test thoroughly
4. Submit pull request
5. Ensure CI passes

## Resources

- [Tauri Documentation](https://tauri.app/)
- [React Documentation](https://react.dev/)
- [Vite Documentation](https://vitejs.dev/)
- [Rust Documentation](https://doc.rust-lang.org/)
- [NexusMods API](https://app.swaggerhub.com/apis-docs/NexusMods/nexus-mods_public_api_params_in_form_data/1.0)
