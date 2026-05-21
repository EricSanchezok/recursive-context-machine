import { getAPI } from './ipc'
import type { Inventory, RcmAst } from '../types/api'

export async function fetchInventory(projectPath: string): Promise<Inventory> {
  const raw = await getAPI().inventory(projectPath)
  return JSON.parse(raw)
}

export async function parseRcm(filePath: string): Promise<RcmAst> {
  const raw = await getAPI().parse(filePath)
  return JSON.parse(raw)
}

export async function runRcm(
  filePath: string,
  onEvent: (line: string) => void,
): Promise<string> {
  return getAPI().runStream(filePath, onEvent)
}
