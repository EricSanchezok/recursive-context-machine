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
    <main style={{
      position: 'relative',
      width: '100vw',
      height: '100vh',
      overflow: 'hidden',
      backgroundColor: 'var(--workspace-bg)',
      backgroundImage: 'radial-gradient(rgba(3,2,19,0.14) 1px, transparent 1px)',
      backgroundSize: '12px 12px',
    }}>
      <header style={{
        position: 'absolute',
        top: 22,
        left: 22,
        display: 'flex',
        alignItems: 'center',
        gap: 14,
        padding: '8px 18px',
        borderRadius: 18,
        background: 'rgba(255,255,255,0.92)',
        border: '1px solid rgba(3,2,19,0.08)',
        boxShadow: '0 18px 50px rgba(15,23,42,0.10)',
        backdropFilter: 'blur(18px)',
      }}>
        <img src={siiLogo} alt="SII" style={{ width: 34, height: 34, objectFit: 'contain' }} />
        <span style={{ fontSize: 29, fontWeight: 750, letterSpacing: '0.06em', color: 'var(--foreground)' }}>SII</span>
      </header>

      <section style={{
        position: 'absolute',
        left: '50%',
        top: '45%',
        transform: 'translate(-50%, -50%)',
        width: 'min(420px, calc(100vw - 80px))',
        padding: '38px 36px 34px',
        borderRadius: 30,
        textAlign: 'center',
        background: 'rgba(255,255,255,0.92)',
        border: '1px solid rgba(3,2,19,0.08)',
        boxShadow: '0 18px 50px rgba(15,23,42,0.10)',
        backdropFilter: 'blur(18px)',
      }}>
        <div style={{ width: 58, height: 58, margin: '0 auto 22px', display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
          <img src={holosLogo} alt="Holos" style={{ width: 52, height: 52, objectFit: 'contain' }} />
        </div>
        <h1 style={{ margin: 0, fontSize: 31, lineHeight: 1.15, letterSpacing: '-0.04em', color: 'var(--foreground)' }}>Holos Lab</h1>
        <p style={{ margin: '14px auto 28px', maxWidth: 310, fontSize: 14, lineHeight: 1.6, color: 'var(--muted-foreground)' }}>
          Recursive Context Machine
        </p>
        <button
          onClick={handleOpenProject}
          disabled={opening}
          style={{
            height: 44,
            minWidth: 190,
            padding: '0 18px',
            display: 'inline-flex',
            alignItems: 'center',
            justifyContent: 'center',
            gap: 10,
            border: 0,
            borderRadius: 14,
            color: 'var(--primary-foreground)',
            background: 'var(--primary)',
            fontSize: 14,
            fontWeight: 650,
            boxShadow: '0 12px 26px rgba(3,2,19,0.16)',
            cursor: opening ? 'default' : 'pointer',
            opacity: opening ? 0.58 : 1,
          }}
        >
          <FolderOpen size={18} />
          {opening ? 'Opening…' : 'Open Project'}
        </button>
        {store.projectPath && (
          <p style={{ margin: '18px 0 0', fontSize: 12, color: 'var(--muted-foreground)', whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis' }}>
            {store.projectPath}
          </p>
        )}
        {error && (
          <p style={{
            margin: '18px 0 0',
            padding: '10px 12px',
            borderRadius: 14,
            border: '1px solid var(--border)',
            background: 'var(--card)',
            color: 'var(--destructive)',
            fontSize: 12,
            lineHeight: 1.45,
            textAlign: 'left',
          }}>
            {error}
          </p>
        )}
      </section>
    </main>
  )
}
