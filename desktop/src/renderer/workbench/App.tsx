import { useState, useEffect } from 'react'
import { Home } from '@workbench/Home'
import { TabBar } from '@workbench/TabBar'
import { ToolBar } from '@workbench/ToolBar'
import { getStore, subscribe, type ProjectStore } from '../stores/projectStore'

export function App() {
  const [store, setStore] = useState(getStore())

  useEffect(() => subscribe((s) => setStore(s)), [])

  if (!store.projectPath) {
    return (
      <div className="w-screen h-screen flex flex-col">
        <TabBar />
        <div className="flex-1">
          <Home />
        </div>
      </div>
    )
  }

  return (
    <div className="w-screen h-screen flex flex-col">
      <TabBar />
      <div className="flex-1 relative">
        <Workspace />
        <ToolBar
          onAddAccelerator={() => {}}
          onAddFlux={() => {}}
          onAddCondition={() => {}}
          onAddWire={() => {}}
          onRun={() => {}}
          isRunning={false}
        />
      </div>
    </div>
  )
}

function Workspace() {
  return (
    <div
      className="w-full h-full flex items-center justify-center"
      style={{ backgroundColor: 'var(--workspace-bg)' }}
    >
      <p className="text-muted-foreground text-sm">Workspace Canvas</p>
    </div>
  )
}
