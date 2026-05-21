import { useEffect, useState } from 'react'
import { Home } from '@workbench/Home'
import { TabBar } from '@workbench/TabBar'
import { ToolBar } from '@workbench/ToolBar'
import { ZoomControls } from '@workbench/ZoomControls'
import { getStore, subscribe } from '../stores/projectStore'

export function App() {
  const [store, setStore] = useState(getStore())
  const [zoom, setZoom] = useState(1)

  useEffect(() => subscribe((nextStore) => setStore(nextStore)), [])

  if (!store.projectPath) {
    return (
      <div className="w-screen h-screen flex flex-col">
        <Home />
      </div>
    )
  }

  return (
    <div className="w-screen h-screen flex flex-col figjam-grid">
      <TabBar />
      <div className="flex-1 relative overflow-hidden">
        <Workspace projectPath={store.projectPath} />
        <ToolBar
          onAddAccelerator={() => {}}
          onAddFlux={() => {}}
          onAddCondition={() => {}}
          onAddWire={() => {}}
          onRun={() => {}}
          isRunning={false}
        />
        <ZoomControls
          zoom={zoom}
          onZoomIn={() => setZoom((value) => Math.min(2, value + 0.1))}
          onZoomOut={() => setZoom((value) => Math.max(0.25, value - 0.1))}
          onResetZoom={() => setZoom(1)}
        />
      </div>
    </div>
  )
}

function Workspace({ projectPath }: { projectPath: string }) {
  return (
    <div className="relative h-full w-full">
      <div className="absolute left-4 top-4 floating-panel rounded-2xl px-4 py-3 flex items-center gap-3">
        <div className="h-8 w-8 rounded-xl flex items-center justify-center" style={{ backgroundColor: 'var(--primary)', color: 'var(--primary-foreground)' }}>
          R
        </div>
        <div>
          <div className="text-sm font-semibold" style={{ color: 'var(--foreground)' }}>RCM workspace</div>
          <div className="text-xs max-w-[360px] truncate" style={{ color: 'var(--muted-foreground)' }}>{projectPath}</div>
        </div>
      </div>

      <div className="absolute left-1/2 top-[42%] -translate-x-1/2 -translate-y-1/2 text-center">
        <div className="canvas-card rounded-3xl px-12 py-10">
          <div className="text-sm font-medium mb-2" style={{ color: 'var(--foreground)' }}>Empty graph</div>
          <div className="text-xs" style={{ color: 'var(--muted-foreground)' }}>Use the toolbar to add accelerators, flux nodes, conditions, and wires.</div>
        </div>
      </div>
    </div>
  )
}
