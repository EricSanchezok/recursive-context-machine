import { ipcMain } from 'electron'
import { spawn } from 'node:child_process'
import { join } from 'node:path'

const ACCELERATE_BIN = process.env['ACCELERATE_PATH'] || 'accelerate'

function accelerate(args: string[], cwd?: string): Promise<string> {
  return new Promise((resolve, reject) => {
    const child = spawn(ACCELERATE_BIN, args, {
      cwd: cwd || process.cwd(),
      env: { ...process.env },
    })
    let stdout = ''
    let stderr = ''
    child.stdout.on('data', (chunk: Buffer) => {
      stdout += chunk.toString()
    })
    child.stderr.on('data', (chunk: Buffer) => {
      stderr += chunk.toString()
    })
    child.on('close', (code) => {
      if (code === 0) {
        resolve(stdout)
      } else {
        reject(new Error(stderr || `accelerate exited with code ${code}`))
      }
    })
    child.on('error', reject)
  })
}

export function accelerateHandlers(): void {
  ipcMain.handle(
    'accelerate:inventory',
    async (_event, projectPath: string): Promise<string> => {
      return accelerate(['inventory', '--project', projectPath])
    },
  )

  ipcMain.handle(
    'accelerate:parse',
    async (_event, filePath: string): Promise<string> => {
      return accelerate(['parse', filePath])
    },
  )

  ipcMain.handle(
    'accelerate:run',
    (_event, filePath: string, onEvent: (line: string) => void): Promise<string> => {
      return new Promise((resolve, reject) => {
        const child = spawn(ACCELERATE_BIN, ['run', filePath, '--stream'], {
          cwd: process.cwd(),
          env: { ...process.env },
        })
        let stderr = ''
        child.stdout.on('data', (chunk: Buffer) => {
          for (const line of chunk.toString().split('\n').filter(Boolean)) {
            onEvent(line)
          }
        })
        child.stderr.on('data', (chunk: Buffer) => {
          stderr += chunk.toString()
        })
        child.on('close', (code) => {
          resolve(code === 0 ? '' : stderr)
        })
        child.on('error', reject)
      })
    },
  )
}
