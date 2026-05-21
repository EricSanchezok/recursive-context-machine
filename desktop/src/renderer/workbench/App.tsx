import { Home as HomePage } from '@workbench/Home'
import { TabBar } from '@workbench/TabBar'
import { ToolBar } from '@workbench/ToolBar'
import { getStore, subscribe } from '../stores/projectStore'

export function App() {
  const store = getStore()

  if (!store.projectPath) {
    return (
      <div className="w-screen h-screen flex flex-col">
        <TabBar />
        <div className="flex-1">
          <HomePage />
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
          onStartWire={() => {}}
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
      <p className="text-gray-400 text-sm">Workspace Canvas</p>
    </div>
  )
}
