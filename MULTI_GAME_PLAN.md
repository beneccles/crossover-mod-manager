# Multi-Game Support - Vertical Slice Plan

## 🎯 Goal

Enable users to mod multiple games through a single application, starting with Cyberpunk 2077 and Skyrim Special Edition as the initial supported games.

---

## 📋 High-Level Approach

### Phase 1: Data Model & Architecture (Foundation)

- Add game configuration system
- Update settings data structure
- Create game detection utilities
- Design UI for game selection

### Phase 2: Backend Implementation (Core Logic)

- Game-specific path handling
- Mod directory structure per game
- Database schema updates
- Game switching logic

### Phase 3: Frontend Implementation (UI/UX)

- Game selector in settings
- First-run game selection
- Current game indicator
- Game-specific mod lists

### Phase 4: Testing & Polish

- Test game switching
- Validate mod isolation
- Update documentation

---

## 🗂️ Detailed Implementation Plan

### 1. DATA MODEL CHANGES

#### 1.1 Settings Structure (Rust)

**File:** `src-tauri/src/settings.rs`

**Current:**

```rust
pub struct AppSettings {
    pub game_path: String,
    pub nexusmods_api_key: String,
    pub first_run: bool,
}
```

**New:**

```rust
pub struct AppSettings {
    pub current_game: String,              // NEW: "cyberpunk2077" | "skyrimse" | etc.
    pub games: HashMap<String, GameConfig>, // NEW: Per-game settings
    pub nexusmods_api_key: String,
    pub first_run: bool,
    pub first_game_setup: bool,            // NEW: Track first game setup
}

pub struct GameConfig {
    pub game_path: String,
    pub game_id: String,                   // NexusMods game domain
    pub mod_staging_path: String,          // Where to stage mod downloads
    pub last_selected: bool,               // Track which was last active
}
```

**Migration Strategy:**

- Detect old settings format
- Migrate existing `game_path` → `games["cyberpunk2077"]`
- Set `current_game` to "cyberpunk2077"

---

#### 1.2 Game Definitions (Rust)

**New File:** `src-tauri/src/game_definitions.rs`

```rust
pub struct GameDefinition {
    pub id: String,                    // "cyberpunk2077"
    pub name: String,                  // "Cyberpunk 2077"
    pub nexus_domain: String,          // "cyberpunk2077"
    pub mod_directories: Vec<String>,  // ["archive/pc/mod", etc.]
    pub detection_files: Vec<String>,  // ["Cyberpunk2077.exe", "bin/x64/Cyberpunk2077.exe"]
    pub icon: String,                  // Path to game icon (optional)
    pub supports_load_order: bool,     // For future load order feature
}

pub fn get_supported_games() -> HashMap<String, GameDefinition> {
    // Returns all supported games
}

pub fn detect_game_from_path(path: &Path) -> Option<GameDefinition> {
    // Auto-detect game from installation path
}
```

**Initial Supported Games:**

1. **Cyberpunk 2077**

   - ID: `cyberpunk2077`
   - NexusMods domain: `cyberpunk2077`
   - Mod dirs: `["archive/pc/mod"]`
   - Detection: `["Cyberpunk2077.exe", "bin/x64/Cyberpunk2077.exe"]`

2. **Skyrim Special Edition**
   - ID: `skyrimse`
   - NexusMods domain: `skyrimspecialedition`
   - Mod dirs: `["Data"]`
   - Detection: `["SkyrimSE.exe"]`

---

#### 1.3 Mod Database Changes

**File:** `src-tauri/src/mod_manager.rs`

**Current:**

```rust
pub struct ModInfo {
    pub mod_id: i32,
    pub file_id: i32,
    // ... other fields
}
```

**New:**

```rust
pub struct ModInfo {
    pub mod_id: i32,
    pub file_id: i32,
    pub game_id: String,              // NEW: Which game this mod belongs to
    pub installed_files: Vec<String>,
    // ... other fields
}
```

**Database Storage:**

- Keep separate JSON files per game: `mods_cyberpunk2077.json`, `mods_skyrimse.json`
- OR: Single JSON with game_id field (I recommend separate files for cleaner isolation)

---

### 2. BACKEND IMPLEMENTATION

#### 2.1 New Tauri Commands

**File:** `src-tauri/src/main.rs`

```rust
#[tauri::command]
async fn get_supported_games() -> Result<Vec<GameDefinition>, String> {
    // Return list of all supported games
}

#[tauri::command]
async fn switch_game(
    game_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    // Switch current game, reload mods for that game
}

#[tauri::command]
async fn detect_game_from_path(
    path: String,
) -> Result<Option<GameDefinition>, String> {
    // Auto-detect which game is at a given path
}

#[tauri::command]
async fn add_game(
    game_id: String,
    game_path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    // Add a new game to settings
}

#[tauri::command]
async fn remove_game(
    game_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    // Remove game from settings (keeps mods database)
}
```

---

#### 2.2 Modified Commands

**Update these existing commands to be game-aware:**

```rust
#[tauri::command]
async fn list_mods(state: State<'_, AppState>) -> Result<Vec<ModInfo>, String> {
    // NOW: Load mods for current_game only
}

#[tauri::command]
async fn handle_nxm_url(
    nxm_url: String,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    // NOW: Validate URL matches current_game
    // OR: Auto-switch game if URL is for different game?
}

#[tauri::command]
async fn install_mod(...) -> Result<(), String> {
    // NOW: Install to current_game's directories
}

#[tauri::command]
async fn uninstall_mod(...) -> Result<(), String> {
    // NOW: Uninstall from current_game's directories
}
```

---

### 3. FRONTEND IMPLEMENTATION

#### 3.1 Settings Component Updates

**File:** `src/components/Settings.jsx`

**Add new sections:**

```jsx
// Game Management Section
<div className="settings-section">
  <h2>🎮 Game Management</h2>

  {/* Current Game Display */}
  <div className="current-game">
    <label>Current Game:</label>
    <div className="game-card">
      <img src={currentGame.icon} alt={currentGame.name} />
      <span>{currentGame.name}</span>
    </div>
  </div>

  {/* Configured Games List */}
  <div className="configured-games">
    <h3>Configured Games</h3>
    {configuredGames.map((game) => (
      <div key={game.id} className="game-item">
        <span>{game.name}</span>
        <span className="game-path">{game.path}</span>
        <button onClick={() => switchGame(game.id)}>
          {game.id === currentGame.id ? "✓ Active" : "Switch"}
        </button>
        <button onClick={() => removeGame(game.id)}>Remove</button>
      </div>
    ))}
  </div>

  {/* Add New Game */}
  <div className="add-game">
    <h3>Add New Game</h3>
    <select
      value={selectedGame}
      onChange={(e) => setSelectedGame(e.target.value)}
    >
      <option value="">Select a game...</option>
      {supportedGames
        .filter((g) => !configuredGames.find((cg) => cg.id === g.id))
        .map((game) => (
          <option key={game.id} value={game.id}>
            {game.name}
          </option>
        ))}
    </select>
    <button onClick={selectGamePath}>Browse for Game Path...</button>
    <button onClick={addGame}>Add Game</button>
  </div>
</div>
```

---

#### 3.2 First-Run Experience

**New Component:** `src/components/GameSetup.jsx`

**Flow:**

1. **Welcome Screen** → "Let's set up your first game!"
2. **Game Selection** → Grid/List of supported games with icons
3. **Path Selection** → Browse for game installation
   - Show auto-detected paths if found
4. **Confirmation** → "Found Cyberpunk 2077 at [path]"
5. **Complete** → Take user to main app

**Implementation:**

```jsx
export function GameSetup({ onComplete }) {
  const [step, setStep] = useState(1); // 1=welcome, 2=select, 3=path, 4=confirm
  const [selectedGame, setSelectedGame] = useState(null);
  const [gamePath, setGamePath] = useState("");
  const [detectedGames, setDetectedGames] = useState([]);

  // Auto-detect games on mount
  useEffect(() => {
    detectGamesOnSystem();
  }, []);

  // ... render based on step
}
```

**When to show:**

- `first_game_setup === true` in settings
- OR no games configured
- Show as modal overlay on app launch

---

#### 3.3 Main App Updates

**File:** `src/App.jsx`

**Add:**

```jsx
// Game indicator in header
<header>
  <div className="current-game-indicator">
    <img src={currentGame.icon} alt="" />
    <span>{currentGame.name}</span>
  </div>
  {/* ... rest of header */}
</header>;

// Show game setup on first run
{
  showGameSetup && <GameSetup onComplete={() => setShowGameSetup(false)} />;
}
```

---

#### 3.4 ModList Component Updates

**File:** `src/components/ModList.jsx`

**Changes:**

- Mods now filtered by current game automatically (backend does this)
- Show "No mods installed for [Game Name]" when empty
- Maybe add game icon next to each mod (optional)

---

### 4. UI/UX DESIGN

#### 4.1 Settings Layout

```
┌─────────────────────────────────────────┐
│ Settings                                │
├─────────────────────────────────────────┤
│                                         │
│ 🎮 Game Management                      │
│ ┌─────────────────────────────────────┐ │
│ │ Current Game:                       │ │
│ │ [CP2077 Icon] Cyberpunk 2077        │ │
│ └─────────────────────────────────────┘ │
│                                         │
│ Configured Games:                       │
│ ┌─────────────────────────────────────┐ │
│ │ [CP2077] Cyberpunk 2077             │ │
│ │ Path: /Users/.../Cyberpunk 2077     │ │
│ │         [✓ Active] [Remove]         │ │
│ ├─────────────────────────────────────┤ │
│ │ [Skyrim] Skyrim Special Edition     │ │
│ │ Path: /Users/.../Skyrim SE          │ │
│ │         [Switch] [Remove]           │ │
│ └─────────────────────────────────────┘ │
│                                         │
│ Add New Game:                           │
│ [Select game... ▼] [Browse...] [Add]   │
│                                         │
│ ─────────────────────────────────────── │
│                                         │
│ 🔑 NexusMods API Key                    │
│ [API Key input...]                      │
│                                         │
└─────────────────────────────────────────┘
```

---

#### 4.2 First-Run Game Setup

```
Step 1: Welcome
┌─────────────────────────────────────────┐
│          Crossover Mod Manager          │
│                                         │
│         Welcome! Let's get started      │
│                                         │
│  Before we begin, we need to set up     │
│  at least one game.                     │
│                                         │
│            [Get Started →]              │
└─────────────────────────────────────────┘

Step 2: Game Selection
┌─────────────────────────────────────────┐
│        Which game would you like        │
│             to mod first?               │
│                                         │
│  ┌────────────┐  ┌────────────┐        │
│  │ [CP2077]   │  │  [Skyrim]  │        │
│  │ Cyberpunk  │  │  Special   │        │
│  │    2077    │  │  Edition   │        │
│  └────────────┘  └────────────┘        │
│                                         │
│          [More games coming soon]       │
└─────────────────────────────────────────┘

Step 3: Path Selection
┌─────────────────────────────────────────┐
│      Where is Cyberpunk 2077            │
│          installed?                     │
│                                         │
│  Auto-detected:                         │
│  ✓ /Users/.../Cyberpunk 2077            │
│    [Use This Path]                      │
│                                         │
│  Or browse manually:                    │
│  [Browse for Game Folder...]            │
│                                         │
│              [← Back]                   │
└─────────────────────────────────────────┘

Step 4: Confirmation
┌─────────────────────────────────────────┐
│           Setup Complete! ✓             │
│                                         │
│  Game: Cyberpunk 2077                   │
│  Path: /Users/.../Cyberpunk 2077        │
│                                         │
│  You're ready to start modding!         │
│                                         │
│          [Start Modding →]              │
└─────────────────────────────────────────┘
```

---

#### 4.3 Main App Header

```
┌─────────────────────────────────────────┐
│ [🎮 Cyberpunk 2077 ▼] Crossover Mod Mgr │
│                                         │
│  [Mods] [Settings] [Logs]               │
└─────────────────────────────────────────┘
```

---

### 5. FILE CHANGES SUMMARY

#### New Files:

1. `src-tauri/src/game_definitions.rs` - Game metadata and detection
2. `src/components/GameSetup.jsx` - First-run game setup wizard
3. `src/components/GameSetup.css` - Styles for setup wizard

#### Modified Files:

1. `src-tauri/src/settings.rs` - Add multi-game settings structure
2. `src-tauri/src/mod_manager.rs` - Add game_id to mods, per-game databases
3. `src-tauri/src/main.rs` - Add new commands, update existing ones
4. `src/components/Settings.jsx` - Add game management section
5. `src/components/Settings.css` - Add game management styles
6. `src/App.jsx` - Add game setup flow, current game indicator
7. `src/App.css` - Add header styles

---

### 6. DATABASE / STORAGE STRUCTURE

**Before (Single Game):**

```
~/Library/Application Support/com.beneccles.crossover-mod-manager/
├── settings.json
└── mods.json
```

**After (Multi-Game):**

```
~/Library/Application Support/com.beneccles.crossover-mod-manager/
├── settings.json                  (updated structure)
├── mods_cyberpunk2077.json       (game-specific)
├── mods_skyrimse.json            (game-specific)
└── downloads/                     (shared download cache)
    ├── cyberpunk2077/
    └── skyrimse/
```

---

### 7. MIGRATION STRATEGY

**For Existing Users:**

1. **Detect old settings format** on app launch
2. **Migrate automatically:**

   ```rust
   if !settings.games.contains_key("cyberpunk2077") {
       // Old format detected
       let game_config = GameConfig {
           game_path: settings.game_path.clone(),
           game_id: "cyberpunk2077".to_string(),
           mod_staging_path: "downloads/cyberpunk2077".to_string(),
           last_selected: true,
       };
       settings.games.insert("cyberpunk2077".to_string(), game_config);
       settings.current_game = "cyberpunk2077".to_string();
       settings.game_path = String::new(); // Clear old field
   }
   ```

3. **Rename mods.json** → **mods_cyberpunk2077.json**
4. **Show notification:** "Multi-game support added! You can now add more games in Settings."

---

### 8. EDGE CASES & ERROR HANDLING

#### 8.1 Game Switching

- **What if user switches game while mod is downloading?**
  - Cancel download or queue it for the target game
  - Show warning: "Download in progress will be cancelled"

#### 8.2 NXM URL Handling

- **What if NXM URL is for a different game than current?**
  - Option A: Auto-switch to that game (with confirmation)
  - Option B: Show error: "This mod is for [Game], please switch games first"
  - **Recommendation:** Option A with confirmation dialog

#### 8.3 Game Removal

- **What if user removes a game with installed mods?**
  - Keep mod database (mods\_\*.json) but mark game as inactive
  - Show warning: "X mods are installed. Remove anyway?"
  - Allow re-adding game without losing mod history

#### 8.4 Path Validation

- **What if game path becomes invalid (game uninstalled)?**
  - Detect on app launch
  - Show error: "Game path for [Game] is invalid. Please update in Settings."
  - Disable operations for that game until fixed

---

### 9. TESTING PLAN

#### 9.1 Unit Tests

- Game detection logic
- Settings migration
- Path validation
- Game switching logic

#### 9.2 Integration Tests

- First-run setup flow
- Adding multiple games
- Switching between games
- Installing mods to correct game

#### 9.3 Manual Testing Scenarios

1. **Fresh Install**

   - Complete first-run setup
   - Add second game
   - Install mods for each game

2. **Existing User Migration**

   - Start with v0.1.0 settings
   - Upgrade to multi-game version
   - Verify mods still work

3. **Game Switching**

   - Install mods in Game A
   - Switch to Game B
   - Install mods in Game B
   - Switch back to Game A
   - Verify correct mods shown

4. **Error Cases**
   - Invalid game path
   - Missing game executable
   - NXM URL for unconfigured game

---

### 10. ROLLOUT STRATEGY

#### Phase 1: Internal Testing (v0.2.0-beta1)

- Implement full feature
- Test with Cyberpunk 2077 + Skyrim SE
- Gather feedback from core testers

#### Phase 2: Public Beta (v0.2.0-beta2)

- Release to r/macgaming
- Collect bug reports
- Add more games based on demand

#### Phase 3: Stable Release (v0.3.0)

- All major bugs fixed
- Add 2-3 more popular games
- Update documentation

---

### 11. FUTURE ENHANCEMENTS

**Not in initial implementation, but design should allow for:**

1. **Custom Game Definitions**

   - Let users add unsupported games manually
   - Provide game definition template

2. **Game Profiles**

   - Different mod loadouts for same game
   - Quick switching between profiles

3. **Cross-Game Features**

   - Shared mod categories/tags
   - Global mod search across games

4. **Cloud Sync**
   - Sync game configs across devices
   - Backup mod lists

---

## 📊 EFFORT ESTIMATION

### Time Breakdown:

- **Backend (Rust):** 8-12 hours

  - Game definitions: 2h
  - Settings migration: 2h
  - Commands update: 3h
  - Testing: 3h

- **Frontend (React):** 6-8 hours

  - Game setup wizard: 3h
  - Settings updates: 2h
  - Main app updates: 2h
  - Styling: 1h

- **Testing & Polish:** 4-6 hours
  - Integration testing: 2h
  - Bug fixes: 2h
  - Documentation: 2h

**Total: 18-26 hours** (2-3 full days of work)

---

## ✅ ACCEPTANCE CRITERIA

### Must Have:

- [ ] User can set up first game on install
- [ ] User can add multiple games in settings
- [ ] User can switch between games
- [ ] Mods are correctly isolated per game
- [ ] NXM URLs work for configured games
- [ ] Existing users' data migrates automatically
- [ ] Settings persist game configurations

### Should Have:

- [ ] Game auto-detection works
- [ ] Current game shown in UI
- [ ] Confirmation dialogs for destructive actions
- [ ] Error messages for invalid paths

### Nice to Have:

- [ ] Game icons in UI
- [ ] Smooth animations for game switching
- [ ] Game-specific tips/help

---

## 🚀 IMPLEMENTATION ORDER

1. **Start:** Game definitions and detection logic
2. **Then:** Settings structure and migration
3. **Then:** Backend commands (add, switch, remove games)
4. **Then:** First-run setup wizard UI
5. **Then:** Settings page game management
6. **Then:** Main app updates (header, indicators)
7. **Finally:** Testing and polish

---

## 🎯 SUCCESS METRICS

- Users can complete first-run setup in < 2 minutes
- Zero data loss during migration from v0.1.0
- Game switching takes < 1 second
- No cross-contamination of mods between games

---

## APPROVAL CHECKLIST

Before we proceed, please confirm:

- [ ] Architecture approach looks good
- [ ] UI/UX flow makes sense
- [ ] Migration strategy is acceptable
- [ ] Initial games (CP2077 + Skyrim SE) are correct
- [ ] Effort estimation is reasonable
- [ ] Any changes/concerns?

---

**Once approved, I'll start implementation in the order specified above!**
