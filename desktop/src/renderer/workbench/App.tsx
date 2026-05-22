import { useEffect, useState } from 'react'
import { FileText } from 'lucide-react'
import { Home } from '@workbench/Home'
import { TabBar } from '@workbench/TabBar'
import { getStore, subscribe, addTab } from '../stores/projectStore'

export function App() {
  const [store, setStore] = useState(getStore())

  useEffect(() => subscribe((nextStore) => setStore(nextStore)), [])

  if (!store.projectPath) {
    return <Home />
  }

  return (
    <div className="w-screen h-screen flex flex-col figjam-grid overflow-hidden">
      <TabBar />
      {store.activeTab ? (
        <GraphWorkspace filePath={store.activeTab} />
      ) : (
        <ProjectDashboard rcmFiles={store.rcmFiles} />
      )}
    </div>
  )
}

function ProjectDashboard({ rcmFiles }: { rcmFiles: string[] }) {
  return (
    <div className="flex-1 relative flex items-start justify-center pt-20 overflow-y-auto">
      {rcmFiles.length === 0 ? (
        <div className="absolute left-1/2 top-[42%] -translate-x-1/2 -translate-y-1/2 text-center">
          <div className="canvas-card rounded-3xl px-12 py-10">
            <div className="text-sm font-medium mb-2" style={{ color: 'var(--foreground)' }}>Empty project</div>
            <div className="text-xs" style={{ color: 'var(--muted-foreground)' }}>No .rcm files found in rcm/.</div>
          </div>
        </div>
      ) : (
        <div className="grid gap-4" style={{ gridTemplateColumns: 'repeat(auto-fill, minmax(240px, 1fr))', maxWidth: 700, width: '100%', padding: '0 24px' }}>
          {rcmFiles.map((filePath) => {
            const name = filePath.split('/').pop() ?? filePath
            return (
              <button
                key={filePath}
                onClick={() => addTab({ id: filePath, name })}
                className="floating-panel rounded-2xl px-5 py-4 text-left flex flex-col gap-2 hover:scale-[1.02] transition-transform"
              >
                <div className="flex items-center gap-2">
                  <FileText size={16} style={{ color: 'var(--muted-foreground)' }} />
                  <span className="text-sm font-semibold" style={{ color: 'var(--foreground)' }}>
                    {name.replace(/\.rcm$/, '')}
                  </span>
                </div>
                <div className="text-xs" style={{ color: 'var(--muted-foreground)' }}>
                  {filePath}
                </div>
              </button>
            )
          })}
        </div>
      )}
    </div>
  )
}

function GraphWorkspace({ filePath }: { filePath: string }) {
  const name = filePath.split('/').pop() ?? filePath
  return (
    <div className="flex-1 relative overflow-hidden">
      <div className="absolute left-1/2 top-[42%] -translate-x-1/2 -translate-y-1/2 text-center">
        <div className="canvas-card rounded-3xl px-12 py-10">
          <div className="text-sm font-medium mb-2" style={{ color: 'var(--foreground)' }}>{name}</div>
          <div className="text-xs" style={{ color: 'var(--muted-foreground)' }}>
            Graph workspace will render this file here.
          </div>
        </div>
      </div>
    </div>
  )
}
