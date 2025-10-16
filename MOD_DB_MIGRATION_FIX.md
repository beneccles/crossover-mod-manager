# Mod Database Migration Fix

**Issue**: Multi-game support broke existing Cyberpunk 2077 installations

**Root Cause**:

- Old system used `mods.json` for all mods
- New system uses `mods_cyberpunk2077.json`, `mods_skyrim.json`, etc.
- Migration existed for `settings.json` but NOT for the mod database
- Users with existing CP2077 mods lost access to their mod list

## Solution Implemented

Added automatic migration in `mod_manager.rs`:

### Changes Made

**File**: `src-tauri/src/mod_manager.rs`

1. **Modified `new_for_game()` constructor** (lines ~79-92)

   - Added check for Cyberpunk 2077 game
   - If `mods_cyberpunk2077.json` doesn't exist, trigger migration
   - Migration happens once, automatically, on first load

2. **Added `migrate_legacy_database()` function** (lines ~119-144)
   - Checks for legacy `mods.json` file
   - Reads all existing mods
   - Sets `game_id = "cyberpunk2077"` for any mods missing it
   - Writes to new `mods_cyberpunk2077.json` file
   - Preserves all mod data (files, conflicts, timestamps, etc.)
   - Keeps original `mods.json` intact (doesn't delete)

## Migration Process

```
User starts app with existing Cyberpunk 2077 mods
  ↓
ModManager::new() called
  ↓
ModManager::new_for_game("cyberpunk2077")
  ↓
Check: Does mods_cyberpunk2077.json exist?
  ↓ NO
Call migrate_legacy_database()
  ↓
Read legacy mods.json
  ↓
For each mod: Set game_id = "cyberpunk2077"
  ↓
Write to mods_cyberpunk2077.json
  ↓
Load and return mods
  ↓
✅ User sees all their existing mods!
```

## What Gets Migrated

- ✅ Mod name, version, author
- ✅ Mod description
- ✅ NexusMods mod_id and file_id
- ✅ Enabled/disabled status
- ✅ All installed file paths
- ✅ File conflict information
- ✅ Install timestamps
- ✅ **NEW**: game_id set to "cyberpunk2077"

## Edge Cases Handled

1. **Legacy file doesn't exist**: No migration, starts fresh
2. **New file already exists**: No migration, uses existing
3. **Migration fails**: Falls back to empty mod list (safe)
4. **Mods already have game_id**: Preserves existing value
5. **Multiple app starts**: Migration only runs once

## Testing

Verified with terminal output:

```
Migrated legacy mods.json to mods_cyberpunk2077.json
```

**Status**: ✅ Working
**Build**: ✅ Compiles successfully
**Warnings**: None critical (only unused functions)

## Future Considerations

- Legacy `mods.json` file is kept (not deleted)
- Could add cleanup to remove old file after successful migration
- Migration is one-way (no rollback needed)
- Other games (Skyrim, etc.) don't need migration (fresh installs)

---

**Fixed in**: Multiple Game Support Feature Branch  
**Date**: October 15, 2025  
**Commit**: Ready for deployment
