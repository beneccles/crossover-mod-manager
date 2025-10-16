# Tired of manually installing mods in CrossOver? I built a mod manager for macOS

---

Hey r/macgaming! 👋

I've been playing Cyberpunk 2077 through CrossOver on my Mac and got frustrated with manually installing mods from NexusMods. So I built a native macOS mod manager to make modding easier!

### What is Crossover Mod Manager?

It's a native Mac app (built with Tauri + Rust) that integrates directly with NexusMods. Click "Download with Mod Manager" on NexusMods, and it automatically downloads, extracts, and installs mods to your CrossOver game installation.

### Key Features:

- **One-click installation** from NexusMods (responds to nxm:// links)
- **Automatic detection** of your game path in CrossOver
- **Multi-format support** (ZIP, 7z, RAR archives)
- **Safe mod removal** - tracks all files and removes cleanly
- **CrossOver-optimized** - handles case sensitivity and Wine paths correctly
- **Native performance** - no Electron bloat, built in Rust

### Current Status:

🧪 **Now in beta testing!**

- **Currently supports:** Cyberpunk 2077
- **Platform:** macOS 11+ with Apple Silicon (M1/M2/M3/M4)
- **Requirements:** CrossOver 25+ (v24 might work)
- **License:** MIT (fully open source)

### Why I built this:

I love modding games, but the existing mod managers (Vortex, MO2) don't work well on Mac, even through Wine/CrossOver. I wanted something that felt native and just worked with how macOS and CrossOver handle files.

### Looking for Beta Testers!

I'm looking for folks to test this out and provide feedback. Since it's in beta, there may be bugs, but it's been stable for me so far.

**What I need help testing:**

- Installation process and setup
- Mod downloads and installations
- Different CrossOver bottle configurations
- Edge cases and error scenarios

### Download & Installation:

**[📦 Download v0.1.0-beta1](https://github.com/beneccles/crossover-mod-manager/releases)**

**Quick setup:**

1. Download the DMG
2. Right-click → Open (first time only, due to Apple's security)
3. Point it to your Cyberpunk 2077 installation in CrossOver
4. Start modding!

**Full instructions:** See the [README](https://github.com/beneccles/crossover-mod-manager)

### Security Note:

The beta is ad-hoc signed, so macOS will show a security warning on first launch. This is normal for beta software. Right-click → Open to bypass it. A fully signed version will come with v1.0.

### Future Plans:

- Support for more games (Skyrim, Fallout, etc.)
- Auto-update functionality
- Mod conflict detection
- Load order management

### Contributing:

The project is fully open source on GitHub. If you find bugs or have feature requests, please [open an issue](https://github.com/beneccles/crossover-mod-manager/issues)!

---

**GitHub:** https://github.com/beneccles/crossover-mod-manager

**Download:** https://github.com/beneccles/crossover-mod-manager/releases

---

Happy to answer any questions! Let me know if you run into any issues or have suggestions for features.

**Note:** This is a passion project I built for the Mac gaming community. No monetization, ads, or data collection - just trying to make modding easier for fellow Mac gamers! 🎮

---

## Alternative Shorter Version (if character limit):

---

### Title:

**[Beta] Built a native mod manager for CrossOver games on Apple Silicon**

### Body:

I got tired of manually installing mods for Cyberpunk 2077 on CrossOver, so I built a native Mac mod manager!

**Features:**

- One-click install from NexusMods (nxm:// protocol)
- Automatic game detection
- Supports ZIP/7z/RAR archives
- Safe mod removal with file tracking
- CrossOver path handling

**Platform:** macOS 11+ Apple Silicon only  
**Currently supports:** Cyberpunk 2077  
**Status:** Beta testing  
**License:** Open source (MIT)

**Looking for testers!** Especially folks with different CrossOver setups.

**[Download v0.1.0-beta1](https://github.com/beneccles/crossover-mod-manager/releases)**

Built with Tauri + Rust, so it's fast and native. No Electron bloat!

Future plans include Skyrim, Fallout, and other moddable games.

**GitHub:** https://github.com/beneccles/crossover-mod-manager

---

## Tips for Posting:

1. **Best time to post:** Morning EST (8-10 AM) or evening EST (6-8 PM) on weekdays
2. **Flair:** Use "Tool/Software" or "Discussion" flair if available
3. **Engage quickly:** Reply to comments within the first hour for visibility
4. **Be humble:** Frame it as "looking for feedback" not "here's my amazing tool"
5. **Screenshots:** Consider adding a screenshot or two (maybe of the app interface)
6. **Video:** A short demo GIF would get more engagement

## Common Questions to Prepare For:

**Q: Why not just use Vortex?**  
A: Vortex doesn't work well through Wine/CrossOver on Mac. This is native, handles CrossOver paths correctly, and doesn't require black magic to get NXM links to work on CrossOver games.

**Q: Intel Mac support?**  
A: Not currently, but could add if there's demand. Built I built this for my M4 Pro MBP initially.

**Q: Why not support more games?**  
A: Working on it! Started with CP2077 since that's what I play. Architecture supports other games, just need to add configs.

**Q: Is this safe?**  
A: Yes - it's open source so you can audit the code. No network access except NexusMods API. All mod operations are local.

**Q: How does it compare to MO2?**  
A: Simpler and Mac-native, but doesn't have all of MO2's advanced features yet. Great for basic modding needs.
