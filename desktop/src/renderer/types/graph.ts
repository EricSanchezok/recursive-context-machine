export type NodeKind = 'accelerator' | 'flux' | 'condition' | 'text'

export interface Position {
  x: number
  y: number
}

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

interface NodeBase<K extends NodeKind> {
  id: string
  kind: K
  name: string
  position: Position
  ports: Port[]
}

export interface AcceleratorGraphNode extends NodeBase<'accelerator'> {
  purpose: string
  model?: string
  tools: string[]
  mcps: string[]
  policy: string
}

export interface FluxGraphNode extends NodeBase<'flux'> {
  mode: string
  channel: string
  arity: number
}

export interface ConditionGraphNode extends NodeBase<'condition'> {
  predicate: string
}

export interface TextGraphNode extends NodeBase<'text'> {
  text: string
}

export type GraphNode = AcceleratorGraphNode | FluxGraphNode | ConditionGraphNode | TextGraphNode
