import { ElectronAPI } from '../../preload/index'

declare global {
  interface Window {
    electronAPI: ElectronAPI
  }
}

export function getAPI(): ElectronAPI {
  if (!window.electronAPI) {
    throw new Error('electronAPI is not available (running outside Electron?)')
  }
  return window.electronAPI
}
