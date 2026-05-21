import { contextBridge, ipcRenderer } from 'electron'

const api = {
  openProject: (): Promise<string | null> => ipcRenderer.invoke('fs:openProject'),
  readFile: (filePath: string): Promise<string> => ipcRenderer.invoke('fs:readFile', filePath),
  writeFile: (filePath: string, content: string): Promise<void> =>
    ipcRenderer.invoke('fs:writeFile', filePath, content),
  listDir: (dirPath: string): Promise<FileEntry[]> => ipcRenderer.invoke('fs:listDir', dirPath),
  inventory: (projectPath: string): Promise<string> =>
    ipcRenderer.invoke('accelerate:inventory', projectPath),
  parse: (filePath: string): Promise<string> =>
    ipcRenderer.invoke('accelerate:parse', filePath),
  runStream: (filePath: string, onLine: (line: string) => void): Promise<string> =>
    ipcRenderer.invoke('accelerate:run', filePath, onLine),
}

contextBridge.exposeInMainWorld('electronAPI', api)

interface FileEntry {
  name: string
  path: string
  isDirectory: boolean
  extension: string
}

export type ElectronAPI = typeof api
