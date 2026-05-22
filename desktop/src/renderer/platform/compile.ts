import type { RcmAst } from '../types/api'
import { getAPI } from './ipc'

export async function parseRcm(filePath: string): Promise<RcmAst> {
  const raw = await getAPI().parse(filePath)
  return JSON.parse(raw)
}

export async function fetchInventory(projectPath: string) {
  const raw = await getAPI().inventory(projectPath)
  return JSON.parse(raw)
}
