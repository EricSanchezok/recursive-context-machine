import { useState, useEffect } from 'react'
import { Plus, X } from 'lucide-react'
import { getStore, addTab, closeTab, setActiveTab, renameTab, subscribe } from '../stores/projectStore'

export function TabBar() {
  const [store, setStore] = useState(getStore())
  const [editingTab, setEditingTab] = useState<string | null>(null)
  const [editName, setEditName] = useState('')

  useEffect(() => subscribe((s) => setStore(s)), [])

  if (!store.projectPath) return null

  return (
    <div style={{
      height: 48,
      display: 'flex',
      alignItems: 'center',
      padding: '0 8px',
      gap: 8,
      borderBottom: '1px solid var(--border)',
      backgroundColor: 'var(--card)',
    }}>
      <div style={{ display: 'flex', flex: 1, alignItems: 'center', gap: 4, overflow: 'auto' }}>
        {store.tabs.map((tab) => {
          const active = store.activeTab === tab.id
          return (
            <div
              key={tab.id}
              onClick={() => setActiveTab(tab.id)}
              onDoubleClick={() => { setEditingTab(tab.id); setEditName(tab.name) }}
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: 8,
                padding: '6px 12px',
                borderRadius: 8,
                cursor: 'pointer',
                minWidth: 120,
                maxWidth: 200,
                backgroundColor: active ? 'var(--primary)' : 'transparent',
                color: active ? 'var(--primary-foreground)' : 'var(--foreground)',
                fontWeight: active ? 600 : 400,
              }}
            >
              {editingTab === tab.id ? (
                <input
                  type="text"
                  value={editName}
                  onChange={(e) => setEditName(e.target.value)}
                  onBlur={() => { renameTab(tab.id, editName); setEditingTab(null) }}
                  onKeyDown={(e) => {
                    if (e.key === 'Enter') { renameTab(tab.id, editName); setEditingTab(null) }
                  }}
                  style={{ flex: 1, background: 'transparent', outline: 'none', border: 'none', fontSize: 13 }}
                  autoFocus
                />
              ) : (
                <span style={{ flex: 1, fontSize: 13, overflow: 'hidden', textOverflow: 'ellipsis' }}>
                  {tab.name}
                </span>
              )}
              <button
                onClick={(e) => { e.stopPropagation(); closeTab(tab.id) }}
                style={{ opacity: 0.4, background: 'none', border: 'none', cursor: 'pointer', padding: 2, borderRadius: 4 }}
              >
                <X size={14} />
              </button>
            </div>
          )
        })}
        <button
          onClick={() => addTab({ id: crypto.randomUUID(), name: 'untitled.rcm' })}
          style={{ padding: 8, background: 'none', border: 'none', cursor: 'pointer', borderRadius: 8 }}
        >
          <Plus size={18} style={{ color: 'var(--muted-foreground)' }} />
        </button>
      </div>
    </div>
  )
}
