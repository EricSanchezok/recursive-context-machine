import type { RcmAst } from '../../types/api'
import type { GraphNode, Port, Wire } from '../../types/graph'

export function rcmToGraph(ast: RcmAst): { nodes: GraphNode[]; wires: Wire[] } {
  const nodes: GraphNode[] = []
  const wires: Wire[] = []

  if ('Primitive' in ast.body) {
    const primitive = ast.body.Primitive
    nodes.push(acceleratorNode({
      id: 'main',
      name: ast.name,
      x: 200,
      y: 200,
      purpose: primitive.purpose ?? '',
      model: primitive.models[0] ?? '',
      tools: primitive.tools ?? [],
      mcps: primitive.mcps ?? [],
      policy: primitive.policy ?? 'captain',
    }))
    return { nodes, wires }
  }

  const graph = ast.body.Graph
  let acceleratorIndex = 0

  for (const accel of graph.accelerators) {
    if ('Inline' in accel.source) {
      const primitive = accel.source.Inline
      nodes.push(acceleratorNode({
        id: accel.id,
        name: accel.id,
        x: 200 + acceleratorIndex * 280,
        y: 200,
        purpose: primitive.purpose ?? '',
        model: primitive.models[0] ?? '',
        tools: primitive.tools ?? [],
        mcps: primitive.mcps ?? [],
        policy: primitive.policy ?? 'captain',
      }))
    } else {
      nodes.push(acceleratorNode({
        id: accel.id,
        name: accel.id,
        x: 200 + acceleratorIndex * 280,
        y: 200,
        purpose: '',
        model: '',
        tools: [],
        mcps: [],
        policy: 'captain',
      }))
    }
    acceleratorIndex += 1
  }

  for (const flux of graph.fluxes) {
    nodes.push(fluxNode({
      id: flux.id,
      name: flux.name ?? flux.id,
      x: 200 + nodes.length * 260,
      y: 360,
      mode: flux.mode,
      channel: flux.channel,
      arity: flux.arity,
    }))
  }

  for (const condition of graph.conditions) {
    nodes.push(conditionNode({
      id: condition.id,
      name: condition.name ?? condition.id,
      x: 200 + nodes.length * 260,
      y: 360,
      predicate: predicateLabel(condition.predicate),
    }))
  }

  for (const wire of graph.wires) {
    const fromOwner = ownerName(wire.from.owner)
    const toOwner = ownerName(wire.to.owner)
    const fromName = endpointName(wire.from.endpoint)
    const toName = endpointName(wire.to.endpoint)

    wires.push({
      id: crypto.randomUUID(),
      from: portFromNode(nodes, fromOwner, fromName, 'out'),
      to: portFromNode(nodes, toOwner, toName, 'in'),
    })
  }

  return { nodes, wires }
}

function acceleratorNode(input: {
  id: string
  name: string
  x: number
  y: number
  purpose: string
  model: string
  tools: string[]
  mcps: string[]
  policy: string
}): GraphNode {
  return {
    kind: 'accelerator',
    id: input.id,
    name: input.name,
    position: { x: input.x, y: input.y },
    purpose: input.purpose,
    model: input.model,
    tools: input.tools,
    mcps: input.mcps,
    policy: input.policy,
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

function fluxNode(input: {
  id: string
  name: string
  x: number
  y: number
  mode: string
  channel: string
  arity: number
}): GraphNode {
  return {
    kind: 'flux',
    id: input.id,
    name: input.name,
    position: { x: input.x, y: input.y },
    mode: input.mode,
    channel: input.channel,
    arity: input.arity,
    ports: [
      ...Array.from({ length: input.arity }, (_, index) =>
        port(input.id, 'in', `slot(${index})`, input.channel, index),
      ),
      port(input.id, 'out', 'out', input.channel, 0),
    ],
  }
}

function conditionNode(input: {
  id: string
  name: string
  x: number
  y: number
  predicate: string
}): GraphNode {
  return {
    kind: 'condition',
    id: input.id,
    name: input.name,
    position: { x: input.x, y: input.y },
    predicate: input.predicate,
    ports: [
      port(input.id, 'in', 'trigger', 'pulse', 0),
      port(input.id, 'out', 'true', 'pulse', 0),
      port(input.id, 'out', 'false', 'pulse', 1),
    ],
  }
}

export function textNode(input: { id: string; name: string; x: number; y: number; text: string }): GraphNode {
  return {
    kind: 'text',
    id: input.id,
    name: input.name,
    position: { x: input.x, y: input.y },
    text: input.text,
    ports: [port(input.id, 'out', 'value', 'purpose', 0)],
  }
}

function port(nodeId: string, direction: 'in' | 'out', name: string, channel: string, index: number): Port {
  return { id: `${nodeId}:${direction}:${name}`, nodeId, direction, name, channel, index }
}

function portFromNode(nodes: GraphNode[], nodeId: string, name: string, direction: 'in' | 'out'): Port {
  const node = nodes.find((candidate) => candidate.id === nodeId)
  const match = node?.ports.find((candidate) => candidate.direction === direction && candidate.name === name)
  return match ?? port(nodeId, direction, name, 'pulse', 0)
}

function ownerName(owner: string | { Component: string }): string {
  return typeof owner === 'string' ? owner : owner.Component
}

function endpointName(endpoint: string | Record<string, unknown>): string {
  if (typeof endpoint === 'string') {
    if (endpoint === 'ConditionTrue') return 'true'
    if (endpoint === 'ConditionFalse') return 'false'
    if (endpoint === 'FluxOut') return 'out'
    return endpoint.toLowerCase()
  }
  if ('State' in endpoint) return endpoint.State as string
  if ('FluxSlot' in endpoint) return `slot(${endpoint.FluxSlot})`
  return 'done'
}

function predicateLabel(predicate: unknown): string {
  if (typeof predicate === 'string') return predicate
  if (!predicate || typeof predicate !== 'object') return 'predicate'
  return Object.keys(predicate)[0] ?? 'predicate'
}
