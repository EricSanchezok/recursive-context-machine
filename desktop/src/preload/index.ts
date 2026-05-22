import { contextBridge, ipcRenderer } from 'electron'

const api = {
  openProject: (): Promise<string | null> => ipcRenderer.invoke('fs:openProject'),
  readFile: (filePath: string): Promise<string> => ipcRenderer.invoke('fs:readFile', filePath),
  writeFile: (filePath: string, content: string): Promise<void> =>
    ipcRenderer.invoke('fs:writeFile', filePath, content),
  listDir: (dirPath: string): Promise<FileEntry[]> => ipcRenderer.invoke('fs:listDir', dirPath),
  inventory: (projectPath: string): Promise<string> => ipcRenderer.invoke('accelerate:inventory', projectPath),
  parse: (filePath: string): Promise<string> => ipcRenderer.invoke('accelerate:parse', filePath),
  onRunLine: (callback: (line: string) => void): (() => void) => {
    const handler = (_event: Electron.IpcRendererEvent, line: string) => callback(line)
    ipcRenderer.on('accelerate:run-line', handler)
    return () => ipcRenderer.removeListener('accelerate:run-line', handler)
  },
  runStream: (filePath: string): Promise<string> => ipcRenderer.invoke('accelerate:run', filePath),
}

contextBridge.exposeInMainWorld('electronAPI', api)

interface FileEntry {
  name: string
  path: string
  isDirectory: boolean
  extension: string
}

export type ElectronAPI = typeof api
