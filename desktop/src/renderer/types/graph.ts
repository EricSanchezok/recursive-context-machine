export type NodeKind = 'accelerator' | 'flux' | 'condition'

export interface Port {
  id: string
  nodeId: string
  direction: 'in' | 'out'
  name: string
  channel: string
  index: number
}

export interface Wire {
  id: string
  from: Port
  to: Port
}

export interface NodeData {
  id: string
  kind: NodeKind
  name: string
  purpose: string
  model?: string
  tools: string[]
  mcps: string[]
  policy: string
  fluxMode?: string
  fluxChannel?: string
  fluxArity?: number
  conditionPredicate?: string
  x: number
  y: number
}
