# RED4ext Compatibility Guide

## What is RED4ext?

RED4ext is a native code extension framework for Cyberpunk 2077 that allows advanced mods to hook directly into the game's engine. It's a powerful but complex modding framework.

## ⚠️ Compatibility Status for macOS/Crossover Users

**RED4ext CAN work on Crossover with proper configuration, but requires more setup than on Windows.**

### ✅ Good News: RED4ext Works on Crossover (With Setup)

RED4ext can successfully run on macOS through Crossover if you:
- Install Visual C++ Redistributables in the bottle
- Configure Wine to Windows 10 mode
- Set proper DLL overrides for `version.dll`
- Follow the detailed installation steps below

### ⚠️ Why RED4ext Is More Complex on macOS

1. **Native Code Injection**: RED4ext uses advanced Windows-specific techniques to inject code into the game process
2. **Wine Limitations**: Wine's translation layer doesn't perfectly emulate all Windows API calls that RED4ext requires
3. **DLL Loading Issues**: Complex dependency chains and loading order problems in Wine environments
4. **Memory Management**: Native code memory allocation patterns that don't translate well through Wine

### Common Error Messages

If you see errors like:

- "RED4ext could not be loaded"
- "Module not found"
- "Failed to initialize RED4ext"
- Game crashes on startup after installing RED4ext

These are typical signs that RED4ext is incompatible with your Wine/Crossover setup.

## Recommended Alternatives

### For Most Users (macOS + Crossover)

1. **Redscript Mods**: Use `.reds` script-based mods instead
2. **Cyber Engine Tweaks (CET)**: Better Wine compatibility for scripting
3. **Archive Mods**: Pure asset replacement mods (`.archive` files)
4. **REDmod**: Official CDPR modding system

### For Windows Users

RED4ext generally works well on native Windows installations with:

- Visual C++ Redistributable 2019 or newer
- Proper antivirus exclusions
- Administrator privileges for first-time setup

## Installation Notes

Our mod manager will:

- ✅ **Detect RED4ext** mods during installation
- ⚠️ **Warn about compatibility** issues on macOS
- 📁 **Install files correctly** to the proper locations:
  - `RED4ext.dll` → `bin/x64/RED4ext.dll`
  - Plugin DLLs → `red4ext/plugins/`
  - Configuration files → `red4ext/config/`

## Troubleshooting Steps

### If RED4ext Fails to Load

1. **Check Visual C++ Redistributables** (Windows):

   ```
   Download from Microsoft: VC++ Redistributable 2019 x64
   ```

2. **Verify File Locations**:

   ```
   Game Directory/
   ├── bin/x64/RED4ext.dll          ← Core RED4ext
   ├── red4ext/
   │   ├── plugins/                 ← Plugin DLLs go here
   │   └── config/                  ← Config files
   ```

3. **Check Game Version Compatibility**:

   - Ensure RED4ext version matches your Cyberpunk 2077 version
   - Check mod description for supported game versions

4. **Run as Administrator** (Windows):
   - Some RED4ext features require elevated privileges

### For Crossover/Wine Users on macOS

RED4ext CAN work on Crossover with proper setup, though it's more complex than on Windows. Here's how:

#### Step 1: Locate Your Cyberpunk 2077 Installation

1. In CrossOver, right-click your Cyberpunk 2077 bottle
2. Select **"Open C: Drive"**
3. Navigate to where the game is installed:
   - **GOG Galaxy**: `Program Files (x86)/GOG Galaxy/Games/Cyberpunk 2077`
   - **Steam**: `Program Files (x86)/Steam/steamapps/common/Cyberpunk 2077`
   - **Epic Games**: `Program Files/Epic Games/Cyberpunk2077`

#### Step 2: Install RED4ext Core

1. Download the latest RED4ext from [GitHub releases](https://github.com/WopsS/RED4ext/releases)
2. Extract the RED4ext archive
3. Copy the contents to your Cyberpunk 2077 game directory:
   - `version.dll` → Game root directory (alongside `bin/` and `engine/` folders)
   - `red4ext/` folder → Game root directory

**Expected Structure**:
```
Cyberpunk 2077/
├── bin/
├── engine/
├── red4ext/              ← RED4ext folder
│   ├── plugins/
│   └── config/
├── version.dll           ← RED4ext loader
└── Cyberpunk2077.exe
```

#### Step 3: Configure Crossover Bottle

**Critical**: RED4ext requires specific Windows settings in Crossover:

1. Open CrossOver
2. Right-click your game's bottle → **"Wine Configuration"** (or Run Command → `winecfg`)
3. Go to **"Applications"** tab
4. Set **"Windows Version"** to **"Windows 10"** (RED4ext works best with this)
5. Go to **"Libraries"** tab
6. Add library override for `version`:
   - In "New override for library" dropdown, type: `version`
   - Click **"Add"**
   - Set to **"Native then Builtin"**
7. Click **"Apply"** then **"OK"**

#### Step 4: Install Visual C++ Redistributables

RED4ext requires Visual C++ Runtime libraries:

**Option A - Using CrossOver's Built-in Installer** (Recommended):
```
CrossOver → Bottle → Install Software → 
Search for "Visual C++" → 
Install "Microsoft Visual C++ 2019 Redistributable (x64)"
```

**Option B - Using Winetricks**:
```bash
# If you have Winetricks installed
winetricks vcrun2019
# Or for newer versions
winetricks vcrun2022
```

#### Step 5: Verify Installation

1. Launch Cyberpunk 2077 through CrossOver
2. After the game starts (or crashes), check for the `red4ext/logs/` folder
3. Look for log files with timestamps (e.g., `red4ext_2025-10-10_14-30-00.log`)
4. Open the latest log file and check for:
   - ✅ `"RED4ext loaded successfully"` or similar success messages
   - ❌ Error messages indicating what failed

**Example Success Log**:
```
[2025-10-10 14:30:01] RED4ext (v1.xx.x) loaded
[2025-10-10 14:30:01] Cyberpunk 2077 v2.x detected
[2025-10-10 14:30:01] Loading plugins...
```

#### Step 6: Install RED4ext Plugins (Optional)

Once RED4ext core is working, you can install plugins:

1. Extract the plugin archive
2. Copy `.dll` files to `red4ext/plugins/` folder
3. Copy configuration files (`.toml`, `.ini`) to `red4ext/config/` folder
4. Launch the game and check logs to verify plugin loaded

#### Common macOS/Crossover Issues

**Issue**: RED4ext doesn't load, no logs appear
- **Solution**: Install vcrun2019 or vcrun2022 in the bottle
- **Solution**: Verify `version.dll` is in the game root directory (not in `bin/x64/`)
- **Solution**: Check bottle is set to Windows 10

**Issue**: Game crashes on startup after installing RED4ext
- **Solution**: Check RED4ext version matches your game version
- **Solution**: Try removing all plugins and test RED4ext core alone
- **Solution**: Install newer Visual C++ Redistributables

**Issue**: RED4ext loads but plugins don't work
- **Solution**: Check plugin compatibility with your RED4ext version
- **Solution**: Verify plugin files are in `red4ext/plugins/` (not in subfolders)
- **Solution**: Check `red4ext/logs/` for plugin-specific errors

**Issue**: Permission errors in logs
- **Solution**: Ensure the game directory has write permissions
- **Solution**: Try running CrossOver with full disk access in macOS settings

#### Performance Notes

- RED4ext on Crossover may have slightly lower performance than native Windows
- Complex plugins with heavy game engine interaction may be less stable
- Some bleeding-edge RED4ext features may not work through Wine

#### When to Use Alternatives

Consider using alternatives if:
- ❌ RED4ext consistently crashes your game
- ❌ You need multiple complex RED4ext plugins
- ❌ The mod you want has Redscript or CET alternatives
- ✅ You're okay with slightly more setup complexity

#### Quick Troubleshooting Checklist

- [ ] RED4ext version matches Cyberpunk 2077 version
- [ ] `version.dll` is in game root directory
- [ ] `red4ext/` folder exists in game root directory
- [ ] Bottle set to Windows 10
- [ ] `version` library override set to "Native then Builtin"
- [ ] Visual C++ 2019/2022 Redistributables installed in bottle
- [ ] `red4ext/logs/` folder exists after launching game
- [ ] Latest CrossOver version installed
- [ ] Game launches without RED4ext first (to verify base game works)

## Alternative Modding Options

### Redscript (.reds files)

- ✅ Excellent Wine compatibility
- ✅ Active development community
- ✅ Many powerful mods available
- 📁 Install to: `r6/scripts/`

### Cyber Engine Tweaks

- ✅ Good Wine compatibility with configuration
- ✅ Lua scripting support
- ✅ In-game console
- 🔧 Requires Wine library configuration

### Archive Mods

- ✅ Perfect compatibility
- ✅ Asset replacement and additions
- ✅ No runtime dependencies
- 📁 Install to: `archive/pc/mod/`

## Getting Help

If you're having RED4ext issues:

1. **Check Mod Compatibility**: Look for Redscript or CET versions
2. **Community Forums**: Visit r/cyberpunkgame or Nexus Mods discussions
3. **Mod Manager Logs**: Check installation logs for specific errors
4. **Wine AppDB**: Check Wine Application Database for latest compatibility reports

## Summary

- 🖥️ **Windows**: RED4ext generally works with proper setup
- 🍎 **macOS/Crossover**: Limited compatibility, use alternatives when possible
- 🔧 **Always check**: Mod descriptions for alternative versions
- 📋 **Best practice**: Try Redscript or CET-based mods first on macOS

---

_Last updated: October 9, 2025_
