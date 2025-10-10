import "./ModList.css";

function ModList({ mods, selectedMod, onSelectMod, loading, onRefresh }) {
  return (
    <div className="mod-list">
      <div className="mod-list-header">
        <h2>Installed Mods</h2>
        <div className="mod-list-actions">
          <span className="mod-count">
            {mods.length} mod{mods.length !== 1 ? "s" : ""}
          </span>
          <button
            onClick={onRefresh}
            className="refresh-button"
            disabled={loading}
            title="Refresh mod list"
          >
            🔄 Refresh
          </button>
        </div>
      </div>

      <div className="mod-list-content">
        {mods.length === 0 ? (
          <div className="empty-state">
            <p>No mods installed yet</p>
            <p className="help-text">
              Click "Download with Mod Manager" on NexusMods to install mods
            </p>
          </div>
        ) : (
          mods.map((mod) => (
            <div
              key={mod.id}
              className={`mod-item ${
                selectedMod?.id === mod.id ? "selected" : ""
              }`}
              onClick={() => onSelectMod(mod)}
            >
              <div className="mod-info">
                <h3>{mod.name}</h3>
                <p className="mod-version">v{mod.version}</p>
              </div>
              <div
                className={`mod-status ${mod.enabled ? "enabled" : "disabled"}`}
              >
                {mod.enabled ? "✓" : "○"}
              </div>
            </div>
          ))
        )}
      </div>
    </div>
  );
}

export default ModList;
