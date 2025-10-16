import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { ask as askDialog } from "@tauri-apps/plugin-dialog";
import ModList from "./components/ModList";
import ModDetails from "./components/ModDetails";
import Settings from "./components/Settings";
import Logs from "./components/Logs";
import "./App.css";

function App() {
  const [mods, setMods] = useState([]);
  const [selectedMod, setSelectedMod] = useState(null);
  const [activeTab, setActiveTab] = useState("mods");
  const [loading, setLoading] = useState(false);
  const [currentGame, setCurrentGame] = useState(null);

  useEffect(() => {
    loadMods();
    loadCurrentGame();

    // Check and run first setup
    const runFirstSetup = async () => {
      try {
        const result = await invoke("check_and_run_first_setup");
        console.log("First setup check:", result);
      } catch (error) {
        console.error("Failed to run first setup:", error);
      }
    };

    runFirstSetup();

    // Listen for NXM URL events from the protocol handler
    const setupNxmListener = async () => {
      try {
        const unlisten = await listen("nxm-url-received", async (event) => {
          console.log("🔵 Received NXM URL event:", event.payload);

          // Log to Tauri backend as well
          try {
            await invoke("add_log_entry", {
              message: `🔵 Frontend: Received NXM URL event, about to call handle_nxm_url`,
              level: "info",
              category: "nxm_protocol",
            });
          } catch (e) {
            console.error("Failed to log to backend:", e);
          }

          // Automatically switch to logs tab to show progress
          setActiveTab("logs");

          try {
            console.log("🟡 About to invoke handle_nxm_url...");
            // Process the NXM URL
            await invoke("handle_nxm_url", { nxmUrl: event.payload });
            console.log("🟢 Successfully processed NXM URL from system");
          } catch (error) {
            console.error("🔴 Failed to process NXM URL from system:", error);
            alert("Failed to process NXM URL: " + error);

            // Try to log the error to backend
            try {
              await invoke("add_log_entry", {
                message: `🔴 Frontend error: ${error}`,
                level: "error",
                category: "nxm_protocol",
              });
            } catch (e) {
              console.error("Failed to log error to backend:", e);
            }
          }
        });

        // Cleanup function
        return unlisten;
      } catch (error) {
        console.error("Failed to setup NXM listener:", error);
      }
    };

    setupNxmListener();

    // Listen for mod-installed events to refresh the mod list
    const setupModInstalledListener = async () => {
      try {
        const unlisten = await listen("mod-installed", async (event) => {
          console.log("🎉 Mod installed event received:", event.payload);

          // Log to backend
          try {
            await invoke("add_log_entry", {
              message: `🎉 Frontend: Received mod-installed event for "${event.payload.name}", refreshing mod list`,
              level: "info",
              category: "installation",
            });
          } catch (e) {
            console.error("Failed to log to backend:", e);
          }

          // Add a small delay to ensure backend database is updated
          await new Promise((resolve) => setTimeout(resolve, 100));

          // Refresh the mod list
          await loadMods();

          // Switch to mods tab to show the newly installed mod
          setActiveTab("mods");
        });

        return unlisten;
      } catch (error) {
        console.error("Failed to setup mod-installed listener:", error);
      }
    };

    setupModInstalledListener();

    // Listen for collection-complete events
    const setupCollectionCompleteListener = async () => {
      try {
        const unlisten = await listen("collection-complete", async (event) => {
          console.log("🎉 Collection complete event received:", event.payload);

          // Log to backend
          try {
            await invoke("add_log_entry", {
              message: `🎉 Frontend: Collection installation complete, refreshing mod list`,
              level: "info",
              category: "installation",
            });
          } catch (e) {
            console.error("Failed to log to backend:", e);
          }

          // Refresh the mod list one final time
          await loadMods();

          // Switch to mods tab to show all newly installed mods
          setActiveTab("mods");
        });

        return unlisten;
      } catch (error) {
        console.error("Failed to setup collection-complete listener:", error);
      }
    };

    setupCollectionCompleteListener();

    // Listen for game-switched events to update current game
    const setupGameSwitchedListener = async () => {
      try {
        const unlisten = await listen("game-switched", async (event) => {
          console.log("🎮 Game switched event received:", event.payload);

          // Reload current game info
          await loadCurrentGame();
        });

        return unlisten;
      } catch (error) {
        console.error("Failed to setup game-switched listener:", error);
      }
    };

    setupGameSwitchedListener();

    // Listen for mods-updated events to refresh both mods and current game
    const setupModsUpdatedListener = async () => {
      try {
        const unlisten = await listen("mods-updated", async () => {
          console.log("🔄 Mods updated event received, refreshing...");
          await loadMods();
          await loadCurrentGame();
        });

        return unlisten;
      } catch (error) {
        console.error("Failed to setup mods-updated listener:", error);
      }
    };

    setupModsUpdatedListener();
  }, []);

  const loadCurrentGame = async () => {
    try {
      const settings = await invoke("get_settings");
      const games = await invoke("get_supported_games");

      if (settings.current_game) {
        const gameInfo = games.find((g) => g.id === settings.current_game);
        setCurrentGame(gameInfo);
      }
    } catch (error) {
      console.error("Failed to load current game:", error);
    }
  };

  const loadMods = async () => {
    try {
      console.log("Loading mods...");
      const modList = await invoke("get_installed_mods");
      console.log("Loaded mods:", modList.length, "mods");
      setMods(modList);
    } catch (error) {
      console.error("Failed to load mods:", error);

      // Log error to backend
      try {
        await invoke("add_log_entry", {
          message: `❌ Frontend: Failed to load mods: ${error}`,
          level: "error",
          category: "system",
        });
      } catch (e) {
        console.error("Failed to log error to backend:", e);
      }
    }
  };

  const handleInstallMod = async (modData) => {
    setLoading(true);
    try {
      await invoke("install_mod", { modData });
      await loadMods();
    } catch (error) {
      console.error("Failed to install mod:", error);
      alert("Failed to install mod: " + error);
    } finally {
      setLoading(false);
    }
  };

  const handleRemoveMod = async (modId) => {
    const confirmed = await askDialog(
      "Are you sure you want to remove this mod? All installed files will be deleted.",
      {
        title: "Remove Mod",
        kind: "warning",
        parent: getCurrentWindow(),
      }
    );

    if (!confirmed) {
      return;
    }

    setLoading(true);
    // Switch to logs tab to show removal progress
    setActiveTab("logs");

    try {
      const result = await invoke("remove_mod", { modId });
      console.log("Mod removed:", result);
      await loadMods();
      setSelectedMod(null);
      alert(result); // Show success message
    } catch (error) {
      console.error("Failed to remove mod:", error);
      alert("Failed to remove mod: " + error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="app">
      <header className="app-header">
        <h1>Crossover Mod Manager</h1>
        <nav className="tabs">
          <button
            className={activeTab === "mods" ? "active" : ""}
            onClick={() => setActiveTab("mods")}
          >
            Mods
          </button>
          <button
            className={activeTab === "logs" ? "active" : ""}
            onClick={() => setActiveTab("logs")}
          >
            Logs
          </button>
          <button
            className={activeTab === "settings" ? "active" : ""}
            onClick={() => setActiveTab("settings")}
          >
            Settings
          </button>
        </nav>
      </header>

      <main className="app-content">
        {activeTab === "mods" ? (
          <div className="mod-manager">
            <ModList
              mods={mods}
              selectedMod={selectedMod}
              onSelectMod={setSelectedMod}
              loading={loading}
              onRefresh={loadMods}
              currentGame={currentGame}
            />
            <ModDetails
              mod={selectedMod}
              onRemove={handleRemoveMod}
              loading={loading}
            />
          </div>
        ) : activeTab === "logs" ? (
          <Logs />
        ) : (
          <Settings />
        )}
      </main>

      {loading && (
        <div className="loading-overlay">
          <div className="spinner"></div>
          <p>Processing...</p>
        </div>
      )}
    </div>
  );
}

export default App;
