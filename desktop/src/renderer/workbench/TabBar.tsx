import { useState } from 'react'
import { Home, Plus, X } from 'lucide-react'
import { getStore, addTab, closeTab, setActiveTab, renameTab, subscribe, type ProjectStore } from '../stores/projectStore'

export function TabBar() {
  const [store, setStore] = useState(getStore())
  const [editingTab, setEditingTab] = useState<string | null>(null)
  const [editName, setEditName] = useState('')

  useState(() =>
    subscribe((s) => setStore(s))
  )

  if (!store.projectPath) return null

  return (
    <div
      className="h-12 flex items-center px-2 gap-2 border-b"
      style={{ backgroundColor: 'var(--card)', borderColor: 'var(--border)' }}
    >
      <HomeButton />
      <div className="flex-1 flex items-center gap-1 overflow-x-auto">
        {store.tabs.map((tab) => (
          <div
            key={tab.id}
            className={`
              group flex items-center gap-2 px-3 py-1.5 rounded-lg cursor-pointer
              transition-all min-w-[120px] max-w-[200px]
              ${store.activeTab === tab.id
                ? 'bg-purple-100 text-purple-900'
                : 'hover:bg-gray-100 text-gray-700'
              }
            `}
            onClick={() => setActiveTab(tab.id)}
            onDoubleClick={() => {
              setEditingTab(tab.id)
              setEditName(tab.name)
            }}
          >
            {editingTab === tab.id ? (
              <input
                type="text"
                value={editName}
                onChange={(e) => setEditName(e.target.value)}
                onBlur={() => {
                  renameTab(tab.id, editName)
                  setEditingTab(null)
                }}
                onKeyDown={(e) => {
                  if (e.key === 'Enter') {
                    renameTab(tab.id, editName)
                    setEditingTab(null)
                  }
                }}
                className="flex-1 bg-transparent outline-none text-sm"
                autoFocus
              />
            ) : (
              <span className="flex-1 text-sm truncate">{tab.name}</span>
            )}
            <button
              onClick={(e) => {
                e.stopPropagation()
                closeTab(tab.id)
              }}
              className="opacity-0 group-hover:opacity-100 hover:bg-white/50 rounded p-0.5 transition-opacity"
            >
              <X size={14} />
            </button>
          </div>
        ))}
        <button
          onClick={() =>
            addTab({ id: crypto.randomUUID(), name: 'untitled.rcm' })
          }
          className="p-2 hover:bg-gray-100 rounded-lg transition-colors"
        >
          <Plus size={18} className="text-gray-600" />
        </button>
      </div>
    </div>
  )
}

function HomeButton() {
  return (
    <button className="p-2 hover:bg-gray-100 rounded-lg transition-colors" title="Home">
      <Home size={20} className="text-gray-600" />
    </button>
  )
}
