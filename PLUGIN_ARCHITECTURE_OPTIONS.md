# Game Path Handler: Plugin Architecture Options

## Current State

Currently, we have hardcoded game-specific functions (`determine_install_path_cyberpunk`, `determine_install_path_skyrim`) with a match statement dispatcher. This requires modifying core code for each new game.

---

## Option 1: Trait-Based Plugin System ⭐ **RECOMMENDED**

### Architecture

Define a trait that each game implements, allowing games to define their own path resolution logic as self-contained modules.

### Structure

```rust
// In game_definitions.rs
pub trait GamePathResolver {
    fn resolve_install_path(
        &self,
        game_dir: &Path,
        relative_path: &Path,
        normalized_path: &Path,
        path_str: &str,
        file_name: &str,
    ) -> Result<PathBuf, String>;
}

pub struct GameDefinition {
    pub id: String,
    pub name: String,
    pub nexus_domain: String,
    pub detection_files: Vec<String>,
    pub mod_directories: Vec<String>,
    pub supports_load_order: bool,
    pub resolver: Box<dyn GamePathResolver + Send + Sync>,
}
```

### Implementation Example

```rust
// Cyberpunk resolver
struct Cyberpunk2077Resolver;

impl GamePathResolver for Cyberpunk2077Resolver {
    fn resolve_install_path(&self, game_dir: &Path, ...) -> Result<PathBuf, String> {
        // All CP2077-specific logic here
        if path_str.starts_with("archive/") {
            return Ok(game_dir.join(normalized_path));
        }
        // ... rest of logic
    }
}

// Skyrim resolver
struct SkyrimResolver;

impl GamePathResolver for SkyrimResolver {
    fn resolve_install_path(&self, game_dir: &Path, ...) -> Result<PathBuf, String> {
        // All Skyrim-specific logic here
        if path_str.starts_with("data/") {
            return Ok(game_dir.join(normalized_path));
        }
        // ... rest of logic
    }
}

// In determine_install_path_for_file()
fn determine_install_path_for_file(...) -> Result<PathBuf, String> {
    let game_def = get_game_by_id(game_id)?;
    game_def.resolver.resolve_install_path(
        game_dir,
        relative_path,
        &normalized_path,
        &path_str,
        &file_name,
    )
}
```

### Pros

- ✅ **True plugin architecture** - Each game is a self-contained module
- ✅ **Type-safe** - Compiler enforces implementation
- ✅ **Easy to extend** - Just implement the trait for new games
- ✅ **No match statements** - Automatic dispatch through trait
- ✅ **Testable** - Each resolver can be unit tested independently
- ✅ **No runtime overhead** - Trait calls are zero-cost abstractions

### Cons

- ⚠️ Requires Rust trait objects (slightly more complex)
- ⚠️ Cannot serialize resolvers (but definitions can be)
- ⚠️ Need `Box<dyn Trait>` for dynamic dispatch

### Migration Effort

**Medium** - Requires refactoring existing functions into trait implementations, but the logic stays the same.

---

## Option 2: Rule-Based Configuration System

### Architecture

Define games through declarative configuration rules instead of imperative code. Each game has a JSON-like rule set that the engine interprets.

### Structure

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathRule {
    pub condition: PathCondition,
    pub action: PathAction,
    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PathCondition {
    StartsWithDir(String),
    FileExtension(String),
    ContainsInPath(String),
    FileNameMatches(String),
    And(Vec<PathCondition>),
    Or(Vec<PathCondition>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PathAction {
    PreservePath,                          // Keep original structure
    MoveToDir(String),                     // Move to specific directory
    MoveToSubdir(String, String),          // e.g., Data/SKSE/Plugins
    RootWithFilename,                      // Game root + filename only
}

pub struct GameDefinition {
    pub id: String,
    pub name: String,
    pub nexus_domain: String,
    pub detection_files: Vec<String>,
    pub mod_directories: Vec<String>,
    pub supports_load_order: bool,
    pub path_rules: Vec<PathRule>,
}
```

### Implementation Example

```rust
// Cyberpunk 2077 rules
vec![
    PathRule {
        condition: PathCondition::StartsWithDir("archive/".into()),
        action: PathAction::PreservePath,
        priority: 100,
    },
    PathRule {
        condition: PathCondition::FileExtension(".archive".into()),
        action: PathAction::MoveToSubdir("archive/pc".into(), "mod".into()),
        priority: 50,
    },
    PathRule {
        condition: PathCondition::FileNameMatches("version.dll".into()),
        action: PathAction::RootWithFilename,
        priority: 200,
    },
]

// Skyrim rules
vec![
    PathRule {
        condition: PathCondition::StartsWithDir("data/".into()),
        action: PathAction::PreservePath,
        priority: 100,
    },
    PathRule {
        condition: PathCondition::Or(vec![
            PathCondition::FileExtension(".esp".into()),
            PathCondition::FileExtension(".esm".into()),
        ]),
        action: PathAction::MoveToDir("Data".into()),
        priority: 90,
    },
    PathRule {
        condition: PathCondition::And(vec![
            PathCondition::FileExtension(".dll".into()),
            PathCondition::Or(vec![
                PathCondition::ContainsInPath("skse".into()),
                PathCondition::ContainsInPath("plugins".into()),
            ]),
        ]),
        action: PathAction::MoveToSubdir("Data/SKSE".into(), "Plugins".into()),
        priority: 95,
    },
]
```

### Pros

- ✅ **No code changes needed** - Rules can be defined in JSON/TOML files
- ✅ **User-modifiable** - Advanced users could customize rules
- ✅ **Highly flexible** - Complex conditions with And/Or logic
- ✅ **Declarative** - Easier to understand than imperative code
- ✅ **Could support game profiles** - Community-contributed rule sets

### Cons

- ⚠️ **Complex implementation** - Need a robust rule engine
- ⚠️ **Harder to debug** - Rule conflicts and priorities can be tricky
- ⚠️ **Performance overhead** - Runtime rule evaluation
- ⚠️ **Less type safety** - Rules are data, not code
- ⚠️ **Overkill for current needs** - We only have 3 games

### Migration Effort

**High** - Requires building an entire rule engine and converting all logic to rules.

---

## Option 3: Strategy Pattern with Registry

### Architecture

Use the Strategy pattern with a runtime registry that maps game IDs to strategy implementations.

### Structure

```rust
// Define the strategy interface
pub type PathResolverFn = fn(
    game_dir: &Path,
    relative_path: &Path,
    normalized_path: &Path,
    path_str: &str,
    file_name: &str,
) -> Result<PathBuf, String>;

// Global registry
lazy_static! {
    static ref PATH_RESOLVERS: Mutex<HashMap<String, PathResolverFn>> = {
        let mut m = HashMap::new();
        m.insert("cyberpunk2077".to_string(), resolve_cyberpunk2077_path as PathResolverFn);
        m.insert("skyrim".to_string(), resolve_skyrim_path as PathResolverFn);
        m.insert("skyrimse".to_string(), resolve_skyrim_path as PathResolverFn);
        Mutex::new(m)
    };
}

// Register a new game resolver
pub fn register_game_resolver(game_id: &str, resolver: PathResolverFn) {
    let mut registry = PATH_RESOLVERS.lock().unwrap();
    registry.insert(game_id.to_string(), resolver);
}
```

### Implementation Example

```rust
// Separate files for each game
// games/cyberpunk2077.rs
pub fn resolve_cyberpunk2077_path(
    game_dir: &Path,
    relative_path: &Path,
    normalized_path: &Path,
    path_str: &str,
    file_name: &str,
) -> Result<PathBuf, String> {
    // All CP2077 logic here
}

// games/skyrim.rs
pub fn resolve_skyrim_path(
    game_dir: &Path,
    relative_path: &Path,
    normalized_path: &Path,
    path_str: &str,
    file_name: &str,
) -> Result<PathBuf, String> {
    // All Skyrim logic here
}

// In determine_install_path_for_file()
fn determine_install_path_for_file(...) -> Result<PathBuf, String> {
    let registry = PATH_RESOLVERS.lock().unwrap();
    let resolver = registry.get(game_id)
        .ok_or_else(|| format!("No path resolver for game: {}", game_id))?;

    resolver(game_dir, relative_path, &normalized_path, &path_str, &file_name)
}
```

### Pros

- ✅ **Simple to understand** - Just function pointers in a map
- ✅ **Easy to extend** - Register new functions at runtime
- ✅ **Modular** - Each game in its own file
- ✅ **Runtime registration** - Could theoretically load from plugins
- ✅ **No trait complexity** - Just plain functions

### Cons

- ⚠️ **Less type safety** - Function pointers can be error-prone
- ⚠️ **Manual registration** - Have to remember to add to registry
- ⚠️ **Global state** - Mutex on a static HashMap
- ⚠️ **Not truly "pluggable"** - Still needs compilation
- ⚠️ **Less Rust-idiomatic** - Traits are preferred in Rust

### Migration Effort

**Low** - Can extract existing functions with minimal changes, just add registry.

---

## Comparison Matrix

| Feature                | Option 1: Traits | Option 2: Rules | Option 3: Registry |
| ---------------------- | ---------------- | --------------- | ------------------ |
| **Complexity**         | Medium           | High            | Low                |
| **Type Safety**        | ★★★★★            | ★★☆☆☆           | ★★★☆☆              |
| **Extensibility**      | ★★★★★            | ★★★★★           | ★★★★☆              |
| **Performance**        | ★★★★★            | ★★★☆☆           | ★★★★★              |
| **Rust Idiomatic**     | ★★★★★            | ★★★☆☆           | ★★★☆☆              |
| **User Customization** | ★☆☆☆☆            | ★★★★★           | ★☆☆☆☆              |
| **Testability**        | ★★★★★            | ★★★★☆           | ★★★★☆              |
| **Migration Effort**   | Medium           | High            | Low                |
| **Future-Proof**       | ★★★★★            | ★★★★☆           | ★★★☆☆              |

---

## 🏆 Recommendation: Option 1 (Trait-Based Plugin System)

### Why Option 1 is Best

1. **Rust Best Practices**: Traits are the idiomatic Rust way to achieve polymorphism and plugin-like behavior

2. **Type Safety**: The compiler ensures every game has a valid path resolver at compile time

3. **Zero Runtime Overhead**: Trait dispatch is optimized away by the compiler in most cases

4. **Clean Architecture**: Each game becomes a true "plugin" module with all its logic self-contained

5. **Easy Testing**: Each resolver can be unit tested independently without touching main.rs

6. **Scalability**: Adding new games becomes trivial - just implement the trait

7. **No Global State**: Unlike Option 3, no mutexes or static variables needed

8. **Better Than Option 2**: For our use case (3-10 games), a rule engine is overkill and adds unnecessary complexity

### Implementation Roadmap

**Phase 1**: Define the trait (30 min)

```rust
pub trait GamePathResolver {
    fn resolve_install_path(...) -> Result<PathBuf, String>;
}
```

**Phase 2**: Extract existing logic into implementations (1 hour)

- Create `Cyberpunk2077Resolver`
- Create `SkyrimResolver`
- Move existing function bodies into trait methods

**Phase 3**: Update GameDefinition (15 min)

- Add `resolver: Box<dyn GamePathResolver>` field
- Update constructors

**Phase 4**: Simplify main.rs (15 min)

- Replace match statement with single trait call
- Remove `determine_install_path_cyberpunk` and `determine_install_path_skyrim`

**Total Time**: ~2 hours

### Future Benefits

- **Community Contributions**: Easy for others to add games
- **Game Mods**: Could separate into a `games/` directory
- **Configuration**: Could add a `get_config()` method to return game-specific settings
- **Validation**: Could add `validate_mod()` method for pre-install checks
- **Post-Processing**: Could add hooks for game-specific post-install steps

---

## Sample Code for Option 1

```rust
// src-tauri/src/game_definitions.rs

use std::path::{Path, PathBuf};

pub trait GamePathResolver: Send + Sync {
    fn resolve_install_path(
        &self,
        game_dir: &Path,
        relative_path: &Path,
        normalized_path: &Path,
        path_str: &str,
        file_name: &str,
    ) -> Result<PathBuf, String>;
}

pub struct GameDefinition {
    pub id: String,
    pub name: String,
    pub nexus_domain: String,
    pub detection_files: Vec<String>,
    pub mod_directories: Vec<String>,
    pub supports_load_order: bool,
    pub resolver: Box<dyn GamePathResolver>,
}

// Cyberpunk 2077 Resolver
pub struct Cyberpunk2077Resolver;

impl GamePathResolver for Cyberpunk2077Resolver {
    fn resolve_install_path(
        &self,
        game_dir: &Path,
        relative_path: &Path,
        normalized_path: &Path,
        path_str: &str,
        file_name: &str,
    ) -> Result<PathBuf, String> {
        // Move all determine_install_path_cyberpunk logic here
        if path_str.starts_with("mods/") || path_str.starts_with("mods\\") {
            return Ok(game_dir.join(normalized_path));
        }
        // ... rest of CP2077 logic
        Ok(game_dir.join(normalized_path))
    }
}

// Skyrim Resolver
pub struct SkyrimResolver;

impl GamePathResolver for SkyrimResolver {
    fn resolve_install_path(
        &self,
        game_dir: &Path,
        relative_path: &Path,
        normalized_path: &Path,
        path_str: &str,
        file_name: &str,
    ) -> Result<PathBuf, String> {
        // Move all determine_install_path_skyrim logic here
        if path_str.starts_with("data/") || path_str.starts_with("data\\") {
            return Ok(game_dir.join(normalized_path));
        }
        // ... rest of Skyrim logic
        Ok(game_dir.join("Data").join(normalized_path))
    }
}

impl GameDefinition {
    pub fn cyberpunk2077() -> Self {
        Self {
            id: "cyberpunk2077".to_string(),
            name: "Cyberpunk 2077".to_string(),
            nexus_domain: "cyberpunk2077".to_string(),
            detection_files: vec![
                "bin/x64/Cyberpunk2077.exe".to_string(),
                "Cyberpunk2077.exe".to_string(),
            ],
            mod_directories: vec![
                "archive/pc/mod".to_string(),
                "bin/x64/plugins".to_string(),
            ],
            supports_load_order: true,
            resolver: Box::new(Cyberpunk2077Resolver),
        }
    }

    pub fn skyrim() -> Self {
        Self {
            id: "skyrim".to_string(),
            name: "Skyrim".to_string(),
            nexus_domain: "skyrim".to_string(),
            detection_files: vec!["TESV.exe".to_string()],
            mod_directories: vec!["Data".to_string()],
            supports_load_order: true,
            resolver: Box::new(SkyrimResolver),
        }
    }
}
```

```rust
// In main.rs - simplified to one line!

fn determine_install_path_for_file(
    game_dir: &Path,
    relative_path: &Path,
    game_id: &str,
) -> Result<PathBuf, String> {
    let game_def = game_definitions::get_game_by_id(game_id)
        .ok_or_else(|| format!("Unsupported game: {}", game_id))?;

    let normalized_path = normalize_game_path(relative_path);
    let path_str = normalized_path.to_string_lossy().to_lowercase();
    let file_name = normalized_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_lowercase();

    game_def.resolver.resolve_install_path(
        game_dir,
        relative_path,
        &normalized_path,
        &path_str,
        &file_name,
    )
}
```

---

## Next Steps

1. **Review this document** and decide which option aligns with your vision
2. **If choosing Option 1** (recommended), I can implement it in ~2 hours
3. **If preferring another option**, I can provide more detailed implementation plans

Would you like me to implement Option 1 (Trait-Based Plugin System)?
