import type { GraphNode, Port } from '../../types/graph'

export function acceleratorNode(input: {
  id: string
  name: string
  x: number
  y: number
  purpose?: string
  model?: string
  tools?: string[]
  mcps?: string[]
  policy?: string
}): GraphNode {
  return {
    kind: 'accelerator',
    id: input.id,
    name: input.name,
    position: { x: input.x, y: input.y },
    purpose: input.purpose ?? '',
    model: input.model ?? '',
    tools: input.tools ?? [],
    mcps: input.mcps ?? [],
    policy: input.policy ?? 'captain',
    ports: [
      port(input.id, 'in', 'trigger', 'pulse', 0),
      port(input.id, 'in', 'purpose', 'purpose', 1),
      port(input.id, 'in', 'context', 'context', 2),
      port(input.id, 'in', 'resources', 'resources', 3),
      port(input.id, 'out', 'done', 'pulse', 0),
      port(input.id, 'out', 'purpose', 'purpose', 1),
      port(input.id, 'out', 'context', 'context', 2),
      port(input.id, 'out', 'resources', 'resources', 3),
    ],
  }
}

export function fluxNode(input: {
  id: string
  name: string
  x: number
  y: number
  mode?: string
  channel?: string
  arity?: number
}): GraphNode {
  const channel = input.channel ?? 'context'
  const arity = input.arity ?? 2
  return {
    kind: 'flux',
    id: input.id,
    name: input.name,
    position: { x: input.x, y: input.y },
    mode: input.mode ?? 'append',
    channel,
    arity,
    ports: [
      ...Array.from({ length: arity }, (_, index) => port(input.id, 'in', `slot(${index})`, channel, index)),
      port(input.id, 'out', 'out', channel, 0),
    ],
  }
}

export function conditionNode(input: {
  id: string
  name: string
  x: number
  y: number
  predicate?: string
}): GraphNode {
  return {
    kind: 'condition',
    id: input.id,
    name: input.name,
    position: { x: input.x, y: input.y },
    predicate: input.predicate ?? 'predicate',
    ports: [
      port(input.id, 'in', 'trigger', 'pulse', 0),
      port(input.id, 'out', 'true', 'pulse', 0),
      port(input.id, 'out', 'false', 'pulse', 1),
    ],
  }
}

export function textNode(input: { id: string; name: string; x: number; y: number; text?: string }): GraphNode {
  return {
    kind: 'text',
    id: input.id,
    name: input.name,
    position: { x: input.x, y: input.y },
    text: input.text ?? 'New text',
    ports: [port(input.id, 'out', 'value', 'purpose', 0)],
  }
}

export function port(nodeId: string, direction: 'in' | 'out', name: string, channel: string, index: number): Port {
  return { id: `${nodeId}:${direction}:${name}`, nodeId, direction, name, channel, index }
}
