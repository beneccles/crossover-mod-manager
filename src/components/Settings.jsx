import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import {
  open as openDialog,
  save as saveDialog,
  ask as askDialog,
} from "@tauri-apps/plugin-dialog";
import GameSelector from "./GameSelector";
import "./Settings.css";

function Settings() {
  const [gamePath, setGamePath] = useState("");
  const [modStoragePath, setModStoragePath] = useState("");
  const [nexusmodsApiKey, setNexusmodsApiKey] = useState("");
  const [loading, setLoading] = useState(false);
  const [saved, setSaved] = useState(false);
  const [autoDetecting, setAutoDetecting] = useState(false);
  const [detectionResult, setDetectionResult] = useState("");
  const [customNxmUrl, setCustomNxmUrl] = useState(
    "nxm://cyberpunk2077/mods/107/files/123169?key=test&expires=1760073990&user_id=260682775"
  );

  useEffect(() => {
    loadSettings();
  }, []);

  const loadSettings = async () => {
    try {
      const settings = await invoke("get_settings");
      setGamePath(settings.game_path || "");
      setModStoragePath(settings.mod_storage_path || "");
      setNexusmodsApiKey(settings.nexusmods_api_key || "");
    } catch (error) {
      console.error("Failed to load settings:", error);
    }
  };

  const selectGamePath = async () => {
    try {
      const selected = await openDialog({
        directory: true,
        multiple: false,
        title: "Select Game Installation Directory",
        parent: getCurrentWindow(),
      });

      if (selected) {
        setGamePath(selected);
      }
    } catch (error) {
      console.error("Failed to select directory:", error);
    }
  };

  const selectModStoragePath = async () => {
    try {
      const selected = await openDialog({
        directory: true,
        multiple: false,
        title: "Select Mod Storage Directory",
        parent: getCurrentWindow(),
      });

      if (selected) {
        setModStoragePath(selected);
      }
    } catch (error) {
      console.error("Failed to select directory:", error);
    }
  };

  const autoDetectGamePath = async () => {
    setAutoDetecting(true);
    setDetectionResult("");
    try {
      const detectedPath = await invoke("auto_detect_game_path");
      if (detectedPath) {
        setGamePath(detectedPath);
        setDetectionResult("✓ Game installation detected automatically!");
        setTimeout(() => setDetectionResult(""), 5000);
      } else {
        setDetectionResult(
          "⚠ Could not automatically detect game installation. Please select manually."
        );
        setTimeout(() => setDetectionResult(""), 5000);
      }
    } catch (error) {
      console.error("Failed to auto-detect game path:", error);
      setDetectionResult("❌ Auto-detection failed. Please select manually.");
      setTimeout(() => setDetectionResult(""), 5000);
    } finally {
      setAutoDetecting(false);
    }
  };

  const saveSettings = async () => {
    setLoading(true);
    setSaved(false);
    try {
      await invoke("save_settings", {
        settings: {
          game_path: gamePath,
          mod_storage_path: modStoragePath,
          nexusmods_api_key: nexusmodsApiKey,
        },
      });
      setSaved(true);
      setTimeout(() => setSaved(false), 3000);
    } catch (error) {
      console.error("Failed to save settings:", error);
      alert("Failed to save settings: " + error);
    } finally {
      setLoading(false);
    }
  };

  const testNxmUrl = async () => {
    try {
      // Test with a sample Cyberpunk 2077 NXM URL
      const testUrl =
        "nxm://cyberpunk2077/mods/107/files/123169?key=SIMRrmIOUwWBwlUHBwf-Gzw&expires=1760073185&user_id=260682775";
      await invoke("handle_nxm_url", { nxm_url: testUrl });
      alert("NXM URL test completed! Check the Logs tab.");
    } catch (error) {
      console.error("Failed to test NXM URL:", error);
      alert("NXM URL test failed: " + error);
    }
  };

  const testCustomNxmUrl = async () => {
    try {
      await invoke("handle_nxm_url", { nxm_url: customNxmUrl });
      alert("Custom NXM URL processed! Check the Logs tab.");
    } catch (error) {
      console.error("Failed to process custom NXM URL:", error);
      alert("Custom NXM URL processing failed: " + error);
    }
  };

  const testBasicLogging = async () => {
    try {
      await invoke("test_logging");
      alert("Test log entry added! Check the Logs tab to see it.");
    } catch (error) {
      console.error("Failed to test logging:", error);
      alert("Logging test failed: " + error);
    }
  };

  const testModDownload = async () => {
    try {
      // Test downloading a small sample file
      const testUrl = "https://httpbin.org/base64/aGVsbG8gd29ybGQ="; // Returns "hello world" as base64
      const filePath = await invoke("download_and_save_mod", {
        mod_name: "Test_Mod",
        download_url: testUrl,
      });
      alert(
        `Test download completed! File saved to: ${filePath}\nCheck the Logs tab.`
      );
    } catch (error) {
      console.error("Failed to test download:", error);
      alert("Download test failed: " + error);
    }
  };

  const testNxmEvent = async () => {
    try {
      const testUrl = "nxm://cyberpunk2077/mods/999/files/888";
      await invoke("test_nxm_event", { test_url: testUrl });
      alert("NXM event test sent! Check the Logs tab for processing.");
    } catch (error) {
      console.error("Failed to test NXM event:", error);
      alert("NXM event test failed: " + error);
    }
  };

  const [cleaningTemp, setCleaningTemp] = useState(false);
  const [cleanupResult, setCleanupResult] = useState("");

  const cleanTempFiles = async () => {
    setCleaningTemp(true);
    setCleanupResult("");
    try {
      const result = await invoke("clean_temp_files");
      setCleanupResult(`✓ ${result}`);
      setTimeout(() => setCleanupResult(""), 5000);
    } catch (error) {
      console.error("Failed to clean temp files:", error);
      setCleanupResult(`❌ Cleanup failed: ${error}`);
      setTimeout(() => setCleanupResult(""), 5000);
    } finally {
      setCleaningTemp(false);
    }
  };

  const exportModProfile = async () => {
    try {
      console.log("Opening save dialog...");

      // Use save dialog to select export location
      const filePath = await saveDialog({
        title: "Export Mod Profile",
        defaultPath: `cmm-profile-${
          new Date().toISOString().split("T")[0]
        }.json`,
        filters: [
          {
            name: "JSON",
            extensions: ["json"],
          },
        ],
        parent: getCurrentWindow(),
      });

      console.log("Save dialog returned:", filePath);

      if (!filePath) {
        console.log("User cancelled or no path selected");
        return; // User cancelled
      }

      console.log("Calling export_mod_profile with path:", filePath);

      const result = await invoke("export_mod_profile", {
        filePath: filePath,
      });

      console.log("Export result:", result);
      alert(`✓ ${result}\n\nFile saved to: ${filePath}`);
    } catch (error) {
      console.error("Failed to export mod profile:", error);
      alert(`❌ Export failed: ${error}`);
    }
  };

  const importModProfile = async () => {
    try {
      // Use open dialog to select import file
      const selected = await openDialog({
        title: "Import Mod Profile",
        multiple: false,
        filters: [
          {
            name: "JSON",
            extensions: ["json"],
          },
        ],
        parent: getCurrentWindow(),
      });

      if (!selected) {
        return; // User cancelled
      }

      // Confirm with user using Tauri's dialog
      const confirmed = await askDialog(
        "This will:\n" +
          "1. Register any mods whose files already exist\n" +
          "2. Automatically download and install missing mods\n\n" +
          "Continue?",
        {
          title: "Import Mod Profile",
          kind: "info",
          parent: getCurrentWindow(),
        }
      );

      if (!confirmed) {
        return;
      }

      const result = await invoke("import_mod_profile", {
        filePath: selected,
      });

      alert(`✓ ${result}\n\nCheck the Logs tab for details.`);
    } catch (error) {
      console.error("Failed to import mod profile:", error);
      alert(`❌ Import failed: ${error}`);
    }
  };

  return (
    <div className="settings">
      <div className="settings-content">
        <h2>Settings</h2>

        <div className="setting-section">
          <GameSelector onGameChange={() => loadSettings()} />
        </div>

        <div className="setting-section">
          <h3>Game Configuration (Legacy)</h3>

          <div className="setting-row">
            <label>Game Installation Path:</label>
            <div className="path-selector">
              <input
                type="text"
                value={gamePath}
                onChange={(e) => setGamePath(e.target.value)}
                placeholder="Select your game installation directory..."
              />
              <div className="path-buttons">
                <button
                  onClick={autoDetectGamePath}
                  disabled={autoDetecting}
                  className="auto-detect-button"
                >
                  {autoDetecting ? "Detecting..." : "Auto-Detect"}
                </button>
                <button onClick={selectGamePath}>Browse</button>
              </div>
            </div>
            {detectionResult && (
              <p
                className={`detection-result ${
                  detectionResult.includes("✓")
                    ? "success"
                    : detectionResult.includes("⚠")
                    ? "warning"
                    : "error"
                }`}
              >
                {detectionResult}
              </p>
            )}
            <p className="help-text">
              This should be the path to your Cyberpunk 2077 installation in
              Crossover (e.g., /Users/username/Library/Application
              Support/CrossOver/Bottles/...)
            </p>
          </div>

          <div className="setting-row">
            <label>Mod Storage Directory:</label>
            <div className="path-selector">
              <input
                type="text"
                value={modStoragePath}
                onChange={(e) => setModStoragePath(e.target.value)}
                placeholder="Select where downloaded mods will be stored..."
              />
              <div className="path-buttons">
                <button onClick={selectModStoragePath}>Browse</button>
              </div>
            </div>
            <p className="help-text">
              Downloaded mods will be saved to this directory. Default:
              ~/Downloads/CrossoverModManager/Mods
            </p>
          </div>

          <div className="setting-row">
            <label>NexusMods API Key:</label>
            <div className="path-selector">
              <input
                type="password"
                value={nexusmodsApiKey}
                onChange={(e) => setNexusmodsApiKey(e.target.value)}
                placeholder="Enter your NexusMods API key..."
              />
            </div>
            <p className="help-text">
              Get your API key from{" "}
              <a
                href="https://www.nexusmods.com/users/myaccount?tab=api"
                target="_blank"
                rel="noopener noreferrer"
              >
                NexusMods API settings
              </a>
              . Required for downloading mods via NXM links.
            </p>
          </div>
        </div>

        <div className="setting-section">
          <h3>� Mod Profile Export & Import</h3>
          <p>Smart backup and restore for your mod setup</p>
          <div className="setting-row">
            <button onClick={exportModProfile} className="test-nxm-button">
              📤 Export Mod Profile
            </button>
            <p className="help-text">
              Export a complete profile of your installed mods including file
              paths and NexusMods IDs. Use this to back up your mod setup or
              transfer it to another system.
            </p>
          </div>
          <div className="setting-row">
            <button onClick={importModProfile} className="test-nxm-button">
              📥 Import Mod Profile
            </button>
            <p className="help-text">
              Import a previously exported mod profile. CMM will automatically
              register mods whose files already exist on disk. Missing mods will
              be flagged so you can re-download them from NexusMods.
            </p>
          </div>
        </div>

        <div className="setting-section">
          <h3>System Maintenance</h3>
          <p>Cleanup and maintenance tools</p>
          <div className="setting-row">
            <button
              onClick={cleanTempFiles}
              className="test-nxm-button"
              disabled={cleaningTemp}
            >
              {cleaningTemp ? "🧹 Cleaning..." : "🧹 Clean Temporary Files"}
            </button>
            {cleanupResult && (
              <p
                className={`cleanup-result ${
                  cleanupResult.includes("✓") ? "success" : "error"
                }`}
              >
                {cleanupResult}
              </p>
            )}
            <p className="help-text">
              Removes orphaned mod archives and extraction directories from
              failed installations. The app automatically cleans up on startup,
              but you can manually trigger cleanup here.
            </p>
          </div>
        </div>

        <div className="setting-section">
          <h3>System Testing</h3>
          <p>Test basic functionality (for debugging)</p>
          <div className="setting-row">
            <button onClick={testBasicLogging} className="test-nxm-button">
              📝 Test Basic Logging
            </button>
            <p className="help-text">
              This will add a test log entry. Check the Logs tab to verify
              logging is working.
            </p>
          </div>
          <div className="setting-row">
            <button onClick={testModDownload} className="test-nxm-button">
              💾 Test Mod Download
            </button>
            <p className="help-text">
              This will test downloading and saving a small test file. Check the
              Logs tab and your mod storage directory.
            </p>
          </div>
          <div className="setting-row">
            <button onClick={testNxmEvent} className="test-nxm-button">
              📡 Test NXM Event System
            </button>
            <p className="help-text">
              This tests the event system that handles NXM URLs from external
              sources.
            </p>
          </div>
        </div>

        <div className="setting-section">
          <h3>NXM Protocol Testing</h3>
          <p>Test NXM URL handling (for development and troubleshooting)</p>
          <div className="setting-row">
            <button onClick={() => testNxmUrl()} className="test-nxm-button">
              🔗 Test Sample NXM URL
            </button>
            <p className="help-text">
              This will test processing of a sample NXM URL from NexusMods.
              Check the Logs tab to see the processing steps.
            </p>
          </div>
          <div className="setting-row">
            <label>Custom NXM URL:</label>
            <div className="path-selector">
              <input
                type="text"
                value={customNxmUrl}
                onChange={(e) => setCustomNxmUrl(e.target.value)}
                placeholder="Paste an NXM URL from NexusMods here..."
              />
              <button onClick={testCustomNxmUrl} className="test-nxm-button">
                🧪 Test Custom URL
              </button>
            </div>
            <p className="help-text">
              Copy an NXM URL from NexusMods (right-click "Download with Mod
              Manager" → Copy Link) and paste it here to test processing without
              going through the protocol registration.
            </p>
          </div>
          <div className="setting-row">
            <p className="help-text">
              <strong>Manual Protocol Association:</strong>
              <br />
              If NXM links don't open automatically, you can manually associate
              them:
              <br />
              1. Right-click on any NXM link on NexusMods
              <br />
              2. Select "Choose Application..."
              <br />
              3. Navigate to your Applications folder and select "Crossover Mod
              Manager"
            </p>
          </div>
        </div>

        <div className="setting-section">
          <h3>About</h3>
          <div className="setting-row">
            <p className="version-info">
              <strong>Crossover Mod Manager</strong> v1.1.0
            </p>
            <p className="help-text">
              Features: Auto-detection, NXM protocol support, comprehensive
              logging, window management, mod download & storage
            </p>
          </div>
        </div>

        <div className="settings-actions">
          <button
            className="save-button"
            onClick={saveSettings}
            disabled={loading || !gamePath || !modStoragePath}
          >
            {loading ? "Saving..." : "Save Settings"}
          </button>
          {saved && (
            <span className="save-success">✓ Settings saved successfully</span>
          )}
        </div>
      </div>
    </div>
  );
}

export default Settings;
