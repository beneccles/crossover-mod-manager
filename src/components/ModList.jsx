import { useState } from "react";
import "./ModList.css";

function ModList({ mods, selectedMod, onSelectMod, loading, onRefresh }) {
  const [searchQuery, setSearchQuery] = useState("");
  const [filterStatus, setFilterStatus] = useState("all"); // all, enabled, disabled
  const [sortBy, setSortBy] = useState("name"); // name, date, version

  // Filter and sort mods
  const filteredAndSortedMods = mods
    .filter((mod) => {
      // Search filter
      const matchesSearch =
        mod.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        (mod.author && mod.author.toLowerCase().includes(searchQuery.toLowerCase())) ||
        (mod.description && mod.description.toLowerCase().includes(searchQuery.toLowerCase()));

      // Status filter
      const matchesStatus =
        filterStatus === "all" ||
        (filterStatus === "enabled" && mod.enabled) ||
        (filterStatus === "disabled" && !mod.enabled);

      return matchesSearch && matchesStatus;
    })
    .sort((a, b) => {
      switch (sortBy) {
        case "name":
          return a.name.localeCompare(b.name);
        case "date":
          // Sort by installed_at if available, otherwise by ID
          if (a.installed_at && b.installed_at) {
            return new Date(b.installed_at) - new Date(a.installed_at);
          }
          return 0;
        case "version":
          return a.version.localeCompare(b.version);
        default:
          return 0;
      }
    });

  return (
    <div className="mod-list">
      <div className="mod-list-header">
        <h2>Installed Mods</h2>
        <div className="mod-list-actions">
          <span className="mod-count">
            {filteredAndSortedMods.length} of {mods.length} mod{mods.length !== 1 ? "s" : ""}
          </span>
          <button
            onClick={onRefresh}
            className="refresh-button"
            disabled={loading}
            title="Refresh mod list"
          >
            Refresh
          </button>
        </div>
      </div>

      <div className="mod-list-filters">
        <input
          type="text"
          className="search-input"
          placeholder="Search mods..."
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
        />
        <div className="filter-controls">
          <select
            className="filter-select"
            value={filterStatus}
            onChange={(e) => setFilterStatus(e.target.value)}
          >
            <option value="all">All Mods</option>
            <option value="enabled">Enabled Only</option>
            <option value="disabled">Disabled Only</option>
          </select>
          <select
            className="sort-select"
            value={sortBy}
            onChange={(e) => setSortBy(e.target.value)}
          >
            <option value="name">Sort by Name</option>
            <option value="date">Sort by Date</option>
            <option value="version">Sort by Version</option>
          </select>
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
        ) : filteredAndSortedMods.length === 0 ? (
          <div className="empty-state">
            <p>No mods match your search or filter</p>
            <p className="help-text">Try different search terms or filters</p>
          </div>
        ) : (
          filteredAndSortedMods.map((mod) => (
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
