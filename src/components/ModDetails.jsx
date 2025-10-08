import './ModDetails.css'

function ModDetails({ mod, onRemove, loading }) {
  if (!mod) {
    return (
      <div className="mod-details">
        <div className="empty-state">
          <p>Select a mod to view details</p>
        </div>
      </div>
    )
  }

  return (
    <div className="mod-details">
      <div className="mod-details-header">
        <h2>{mod.name}</h2>
        <button 
          className="remove-button"
          onClick={() => onRemove(mod.id)}
          disabled={loading}
        >
          Remove Mod
        </button>
      </div>

      <div className="mod-details-content">
        <div className="detail-section">
          <h3>Information</h3>
          <div className="detail-row">
            <span className="label">Version:</span>
            <span className="value">{mod.version}</span>
          </div>
          <div className="detail-row">
            <span className="label">Author:</span>
            <span className="value">{mod.author || 'Unknown'}</span>
          </div>
          <div className="detail-row">
            <span className="label">Mod ID:</span>
            <span className="value">{mod.mod_id || 'N/A'}</span>
          </div>
          <div className="detail-row">
            <span className="label">File ID:</span>
            <span className="value">{mod.file_id || 'N/A'}</span>
          </div>
          <div className="detail-row">
            <span className="label">Status:</span>
            <span className={`value status ${mod.enabled ? 'enabled' : 'disabled'}`}>
              {mod.enabled ? 'Enabled' : 'Disabled'}
            </span>
          </div>
        </div>

        <div className="detail-section">
          <h3>Installed Files</h3>
          <div className="file-list">
            {mod.files && mod.files.length > 0 ? (
              mod.files.map((file, index) => (
                <div key={index} className="file-item">
                  {file}
                </div>
              ))
            ) : (
              <p className="no-files">No file information available</p>
            )}
          </div>
        </div>

        {mod.description && (
          <div className="detail-section">
            <h3>Description</h3>
            <p className="description">{mod.description}</p>
          </div>
        )}
      </div>
    </div>
  )
}

export default ModDetails
