import { useEffect, useState } from 'react'
import { FileText } from 'lucide-react'
import { Home } from '@workbench/Home'
import { TabBar } from '@workbench/TabBar'
import { getStore, subscribe, addTab } from '../stores/projectStore'

export function App() {
  const [store, setStore] = useState(getStore())

  useEffect(() => subscribe((nextStore) => {
    setStore(nextStore)
  }), [])

  if (!store.projectPath) {
    return <Home />
  }

  const rcmFiles = store.rcmFiles ?? []

  return (
    <div style={{ width: '100vw', height: '100vh', display: 'flex', flexDirection: 'column', overflow: 'hidden' }}>
      <TabBar />
      <div style={{
        flex: 1,
        backgroundColor: 'var(--workspace-bg)',
        backgroundImage: 'radial-gradient(rgba(3,2,19,0.14) 1px, transparent 1px)',
        backgroundSize: '12px 12px',
        display: 'flex',
        alignItems: 'flex-start',
        justifyContent: 'center',
        paddingTop: 80,
        overflow: 'auto',
      }}>
        {rcmFiles.length === 0 ? (
          <div style={{ textAlign: 'center', marginTop: '16vh' }}>
            <div className="canvas-card" style={{ padding: '40px 48px', borderRadius: 24 }}>
              <div style={{ fontSize: 14, fontWeight: 500, marginBottom: 8, color: 'var(--foreground)' }}>Empty project</div>
              <div style={{ fontSize: 12, color: 'var(--muted-foreground)' }}>No .rcm files found in rcm/.</div>
            </div>
          </div>
        ) : (
          <div style={{
            display: 'grid',
            gridTemplateColumns: 'repeat(auto-fill, minmax(240px, 1fr))',
            gap: 16,
            width: '100%',
            maxWidth: 700,
            padding: '0 24px',
          }}>
            {rcmFiles.map((filePath) => {
              const name = filePath.split('/').pop() ?? filePath
              return (
                <button
                  key={filePath}
                  onClick={() => addTab({ id: filePath, name })}
                  style={{
                    textAlign: 'left',
                    cursor: 'pointer',
                    border: 'none',
                    background: 'rgba(255,255,255,0.92)',
                    borderRadius: 16,
                    padding: '16px 20px',
                    boxShadow: '0 18px 50px rgba(15,23,42,0.10)',
                    backdropFilter: 'blur(18px)',
                    display: 'flex',
                    flexDirection: 'column',
                    gap: 8,
                  }}
                >
                  <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                    <FileText size={16} style={{ color: 'var(--muted-foreground)' }} />
                    <span style={{ fontSize: 14, fontWeight: 600, color: 'var(--foreground)' }}>
                      {name.replace(/\.rcm$/, '')}
                    </span>
                  </div>
                  <div style={{ fontSize: 12, color: 'var(--muted-foreground)' }}>
                    {filePath}
                  </div>
                </button>
              )
            })}
          </div>
        )}
      </div>
    </div>
  )
}
