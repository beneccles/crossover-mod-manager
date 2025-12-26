import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./LoadOrderManager.css";

function LoadOrderManager() {
  const [archiveFiles, setArchiveFiles] = useState([]);
  const [loading, setLoading] = useState(false);
  const [draggedIndex, setDraggedIndex] = useState(null);

  useEffect(() => {
    loadArchiveFiles();
  }, []);

  const loadArchiveFiles = async () => {
    setLoading(true);
    try {
      const files = await invoke("get_archive_load_order");
      setArchiveFiles(files);
    } catch (error) {
      console.error("Failed to load archive files:", error);
    } finally {
      setLoading(false);
    }
  };

  const handleDragStart = (index) => {
    setDraggedIndex(index);
  };

  const handleDragOver = (e, index) => {
    e.preventDefault();
    if (draggedIndex === null || draggedIndex === index) return;

    const newFiles = [...archiveFiles];
    const draggedFile = newFiles[draggedIndex];
    newFiles.splice(draggedIndex, 1);
    newFiles.splice(index, 0, draggedFile);

    setDraggedIndex(index);
    setArchiveFiles(newFiles);
  };

  const handleDragEnd = () => {
    setDraggedIndex(null);
  };

  const handleApplyLoadOrder = async () => {
    setLoading(true);
    try {
      await invoke("apply_archive_load_order", {
        newOrder: archiveFiles.map((f) => f.path),
      });
      alert("Load order applied successfully!");
      await loadArchiveFiles();
    } catch (error) {
      console.error("Failed to apply load order:", error);
      alert("Failed to apply load order: " + error);
    } finally {
      setLoading(false);
    }
  };

  const handleReset = () => {
    loadArchiveFiles();
  };

  const addPrefix = async (index, prefix) => {
    setLoading(true);
    try {
      await invoke("rename_archive_with_prefix", {
        filePath: archiveFiles[index].path,
        prefix: prefix,
      });
      await loadArchiveFiles();
    } catch (error) {
      console.error("Failed to rename file:", error);
      alert("Failed to rename file: " + error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="load-order-manager">
      <div className="load-order-header">
        <h2>Archive Load Order</h2>
        <div className="load-order-actions">
          <button
            onClick={handleReset}
            className="reset-button"
            disabled={loading}
          >
            Reset
          </button>
          <button
            onClick={handleApplyLoadOrder}
            className="apply-button"
            disabled={loading}
          >
            Apply Order
          </button>
        </div>
      </div>

      <div className="load-order-info">
        <p>
          📦 Cyberpunk 2077 loads .archive files alphabetically. Files loaded
          <strong> later override earlier ones</strong>.
        </p>
        <p>
          Drag and drop to reorder, or use quick prefix buttons (0- loads
          first, z- loads last).
        </p>
      </div>

      <div className="archive-list">
        {loading && archiveFiles.length === 0 ? (
          <div className="loading-state">Loading archive files...</div>
        ) : archiveFiles.length === 0 ? (
          <div className="empty-state">
            <p>No .archive files found</p>
            <p className="help-text">
              Install mods with .archive files to manage load order
            </p>
          </div>
        ) : (
          archiveFiles.map((file, index) => (
            <div
              key={file.path}
              className={`archive-item ${
                draggedIndex === index ? "dragging" : ""
              }`}
              draggable
              onDragStart={() => handleDragStart(index)}
              onDragOver={(e) => handleDragOver(e, index)}
              onDragEnd={handleDragEnd}
            >
              <div className="archive-info">
                <span className="load-order-number">{index + 1}</span>
                <div className="archive-details">
                  <h4>{file.name}</h4>
                  <p className="archive-mod">From: {file.mod_name}</p>
                </div>
              </div>
              <div className="archive-actions">
                <button
                  onClick={() => addPrefix(index, "0-")}
                  className="prefix-button"
                  title="Add 0- prefix (load first)"
                  disabled={loading || file.name.startsWith("0-")}
                >
                  0-
                </button>
                <button
                  onClick={() => addPrefix(index, "z-")}
                  className="prefix-button"
                  title="Add z- prefix (load last)"
                  disabled={loading || file.name.startsWith("z-")}
                >
                  z-
                </button>
                <span className="drag-handle">⋮⋮</span>
              </div>
            </div>
          ))
        )}
      </div>

      {archiveFiles.length > 1 && (
        <div className="load-order-summary">
          <h3>Load Order Summary</h3>
          <p>
            🔽 <strong>First to load:</strong> {archiveFiles[0]?.name}
          </p>
          <p>
            🔼 <strong>Last to load (wins conflicts):</strong>{" "}
            {archiveFiles[archiveFiles.length - 1]?.name}
          </p>
        </div>
      )}
    </div>
  );
}

export default LoadOrderManager;
