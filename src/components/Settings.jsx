import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { open as openDialog } from '@tauri-apps/api/dialog'
import './Settings.css'

function Settings() {
  const [gamePath, setGamePath] = useState('')
  const [loading, setLoading] = useState(false)
  const [saved, setSaved] = useState(false)

  useEffect(() => {
    loadSettings()
  }, [])

  const loadSettings = async () => {
    try {
      const settings = await invoke('get_settings')
      setGamePath(settings.game_path || '')
    } catch (error) {
      console.error('Failed to load settings:', error)
    }
  }

  const selectGamePath = async () => {
    try {
      const selected = await openDialog({
        directory: true,
        multiple: false,
        title: 'Select Game Installation Directory'
      })
      
      if (selected) {
        setGamePath(selected)
      }
    } catch (error) {
      console.error('Failed to select directory:', error)
    }
  }

  const saveSettings = async () => {
    setLoading(true)
    setSaved(false)
    try {
      await invoke('save_settings', { 
        settings: { game_path: gamePath } 
      })
      setSaved(true)
      setTimeout(() => setSaved(false), 3000)
    } catch (error) {
      console.error('Failed to save settings:', error)
      alert('Failed to save settings: ' + error)
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="settings">
      <div className="settings-content">
        <h2>Settings</h2>
        
        <div className="setting-section">
          <h3>Game Configuration</h3>
          
          <div className="setting-row">
            <label>Game Installation Path:</label>
            <div className="path-selector">
              <input 
                type="text" 
                value={gamePath} 
                onChange={(e) => setGamePath(e.target.value)}
                placeholder="Select your game installation directory..."
              />
              <button onClick={selectGamePath}>Browse</button>
            </div>
            <p className="help-text">
              This should be the path to your Cyberpunk 2077 installation in Crossover
              (e.g., /Users/username/Library/Application Support/CrossOver/Bottles/...)
            </p>
          </div>
        </div>

        <div className="setting-section">
          <h3>About</h3>
          <p>Crossover Mod Manager v1.0.0</p>
          <p>A mod manager for PC games on Mac via Crossover</p>
          <p>Supports NexusMods integration for seamless mod installation</p>
        </div>

        <div className="settings-actions">
          <button 
            className="save-button"
            onClick={saveSettings}
            disabled={loading || !gamePath}
          >
            {loading ? 'Saving...' : 'Save Settings'}
          </button>
          {saved && <span className="save-success">✓ Settings saved successfully</span>}
        </div>
      </div>
    </div>
  )
}

export default Settings
