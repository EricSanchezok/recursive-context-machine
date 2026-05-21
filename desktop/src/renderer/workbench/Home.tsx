import { useState } from 'react'
import { FolderOpen } from 'lucide-react'
import { openProject, getStore, subscribe, type ProjectStore } from '../stores/projectStore'

export function Home() {
  const [, setTick] = useState(0)

  return (
    <div className="w-full h-full flex flex-col items-center justify-center bg-[var(--workspace-bg)]">
      <div className="text-center space-y-6">
        <h1 className="text-4xl font-bold" style={{ color: 'var(--foreground)' }}>
          RICA
        </h1>
        <p className="text-lg" style={{ color: 'var(--muted-foreground)' }}>
          Composable AI Pipelines
        </p>
        <button
          onClick={async () => {
            await openProject()
            setTick((t) => t + 1)
          }}
          className="flex items-center gap-2 px-6 py-3 rounded-xl text-white font-medium
                     transition-colors shadow-lg hover:shadow-xl"
          style={{ backgroundColor: 'var(--primary)' }}
        >
          <FolderOpen size={20} />
          Open Project
        </button>
      </div>
    </div>
  )
}

Home.shouldDisplay = (store: ProjectStore) => !store.projectPath
