import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { open as openDialog } from "@tauri-apps/plugin-dialog";
import "./GameSelector.css";

function GameSelector({ onGameChange }) {
  const [supportedGames, setSupportedGames] = useState([]);
  const [configuredGames, setConfiguredGames] = useState([]);
  const [currentGame, setCurrentGame] = useState("");
  const [selectedNewGame, setSelectedNewGame] = useState("");
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    loadGames();
  }, []);

  const loadGames = async () => {
    try {
      // Get supported games
      const games = await invoke("get_supported_games");
      setSupportedGames(games);

      // Get settings to see configured games
      const settings = await invoke("get_settings");
      const configured = Object.entries(settings.games || {}).map(
        ([id, config]) => ({
          id,
          ...config,
          name: games.find((g) => g.id === id)?.name || id,
        })
      );
      setConfiguredGames(configured);
      setCurrentGame(settings.current_game || "");
    } catch (error) {
      console.error("Failed to load games:", error);
    }
  };

  const switchGame = async (gameId) => {
    try {
      setLoading(true);
      await invoke("switch_game", { gameId });
      setCurrentGame(gameId);
      if (onGameChange) onGameChange(gameId);
      alert(
        `✓ Switched to ${supportedGames.find((g) => g.id === gameId)?.name}`
      );
      await loadGames();
    } catch (error) {
      alert(`❌ Failed to switch game: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const autoDetectGame = async () => {
    if (!selectedNewGame) {
      alert("Please select a game first");
      return;
    }

    try {
      setLoading(true);
      const gameName = supportedGames.find(
        (g) => g.id === selectedNewGame
      )?.name;

      // Try to auto-detect the game in CrossOver bottles
      const detectedPath = await invoke("auto_detect_game_in_bottles", {
        gameId: selectedNewGame,
      });

      if (!detectedPath) {
        alert(
          `Could not auto-detect ${gameName} in any CrossOver bottles.\n\nPlease use the "Browse..." button to select the game folder manually.`
        );
        return;
      }

      // Add the game with the detected path
      await invoke("add_game", {
        gameId: selectedNewGame,
        gamePath: detectedPath,
      });

      alert(`✓ Auto-detected and added ${gameName}!\n\nPath: ${detectedPath}`);
      setSelectedNewGame("");
      await loadGames();
    } catch (error) {
      alert(`❌ Failed to auto-detect game: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const addNewGame = async () => {
    if (!selectedNewGame) {
      alert("Please select a game first");
      return;
    }

    try {
      const gamePath = await openDialog({
        title: `Select ${
          supportedGames.find((g) => g.id === selectedNewGame)?.name
        } Installation Folder`,
        directory: true,
        parent: getCurrentWindow(),
      });

      if (!gamePath) return;

      setLoading(true);

      // Detect game at path
      const detected = await invoke("detect_game_from_path", {
        path: gamePath,
      });

      if (!detected) {
        alert(
          `Could not detect ${
            supportedGames.find((g) => g.id === selectedNewGame)?.name
          } at the selected path. Please make sure you selected the correct game directory.`
        );
        return;
      }

      if (detected.id !== selectedNewGame) {
        alert(
          `Detected ${detected.name} but you selected ${
            supportedGames.find((g) => g.id === selectedNewGame)?.name
          }. Please select the correct game folder.`
        );
        return;
      }

      // Add the game
      await invoke("add_game", {
        gameId: selectedNewGame,
        gamePath: gamePath,
      });

      alert(`✓ Added ${detected.name}!`);
      setSelectedNewGame("");
      await loadGames();
    } catch (error) {
      alert(`❌ Failed to add game: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const removeGame = async (gameId) => {
    const gameName = configuredGames.find((g) => g.id === gameId)?.name;
    if (
      !confirm(
        `Remove ${gameName}?\n\nThis will not delete any mod files, only remove the game configuration.`
      )
    ) {
      return;
    }

    try {
      setLoading(true);
      await invoke("remove_game", { gameId });
      alert(`✓ Removed ${gameName}`);
      await loadGames();
    } catch (error) {
      alert(`❌ Failed to remove game: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const availableGames = supportedGames.filter(
    (game) => !configuredGames.find((cg) => cg.id === game.id)
  );

  return (
    <div className="game-selector">
      <h3>🎮 Game Management</h3>

      {configuredGames.length === 0 ? (
        <div className="no-games">
          <p>No games configured yet. Add a game to get started!</p>
        </div>
      ) : (
        <>
          <div className="current-game-section">
            <label>Current Game:</label>
            <div className="current-game">
              {currentGame ? (
                <>
                  <span className="game-name">
                    {configuredGames.find((g) => g.id === currentGame)?.name ||
                      currentGame}
                  </span>
                  <span className="active-badge">✓ Active</span>
                </>
              ) : (
                <span className="no-game">No game selected</span>
              )}
            </div>
          </div>

          <div className="configured-games-section">
            <label>Configured Games:</label>
            <div className="games-list">
              {configuredGames.map((game) => (
                <div key={game.id} className="game-item">
                  <div className="game-info">
                    <div className="game-header">
                      <span className="game-name">{game.name}</span>
                      {game.id === currentGame && (
                        <span className="current-indicator">Current</span>
                      )}
                    </div>
                    <span className="game-path">{game.game_path}</span>
                  </div>
                  <div className="game-actions">
                    {game.id !== currentGame && (
                      <button
                        onClick={() => switchGame(game.id)}
                        disabled={loading}
                        className="switch-button"
                      >
                        Switch
                      </button>
                    )}
                    <button
                      onClick={() => removeGame(game.id)}
                      disabled={loading}
                      className="remove-button"
                    >
                      Remove
                    </button>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </>
      )}

      {availableGames.length > 0 && (
        <div className="add-game-section">
          <label>Add New Game:</label>
          <div className="add-game-controls">
            <select
              value={selectedNewGame}
              onChange={(e) => setSelectedNewGame(e.target.value)}
              disabled={loading}
            >
              <option value="">Select a game...</option>
              {availableGames.map((game) => (
                <option key={game.id} value={game.id}>
                  {game.name}
                </option>
              ))}
            </select>
            <button
              onClick={autoDetectGame}
              disabled={!selectedNewGame || loading}
              className="auto-detect-button"
              title="Automatically scan CrossOver bottles for this game"
            >
              🔍 Auto-detect
            </button>
            <button
              onClick={addNewGame}
              disabled={!selectedNewGame || loading}
              className="add-button"
              title="Manually browse for game folder"
            >
              📁 Browse...
            </button>
          </div>
          <p className="help-text">
            Try auto-detect to scan CrossOver bottles, or browse to manually
            select the game folder.
          </p>
        </div>
      )}
    </div>
  );
}

export default GameSelector;
