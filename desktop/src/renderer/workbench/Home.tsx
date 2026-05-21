import { useEffect, useState } from 'react'
import { FolderOpen } from 'lucide-react'
import { openProject, subscribe, getStore } from '../stores/projectStore'
import holosLogo from '../../../resources/holos-dark.svg'
import siiLogo from '../../../resources/sii-logo.png'

export function Home() {
  const [store, setStore] = useState(getStore())
  const [error, setError] = useState<string | null>(null)
  const [opening, setOpening] = useState(false)

  useEffect(() => subscribe((nextStore) => setStore(nextStore)), [])

  async function handleOpenProject() {
    setError(null)
    setOpening(true)
    try {
      await openProject()
    } catch (cause) {
      const message = cause instanceof Error ? cause.message : String(cause)
      setError(message)
    } finally {
      setOpening(false)
    }
  }

  return (
    <main className="rcm-home figjam-grid">
      <header className="rcm-home-brand floating-panel">
        <img className="rcm-home-sii" src={siiLogo} alt="SII" />
      </header>

      <section className="rcm-home-card floating-panel">
        <div className="rcm-home-mark">
          <img src={holosLogo} alt="Holos" />
        </div>
        <p className="rcm-home-kicker">RCM</p>
        <h1>Recursive Context Machine</h1>
        <p className="rcm-home-subtitle">
          Compose accelerators into recursive context graphs.
        </p>
        <button className="rcm-home-open" onClick={handleOpenProject} disabled={opening}>
          <FolderOpen size={18} />
          {opening ? 'Opening…' : 'Open Project'}
        </button>
        {store.projectPath && <p className="rcm-home-path">{store.projectPath}</p>}
        {error && <p className="rcm-home-error">{error}</p>}
      </section>
    </main>
  )
}
