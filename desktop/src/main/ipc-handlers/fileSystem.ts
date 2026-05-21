import { ipcMain, dialog } from 'electron'
import { readFile, writeFile, readdir, stat } from 'node:fs/promises'
import { join } from 'node:path'

export function fileSystemHandlers(): void {
  ipcMain.handle('fs:openProject', async (): Promise<string | null> => {
    const result = await dialog.showOpenDialog({
      properties: ['openDirectory'],
      title: 'Open Project Folder',
    })
    return result.canceled ? null : result.filePaths[0]
  })

  ipcMain.handle('fs:readFile', async (_event, filePath: string): Promise<string> => {
    return readFile(filePath, 'utf-8')
  })

  ipcMain.handle('fs:writeFile', async (_event, filePath: string, content: string): Promise<void> => {
    await writeFile(filePath, content, 'utf-8')
  })

  ipcMain.handle('fs:listDir', async (_event, dirPath: string): Promise<FileEntry[]> => {
    const names = await readdir(dirPath)
    const entries: FileEntry[] = []
    for (const name of names) {
      const fullPath = join(dirPath, name)
      const s = await stat(fullPath)
      entries.push({
        name,
        path: fullPath,
        isDirectory: s.isDirectory(),
        extension: name.includes('.') ? name.split('.').pop() ?? '' : '',
      })
    }
    return entries
  })
}

export interface FileEntry {
  name: string
  path: string
  isDirectory: boolean
  extension: string
}
