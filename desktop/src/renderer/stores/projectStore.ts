import { getAPI } from '../platform/ipc'
import type { Inventory } from '../types/api'

async function listRcmFiles(dir: string): Promise<string[]> {
  const api = getAPI()
  const rcmDir = `${dir}/rcm`
  try {
    const entries = await api.listDir(rcmDir)
    return entries
      .filter((entry) => entry.extension === 'rcm')
      .map((entry) => entry.path)
  } catch (err) {
    console.error('listRcmFiles failed:', rcmDir, err)
    return []
  }
}

export interface Tab {
  id: string
  name: string
}

export interface ProjectStore {
  projectPath: string | null
  rcmFiles: string[]
  inventory: Inventory | null
  tabs: Tab[]
  activeTab: string | null
}

let store: ProjectStore = {
  projectPath: null,
  rcmFiles: [],
  inventory: null,
  tabs: [],
  activeTab: null,
}

const listeners: Array<(store: ProjectStore) => void> = []

export function getStore(): ProjectStore {
  return snapshot()
}

export function subscribe(fn: (store: ProjectStore) => void): () => void {
  listeners.push(fn)
  return () => {
    const index = listeners.indexOf(fn)
    if (index >= 0) listeners.splice(index, 1)
  }
}

function emit(): void {
  const next = snapshot()
  for (const listener of listeners) listener(next)
}

function snapshot(): ProjectStore {
  return {
    ...store,
    rcmFiles: [...store.rcmFiles],
    tabs: [...store.tabs],
  }
}

export async function openProject(): Promise<void> {
  const api = getAPI()
  const projectPath = await api.openProject()
  if (!projectPath) return

  const rcmFiles = await listRcmFiles(projectPath)
  let inventory: Inventory | null = null
  try {
    inventory = JSON.parse(await api.inventory(projectPath))
  } catch (err) {
    console.error('accelerate inventory failed:', err)
  }

  store = {
    projectPath,
    rcmFiles,
    inventory,
    tabs: [],
    activeTab: null,
  }
  emit()
}

export function showHome(): void {
  store = { ...store, activeTab: null }
  emit()
}

export function addTab(tab: Tab): void {
  const tabs = store.tabs.some((existing) => existing.id === tab.id)
    ? store.tabs
    : [...store.tabs, tab]
  store = { ...store, tabs, activeTab: tab.id }
  emit()
}

export function closeTab(tabId: string): void {
  const tabs = store.tabs.filter((tab) => tab.id !== tabId)
  const activeTab = store.activeTab === tabId ? tabs[0]?.id ?? null : store.activeTab
  store = { ...store, tabs, activeTab }
  emit()
}

export function setActiveTab(tabId: string): void {
  store = { ...store, activeTab: tabId }
  emit()
}

export function renameTab(tabId: string, name: string): void {
  const tabs = store.tabs.map((tab) => (tab.id === tabId ? { ...tab, name } : tab))
  store = { ...store, tabs }
  emit()
}
