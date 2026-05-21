import { getAPI } from '../platform/ipc'
import type { Inventory } from '../types/api'

export interface Tab {
  id: string
  name: string
}

export interface ProjectStore {
  projectPath: string | null
  inventory: Inventory | null
  tabs: Tab[]
  activeTab: string | null
}

let store: ProjectStore = {
  projectPath: null,
  inventory: null,
  tabs: [],
  activeTab: null,
}

const listeners: Array<(store: ProjectStore) => void> = []

export function getStore(): ProjectStore {
  return store
}

export function subscribe(fn: (store: ProjectStore) => void): () => void {
  listeners.push(fn)
  return () => {
    const idx = listeners.indexOf(fn)
    if (idx >= 0) listeners.splice(idx, 1)
  }
}

function emit(): void {
  for (const fn of listeners) fn(store)
}

export async function openProject(): Promise<void> {
  const api = getAPI()
  const projectPath = await api.openProject()
  if (!projectPath) return
  store.projectPath = projectPath
  store.tabs = []
  store.activeTab = null
  try {
    store.inventory = JSON.parse(await api.inventory(projectPath))
  } catch {
    store.inventory = null
  }
  emit()
}

export function addTab(tab: Tab): void {
  store.tabs.push(tab)
  store.activeTab = tab.id
  emit()
}

export function closeTab(tabId: string): void {
  store.tabs = store.tabs.filter((t) => t.id !== tabId)
  if (store.activeTab === tabId) {
    store.activeTab = store.tabs[0]?.id ?? null
  }
  emit()
}

export function setActiveTab(tabId: string): void {
  store.activeTab = tabId
  emit()
}

export function renameTab(tabId: string, name: string): void {
  const tab = store.tabs.find((t) => t.id === tabId)
  if (tab) tab.name = name
  emit()
}
