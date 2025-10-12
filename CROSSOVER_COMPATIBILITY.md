# Crossover/Wine Compatibility Guide

This document outlines potential installation issues specific to running Cyberpunk 2077 mods through Crossover/Wine on macOS.

## Overview

While most Cyberpunk 2077 mods work well under Crossover, the Wine compatibility layer introduces some unique challenges that differ from native Windows installations. This guide helps identify and address these issues.

---

## 🚨 Critical Issues

### 1. Case Sensitivity Conflicts

**Problem**: macOS (HFS+/APFS) is case-insensitive by default, but Wine bottle filesystems can be case-sensitive. Windows is always case-insensitive.

**Risk**:

- Mods packaged on Windows may have inconsistent casing in file paths
- Example: `bin/x64/ModFile.dll` vs `Bin/X64/modfile.dll`
- Game might expect exact case, but mod files might not match
- File lookups can fail even when files exist

**Current Status**: ✅ **IMPLEMENTED** - Full case sensitivity handling with automatic path normalization

**Implementation Details**:

- ✅ **Path Normalization**: All paths automatically normalized to match Cyberpunk 2077's expected directory structure

  - Known directories corrected: `bin`, `x64`, `archive`, `pc`, `mod`, `mods`, `r6`, `scripts`, `engine`, `config`, `red4ext`, `plugins`
  - Example: `Bin/X64/file.dll` → `bin/x64/file.dll`
  - Example: `ARCHIVE/PC/MOD/file.archive` → `archive/pc/mod/file.archive`

- ✅ **Case Mismatch Detection**: Automatically detects files with incorrect casing during installation

  - Compares original paths against normalized game structure
  - Logs individual case mismatches with specific issues
  - Tracks total count of corrected files

- ✅ **Existing File Detection**: Checks for files with different casing already in game directory

  - Warns when replacing files with different case (e.g., `ModFile.dll` → `modfile.dll`)
  - Prevents duplicate files with case variants

- ✅ **User Warnings**: Comprehensive logging for case sensitivity issues

  - Individual warnings per file: "⚠️ Case sensitivity issue detected: Case mismatch: 'Bin/X64/file.dll' should be 'bin/x64/file.dll'"
  - Auto-correction notification: "🔧 Auto-correcting path casing to match game structure"
  - Summary after installation: "📊 Case Sensitivity Summary: X file(s) had incorrect casing and were auto-corrected"
  - Platform-specific tips for macOS users about Windows→macOS compatibility

- ✅ **Case-Insensitive File Operations**: Helper functions available for future use
  - `find_path_case_insensitive()`: Locates files regardless of case
  - `normalize_game_path_component()`: Normalizes individual path components
  - `normalize_game_path()`: Normalizes full file paths
  - `check_case_mismatch()`: Detects and reports case issues

**What Users See**:

```
⚠️ Case sensitivity issue detected: Case mismatch: 'Bin/x64/plugin.dll' should be 'bin/x64/plugin.dll'
🔧 Auto-correcting path casing to match game structure
...
📊 Case Sensitivity Summary: 5 file(s) had incorrect casing and were auto-corrected
✅ All paths normalized to match Cyberpunk 2077's expected directory structure
💡 macOS/Crossover Tip: This is normal when installing mods created on Windows
The mod manager automatically corrects case mismatches to ensure compatibility
```

**User Impact**: HIGH - Can cause "mod not found" errors even when files are present (✅ NOW RESOLVED)

---

### 2. REDmod Launch Parameter Requirement

**Problem**: REDmod mods (official CDPR modding system) require launching the game with the `-modded` parameter.

**Risk**:

- Mod manager correctly installs REDmod mods to `mods/` folder
- But users won't know they need to add `-modded` to launch parameters
- Mods won't load, causing confusion
- No visual indication in-game that REDmod is inactive

**Current Status**: ✅ **IMPLEMENTED** - Files installed correctly with comprehensive launch parameter warnings

**Implementation Details**:

- ✅ Detects REDmod structure during installation (`mods/modname/info.json` pattern)
- ✅ Displays prominent "CRITICAL" warning about `-modded` requirement
- ✅ Provides platform-specific step-by-step instructions:
  - **macOS/Crossover**: Separate instructions for GOG Galaxy, Steam, and Epic Games launchers
  - **Windows**: Native launcher instructions for each platform
- ✅ Clear warning that mods will NOT load without this parameter
- ✅ Helpful tip that parameter only needs to be set once for all REDmod mods

**What Users See**:

```
🎮 REDmod detected! This mod uses the official CDPR modding system.
⚠️ CRITICAL: REDmod mods require the '-modded' launch parameter to work!
Without this parameter, your mod will NOT load and you'll see no effects in-game.
📋 How to add '-modded' parameter in Crossover:
  • GOG Galaxy: Settings → Cyberpunk 2077 → Additional Launch Arguments → Add: -modded
  • Steam: Right-click game → Properties → Launch Options → Add: -modded
  • Epic Games: Library → Cyberpunk 2077 → ⋯ → Manage → Additional Command Line Arguments → Add: -modded
💡 Tip: You only need to set this once, and it applies to all REDmod mods.
```

**User Impact**: CRITICAL - Mods silently fail to load without proper launch configuration

**Example Configuration**:

```
GOG Galaxy → Cyberpunk 2077 → Settings →
Additional Launch Arguments: -modded
```

---

### 3. Wine Registry and DLL Configuration for CET

**Problem**: Cyber Engine Tweaks (CET) requires specific Wine DLL overrides to function properly.

**Risk**:

- CET uses `version.dll` and `winmm.dll` for injection
- These DLLs must be set to "Native then Builtin" in Wine configuration
- Without proper configuration, CET fails to load silently
- Users install CET but see no console in-game

**Current Status**: ✅ Warns users about configuration, ❌ Doesn't automate it

**Solutions**:

**Manual Configuration Steps**:

```
1. Open CrossOver
2. Right-click your game's bottle (e.g., "GOG Galaxy")
3. Select "Wine Configuration" (or Run Command → winecfg)
4. Go to "Libraries" tab
5. In "New override for library" dropdown:
   - Add "version" → Click "Add" → Set to "Native then Builtin"
   - Add "winmm" → Click "Add" → Set to "Native then Builtin"
6. Click "Apply" then "OK"
7. Restart the game launcher (GOG/Steam/Epic)
```

**Advanced Solution** (Future Enhancement):

- Automatically detect bottle path from game installation
- Offer to configure Wine registry automatically
- Provide one-click script to set DLL overrides
- Verify configuration after setting

**User Impact**: CRITICAL - CET completely non-functional without proper Wine configuration

---

## ⚠️ Medium Priority Issues

### 4. Path Separator Conflicts

**Problem**: Mods may contain Windows-style path separators (`\`) in archives or metadata.

**Risk**:

- Archives created on Windows might use backslashes in file paths
- Internal mod configuration might reference Windows paths
- Path normalization might not happen during extraction

**Current Status**: ✅ Mostly handled with path normalization, but should verify archive extraction

**Solutions**:

- Ensure zip extraction properly converts all path separators to forward slashes
- Normalize paths in mod metadata and configuration files
- Replace backslashes in file paths during installation
- Log path conversions for debugging

**User Impact**: MEDIUM - Can cause file placement errors or broken mod references

---

### 5. Symlinks in Crossover Bottles

**Problem**: Some advanced mods use symbolic links, and Wine/Crossover handle symlinks differently than native Windows.

**Risk**:

- Windows junction points and symlinks don't directly translate to Unix symlinks
- Wine bottles might have their own symlink structures that could break
- Symlinks created by mods might not work correctly in Wine
- Game might not follow symlinks properly

**Current Status**: ✅ **IMPLEMENTED** - Automatic symlink detection with user warnings

**Implementation Details**:

- ✅ **Symlink Detection**: Automatically detects symbolic links during mod installation

  - Checks `file_type().is_symlink()` for each file entry
  - Reads symlink targets using `std::fs::read_link()`
  - Tracks all detected symlinks with paths and targets

- ✅ **User Warnings**: Comprehensive warnings about symlink compatibility

  - Lists all detected symlinks with their target paths
  - Explains Wine/Crossover compatibility issues
  - Provides platform-specific advice for macOS users
  - Summary statistics at end of installation

- ✅ **Automatic Handling**: Symlinks are skipped for compatibility
  - Symlinks are NOT installed (compatibility protection)
  - Installation continues normally for other files
  - User is informed that symlinks were skipped

**What Users See**:

```
🔗 Symlink Detection Warning
⚠️  2 symbolic link(s) detected in this mod
  • scripts/init.lua → ../common/init.lua
  • bin/plugin.dll → /shared/plugins/plugin.dll
⚠️  Symlinks may not work correctly in Wine/Crossover environments
ℹ️  Symlinks were NOT installed (skipped for compatibility)
💡 macOS/Crossover Tip: Symlinks are rarely used in Cyberpunk 2077 mods
   If the mod doesn't work, it may rely on symlinks. Check for alternative versions.
   Most mods on NexusMods are packaged without symlinks for compatibility.
📊 Symlink Summary: 2 symlink(s) detected and skipped
```

**Solutions**:

- Detect if mod archive contains symlinks during extraction
- Warn users that symlinks may not work in Wine environment
- Option to dereference symlinks (copy actual files instead of preserving symlinks)
- Document known mods that use symlinks and their compatibility status

**Detection Code**:

```rust
// Check if entry is a symlink
if entry.file_type().is_symlink() {
    warn!("Symlink detected: {} - may not work in Wine", entry.path());
}
```

**User Impact**: MEDIUM - Rare issue, but can cause complete mod failure when encountered

---

### 6. Archive Load Order Management

**Problem**: `.archive` files in Cyberpunk 2077 load alphabetically, and load order determines which mod's assets take precedence.

**Risk**:

- Multiple mods modifying the same game assets will conflict
- Last-loaded mod wins, but users have no control over order
- Mod manager installs files as-is without load order management
- No warning about potential conflicts

**Current Status**: ❌ Not managed

**Solutions**:

- Detect conflicts when installing mods that modify same asset categories
- Allow users to rename `.archive` files to control load order
- Common patterns:
  - Prefix with `0-` to load first
  - Prefix with `z-` to load last
  - Use `a-`, `b-`, `c-` for manual ordering
- Track mod dependencies and suggest load order
- Provide UI to reorder installed archive mods

**Example Warning**:

```
⚠️ Conflict detected: Both 'ModA' and 'ModB' modify vehicle textures.
'ModB' will override 'ModA' assets. To change load order:
- Rename: basegame_ModB.archive → z_basegame_ModB.archive (load last)
```

**User Impact**: MEDIUM - Visual conflicts and unexpected behavior when mods overlap

---

### 7. Unicode and Special Characters in Filenames

**Problem**: Mod names or file paths with non-ASCII characters might not work correctly in Wine.

**Risk**:

- Mods with names like `Modów Polski`, `日本語モッド`, or `Café Racer`
- Wine's character encoding might not handle these correctly
- Files might copy successfully but fail to load in game
- Path lookups can fail with encoding mismatches

**Current Status**: ❌ Not explicitly handled

**Solutions**:

- Detect non-ASCII characters in mod filenames during installation
- Warn users about potential compatibility issues
- Offer to sanitize filenames to ASCII-safe characters
- Transliterate common characters (ö→o, é→e, etc.)
- Maintain mapping of original to sanitized names for uninstallation

**Sanitization Strategy**:

```rust
// Replace problematic characters
fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            'á'..='ž' => 'a',  // Accented chars → base chars
            '日'..='龯' => '_',  // CJK → underscore
            c if c.is_ascii_alphanumeric() || c == '-' || c == '_' => c,
            _ => '_'
        })
        .collect()
}
```

**User Impact**: MEDIUM - Mod installs but doesn't load, difficult to debug

---

## 📋 Lower Priority Issues

### 8. File Permissions in Wine Bottles

**Problem**: Wine bottles have their own permission system that doesn't perfectly match macOS or Windows.

**Risk**:

- Files might be copied without execute permissions where needed
- DLLs might need specific permissions to load properly
- Config files might be read-only when they should be writable
- Permission issues can cause silent failures

**Current Status**: ❌ Not explicitly handled

**Solutions**:

- Set appropriate Unix permissions after file copy on macOS
- DLL files: `0o644` (rw-r--r--)
- Config files: `0o644` (rw-r--r--)
- Directories: `0o755` (rwxr-xr-x)

**Implementation**:

```rust
#[cfg(target_os = "macos")]
{
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(&install_path)?.permissions();
    perms.set_mode(0o644); // Read/write for owner, read for others
    fs::set_permissions(&install_path, perms)?;
}
```

**User Impact**: LOW - Rare issue, mostly affects advanced mods

---

### 9. Long Path Names (macOS Limits)

**Problem**: macOS has a 1024-character path limit (NAME_MAX = 255, PATH_MAX = 1024).

**Risk**:

- Deep Wine bottle paths combined with deep game structures can approach limits
- Example path length:
  ```
  /Users/username/Library/Application Support/CrossOver/Bottles/
  GOG Galaxy/drive_c/Program Files (x86)/GOG Galaxy/Games/
  Cyberpunk 2077/archive/pc/mod/VeryLongModNameWithLotsOfCharacters/
  subfolder/deepstructure/file.archive
  ```
- Path operations fail when limit exceeded
- Error messages are often cryptic

**Current Status**: ❌ Not handled

**Solutions**:

- Calculate full installation path length before copying files
- Warn if path approaches 900 characters (safety margin)
- Suggest shortening bottle name or moving bottle location
- Offer to use shorter mod folder names

**Check Implementation**:

```rust
fn check_path_length(path: &Path) -> Result<(), String> {
    let path_str = path.to_string_lossy();
    if path_str.len() > 900 {
        return Err(format!(
            "Path too long ({} chars). Maximum safe length is 900 characters.\n\
             Consider using a shorter bottle name or game path.",
            path_str.len()
        ));
    }
    Ok(())
}
```

**User Impact**: LOW - Rare, but causes installation failure when encountered

---

### 10. Disk Space in Bottle vs macOS

**Problem**: Wine bottles can appear to have different available space than the host filesystem.

**Risk**:

- macOS might have plenty of space, but Wine bottle might report full
- Large mod downloads fail unexpectedly
- No clear error message about space issues

**Current Status**: ❌ Not checked

**Solutions**:

- Check available disk space before downloading large mods
- Use macOS native disk space check, not Wine's reported space
- Warn if available space < (mod size \* 3) for extraction buffer
- Clean up temp files before installation

**Implementation**:

```rust
use std::fs;

fn check_disk_space(path: &Path, required_bytes: u64) -> Result<(), String> {
    let metadata = fs::metadata(path)?;
    let available = fs2::available_space(path)?;

    if available < required_bytes * 3 {
        return Err(format!(
            "Insufficient disk space. Required: {} MB, Available: {} MB",
            required_bytes / 1_000_000,
            available / 1_000_000
        ));
    }
    Ok(())
}
```

**User Impact**: LOW - Causes installation failure, but with clear error

---

### 11. Windows Version Emulation

**Problem**: Some mods check Windows version and might fail if Wine reports unexpected version.

**Risk**:

- RED4ext and CET might check for specific Windows versions
- Default Wine settings might report Windows 7 or 10
- Mods expecting Windows 11 might refuse to install
- Version checks might be hardcoded

**Current Status**: ❌ Not addressed

**Solutions**:

- Document recommended Wine version settings for Crossover bottles
- Suggest setting Wine to emulate Windows 10 (most compatible)
- Configuration: `winecfg → Applications → Windows Version → Windows 10`
- Add to troubleshooting documentation

**Recommended Settings**:

```
CrossOver → Bottle → Wine Configuration → Applications
Windows Version: Windows 10
```

**User Impact**: LOW - Rare, mostly affects very new or old mods

---

### 12. Temporary File Cleanup

**Problem**: Downloads and extracted files in temp directories might not get cleaned up properly.

**Risk**:

- Failed installations leave large temp files
- Disk space gradually fills up with abandoned downloads
- Multiple failed attempts = multiple orphaned files
- No automatic cleanup mechanism

**Current Status**: ✅ Partial cleanup, but might fail silently on errors

**Solutions**:

- Ensure cleanup happens even on error paths (use defer/RAII pattern)
- Implement cleanup on application startup for orphaned temp files
- Add manual "Clean Temp Files" button in settings
- Log cleanup operations for debugging

**Robust Cleanup Pattern**:

```rust
struct TempFileGuard(PathBuf);

impl Drop for TempFileGuard {
    fn drop(&mut self) {
        fs::remove_file(&self.0).ok(); // Cleanup on scope exit
    }
}

// Usage
let _guard = TempFileGuard(temp_file_path);
// File automatically cleaned up when function returns
```

**User Impact**: LOW - Wastes disk space but doesn't break functionality

---

## 🎯 Priority Implementation Roadmap

### Phase 1: Critical Fixes (Immediate) - ✅ COMPLETED

1. ✅ **REDmod launch parameter detection and warning** - **COMPLETED**

   - ✅ Detection implemented during installation
   - ✅ Prominent "CRITICAL" warning displayed
   - ✅ Platform-specific launcher instructions provided
   - ✅ Clear user guidance prevents silent mod failures

2. ✅ **Case sensitivity handling** - **COMPLETED**

   - ✅ Case-insensitive file existence checks implemented
   - ✅ Full path normalization for all game directories
   - ✅ Case mismatch detection with detailed logging
   - ✅ Automatic correction of incorrect casing
   - ✅ Detection of existing files with different case
   - ✅ Summary warnings with platform-specific tips
   - ✅ Helper functions for future case-insensitive operations

3. **Wine/CET configuration documentation** - ✅ DOCUMENTED
   - ✅ Comprehensive RED4ext setup guide created (RED4EXT_COMPATIBILITY.md)
   - ✅ Step-by-step Crossover configuration instructions
   - ✅ CET warnings included during installation
   - Future: Add bottle detection (automation)

### Phase 2: Medium Priority (Next Release)

4. **Symlink detection and handling**

   - Warn about symlinks in mod archives
   - Option to dereference during installation

5. **Archive load order conflict detection**

   - Detect overlapping archive mods
   - Basic conflict warnings

6. **Unicode filename sanitization**
   - Detect and warn about non-ASCII filenames
   - Optional sanitization during installation

### Phase 3: Polish and Edge Cases (Future)

7. Path length validation
8. File permissions management
9. Disk space checking
10. Improved temp file cleanup

---

## Testing Checklist

When testing mods on Crossover, verify:

- [ ] Case-sensitive filesystem handling
- [ ] REDmod mods show launch parameter warning
- [ ] CET configuration instructions are clear
- [ ] Path separators are normalized
- [ ] Long paths are validated
- [ ] Unicode filenames are handled
- [ ] Load order conflicts are detected
- [ ] Temp files are cleaned up on success and failure
- [ ] File permissions are appropriate
- [ ] Disk space is sufficient before download

---

## Known Compatible Mod Types

### ✅ Excellent Compatibility

- **Archive Mods** (`.archive` files) - Pure asset replacement, no runtime code
- **Redscript Mods** (`.reds` files) - Script-based, excellent Wine compatibility
- **REDmod System** - Official CDPR system, works well with `-modded` parameter
- **Texture/Model Replacements** - Direct asset swaps, no compatibility issues

### ⚠️ Good Compatibility (with configuration)

- **Cyber Engine Tweaks (CET)** - Works well after Wine DLL configuration
- **TweakXL** - Configuration-based tweaks, generally compatible
- **ArchiveXL** - Archive loading extension, good compatibility

### ❌ Limited Compatibility

- **RED4ext** - Native code injection, often fails in Wine
- **Native DLL Mods** - Windows-specific code, Wine translation issues
- **Anti-cheat Mods** - Kernel-level code incompatible with Wine

---

## Getting Help

If you encounter Crossover-specific issues:

1. **Check mod type**: Refer to compatibility list above
2. **Review logs**: Look for case sensitivity or path errors
3. **Verify Wine configuration**: Ensure DLL overrides are set for CET
4. **Check launch parameters**: REDmod requires `-modded` flag
5. **Community resources**:
   - [WineHQ AppDB](https://appdb.winehq.org/) - Wine compatibility database
   - [CodeWeavers Forums](https://www.codeweavers.com/support/forums) - Crossover-specific help
   - [r/Crossover](https://reddit.com/r/Crossover) - Community support

---

## Contributing

Found a Crossover-specific issue not listed here? Please:

1. Document the issue with specific mod name and version
2. Include Crossover version and macOS version
3. Provide error logs if available
4. Submit issue or PR to improve this guide

---

_Last updated: October 9, 2025_
_For RED4ext-specific issues, see: [RED4EXT_COMPATIBILITY.md](RED4EXT_COMPATIBILITY.md)_
