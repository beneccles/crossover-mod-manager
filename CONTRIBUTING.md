# Contributing to Crossover Mod Manager

Thank you for your interest in contributing to Crossover Mod Manager! This document provides guidelines for contributing to the project.

## How to Contribute

### Reporting Bugs

If you find a bug, please create an issue with:

1. **Clear title**: Describe the bug concisely
2. **Description**: Detailed description of the issue
3. **Steps to reproduce**: How to trigger the bug
4. **Expected behavior**: What should happen
5. **Actual behavior**: What actually happens
6. **Environment**: OS, app version, game version
7. **Screenshots**: If applicable

Example:
```
Title: Mod installation fails with special characters in filename

Description: When installing a mod with special characters (e.g., "Mod™.zip"), 
the installation fails with an error.

Steps to reproduce:
1. Download a mod with special characters in filename
2. Click "Install"
3. Observe error message

Expected: Mod installs successfully
Actual: Error: "Failed to extract archive"

Environment: macOS 14.2, Crossover 24.0, CP2077 v2.1
```

### Suggesting Features

Feature requests are welcome! Please create an issue with:

1. **Clear title**: What feature you want
2. **Use case**: Why this feature is needed
3. **Proposed solution**: How it might work
4. **Alternatives**: Other approaches you've considered

### Pull Requests

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/your-feature-name`
3. **Make your changes**
4. **Test thoroughly**
5. **Commit with clear messages**: See commit guidelines below
6. **Push to your fork**
7. **Create a pull request**

#### Pull Request Guidelines

- **One feature per PR**: Keep PRs focused
- **Update documentation**: If you change functionality
- **Add tests**: If applicable
- **Follow code style**: See Development Guide
- **Update CHANGELOG**: Add your changes

#### Commit Message Format

Follow conventional commits:

```
type(scope): short description

Longer description if needed

Fixes #123
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Code style (formatting, no logic change)
- `refactor`: Code refactoring
- `test`: Adding tests
- `chore`: Maintenance tasks

Examples:
```
feat(mod-list): add search and filter functionality

fix(installation): handle special characters in filenames
Fixes #123

docs(readme): add troubleshooting section

refactor(mod-manager): simplify file path logic
```

## Development Guidelines

### Code Style

#### JavaScript/React

```javascript
// Use functional components
function MyComponent({ prop1, prop2 }) {
  const [state, setState] = useState(null)
  
  useEffect(() => {
    // Side effects
  }, [])
  
  return (
    <div className="my-component">
      {/* JSX */}
    </div>
  )
}

// Use descriptive variable names
const installedModsList = mods.filter(mod => mod.enabled)

// Use async/await for async operations
const handleInstall = async () => {
  try {
    const result = await invoke('install_mod', { modData })
  } catch (error) {
    console.error('Installation failed:', error)
  }
}
```

#### Rust

```rust
// Use descriptive names
fn install_mod(&mut self, mod_data: ModData) -> Result<(), String> {
    // Implementation
}

// Handle errors properly
let file = fs::read_to_string(path)
    .map_err(|e| format!("Failed to read file: {}", e))?;

// Add documentation comments for public items
/// Installs a mod from the provided data.
/// 
/// # Arguments
/// 
/// * `mod_data` - Information about the mod to install
/// 
/// # Returns
/// 
/// Returns `Ok(())` on success, or an error message on failure.
pub fn install_mod(&mut self, mod_data: ModData) -> Result<(), String> {
    // Implementation
}
```

### Testing Your Changes

Before submitting a PR:

1. **Build the frontend**: `npm run build`
2. **Run the app**: `npm run tauri:dev`
3. **Test your changes**: Manually verify functionality
4. **Check for console errors**: Look for errors in DevTools and terminal
5. **Test edge cases**: Try unusual inputs or scenarios

### Areas for Contribution

Here are some areas where contributions would be valuable:

#### Features
- Additional game support (Skyrim, Fallout, etc.)
- Mod conflict detection
- Automatic mod updates
- Load order management
- Mod profiles/collections
- Better error messages and recovery
- Dark/Light theme toggle
- Search and filter mods
- Batch mod operations

#### Improvements
- Better file type detection
- Progress bars for downloads
- Thumbnail/image support
- Mod descriptions from NexusMods API
- Keyboard shortcuts
- Better logging
- Performance optimizations
- UI/UX improvements

#### Documentation
- Video tutorials
- Screenshots
- Translation to other languages
- More examples
- FAQ section
- Troubleshooting guide

#### Testing
- Unit tests for Rust code
- Integration tests
- End-to-end tests
- Test fixtures and mocks

## Questions?

If you have questions about contributing:

1. Check the [README](README.md) and [Development Guide](DEVELOPMENT.md)
2. Search existing issues
3. Create a new issue with the "question" label

## Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inclusive environment for everyone, regardless of:
- Age, body size, disability, ethnicity
- Gender identity and expression
- Level of experience
- Nationality
- Personal appearance
- Race, religion
- Sexual identity and orientation

### Our Standards

Positive behavior includes:
- Using welcoming and inclusive language
- Being respectful of differing viewpoints
- Accepting constructive criticism gracefully
- Focusing on what's best for the community
- Showing empathy towards others

Unacceptable behavior includes:
- Harassment, trolling, or insulting comments
- Personal or political attacks
- Publishing others' private information
- Other conduct that could be considered inappropriate

### Enforcement

Project maintainers are responsible for clarifying standards and taking appropriate action in response to unacceptable behavior.

Maintainers have the right to remove, edit, or reject:
- Comments, commits, code, issues, and other contributions
- That don't align with this Code of Conduct

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Recognition

Contributors will be recognized in:
- CHANGELOG.md for their contributions
- GitHub contributor statistics
- Release notes when applicable

Thank you for contributing to Crossover Mod Manager! 🎮
