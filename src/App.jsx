import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import ModList from './components/ModList'
import ModDetails from './components/ModDetails'
import Settings from './components/Settings'
import './App.css'

function App() {
  const [mods, setMods] = useState([])
  const [selectedMod, setSelectedMod] = useState(null)
  const [activeTab, setActiveTab] = useState('mods')
  const [loading, setLoading] = useState(false)

  useEffect(() => {
    loadMods()
  }, [])

  const loadMods = async () => {
    try {
      const modList = await invoke('get_installed_mods')
      setMods(modList)
    } catch (error) {
      console.error('Failed to load mods:', error)
    }
  }

  const handleInstallMod = async (modData) => {
    setLoading(true)
    try {
      await invoke('install_mod', { modData })
      await loadMods()
    } catch (error) {
      console.error('Failed to install mod:', error)
      alert('Failed to install mod: ' + error)
    } finally {
      setLoading(false)
    }
  }

  const handleRemoveMod = async (modId) => {
    setLoading(true)
    try {
      await invoke('remove_mod', { modId })
      await loadMods()
      setSelectedMod(null)
    } catch (error) {
      console.error('Failed to remove mod:', error)
      alert('Failed to remove mod: ' + error)
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="app">
      <header className="app-header">
        <h1>Crossover Mod Manager</h1>
        <nav className="tabs">
          <button 
            className={activeTab === 'mods' ? 'active' : ''} 
            onClick={() => setActiveTab('mods')}
          >
            Mods
          </button>
          <button 
            className={activeTab === 'settings' ? 'active' : ''} 
            onClick={() => setActiveTab('settings')}
          >
            Settings
          </button>
        </nav>
      </header>

      <main className="app-content">
        {activeTab === 'mods' ? (
          <div className="mod-manager">
            <ModList 
              mods={mods} 
              selectedMod={selectedMod}
              onSelectMod={setSelectedMod}
              loading={loading}
            />
            <ModDetails 
              mod={selectedMod}
              onRemove={handleRemoveMod}
              loading={loading}
            />
          </div>
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
  )
}

export default App
