import type { RcmAst } from '../../types/api'
import type { NodeData, Wire, Port } from '../../types/graph'

export function rcmToGraph(ast: RcmAst): { nodes: NodeData[]; wires: Wire[]; models: ModelDef[] } {
  const nodes: NodeData[] = []
  const wires: Wire[] = []
  const models: ModelDef[] = []

  for (const m of ast.models) {
    models.push({ id: m.id, protocol: m.protocol, endpoint: m.endpoint, credentials_env: m.credentials_env })
  }

  if ('Primitive' in ast.body) {
    const primitive = ast.body.Primitive
    nodes.push({
      id: 'main',
      kind: 'accelerator',
      name: ast.name,
      purpose: primitive.purpose ?? '',
      model: primitive.models[0] ?? '',
      tools: primitive.tools ?? [],
      mcps: primitive.mcps ?? [],
      policy: primitive.policy ?? 'captain',
      x: 200,
      y: 200,
    })
    return { nodes, wires, models }
  }

  const graph = ast.body.Graph
  for (const accel of graph.accelerators) {
    if ('Inline' in accel.source) {
      const p = accel.source.Inline
      nodes.push({
        id: accel.id,
        kind: 'accelerator',
        name: accel.id,
        purpose: p.purpose ?? '',
        model: p.models[0] ?? '',
        tools: p.tools ?? [],
        mcps: p.mcps ?? [],
        policy: p.policy ?? 'captain',
        x: 200 + nodes.length * 280,
        y: 200,
      })
    } else {
      nodes.push({
        id: accel.id,
        kind: 'accelerator',
        name: accel.id,
        purpose: '',
        model: '',
        tools: [],
        mcps: [],
        policy: 'captain',
        x: 200 + nodes.length * 280,
        y: 200,
      })
    }
  }

  for (const flux of graph.fluxes) {
    nodes.push({
      id: flux.id,
      kind: 'flux',
      name: flux.name ?? flux.id,
      purpose: '',
      tools: [],
      mcps: [],
      policy: '',
      fluxMode: flux.mode,
      fluxChannel: flux.channel,
      fluxArity: flux.arity,
      x: 200 + nodes.length * 280,
      y: 300,
    })
  }

  for (const cond of graph.conditions) {
    nodes.push({
      id: cond.id,
      kind: 'condition',
      name: cond.name ?? cond.id,
      purpose: '',
      tools: [],
      mcps: [],
      policy: '',
      x: 200 + nodes.length * 280,
      y: 300,
    })
  }

  for (const wire of graph.wires) {
    wires.push({
      id: crypto.randomUUID(),
      from: makePort(wire.from, 'out'),
      to: makePort(wire.to, 'in'),
    })
  }

  return { nodes, wires, models }
}

interface ModelDef {
  id: string
  protocol: string
  endpoint?: string
  credentials_env?: string
}

function makePort(
  raw: { owner: string | { Component: string }; endpoint: string | Record<string, unknown> },
  _direction: 'in' | 'out',
): Port {
  const ownerStr = typeof raw.owner === 'string' ? raw.owner : (raw.owner as { Component: string }).Component
  const endpointStr =
    typeof raw.endpoint === 'string'
      ? raw.endpoint
      : typeof raw.endpoint === 'object' && 'State' in raw.endpoint
        ? (raw.endpoint as { State: string }).State
        : 'done'
  return {
    id: crypto.randomUUID(),
    nodeId: ownerStr === 'Input' ? 'input' : ownerStr === 'Output' ? 'output' : ownerStr,
    direction: ownerStr === 'Input' ? 'in' : ownerStr === 'Output' ? 'out' : _direction,
    name: endpointStr,
    channel: 'pulse',
    index: 0,
  }
}
