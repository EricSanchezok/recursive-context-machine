import type { RcmAst } from '../../types/api'
import type { NodeData, Wire } from '../../types/graph'

export function rcmToGraph(ast: RcmAst): { nodes: NodeData[]; wires: Wire[] } {
  const nodes: NodeData[] = []
  const wires: Wire[] = []

  if ('Primitive' in ast.body) {
    const p = ast.body.Primitive
    nodes.push({
      id: 'main',
      kind: 'accelerator',
      name: ast.name,
      purpose: p.purpose ?? '',
      model: p.models[0] ?? '',
      tools: p.tools ?? [],
      mcps: p.mcps ?? [],
      policy: p.policy ?? 'captain',
      x: 200,
      y: 200,
    })
    return { nodes, wires }
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
        x: 200 + nodes.filter((n) => n.kind === 'accelerator').length * 280,
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
        x: 200 + nodes.filter((n) => n.kind === 'accelerator').length * 280,
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
      model: '',
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
      model: '',
      tools: [],
      mcps: [],
      policy: '',
      conditionPredicate: predicateLabel(cond.predicate),
      x: 200 + nodes.length * 280,
      y: 300,
    })
  }

  for (const wire of graph.wires) {
    const fromOwner = typeof wire.from.owner === 'string' ? wire.from.owner : (wire.from.owner as { Component: string }).Component
    const toOwner = typeof wire.to.owner === 'string' ? wire.to.owner : (wire.to.owner as { Component: string }).Component
    const fromName = typeof wire.from.endpoint === 'string' ? wire.from.endpoint : JSON.stringify(wire.from.endpoint)
    const toName = typeof wire.to.endpoint === 'string' ? wire.to.endpoint : JSON.stringify(wire.to.endpoint)

    wires.push({
      id: crypto.randomUUID(),
      from: { id: crypto.randomUUID(), nodeId: fromOwner, direction: 'out', name: fromName, channel: 'pulse', index: 0 },
      to: { id: crypto.randomUUID(), nodeId: toOwner, direction: 'in', name: toName, channel: 'pulse', index: 0 },
    })
  }

  return { nodes, wires }
}

function predicateLabel(p: unknown): string {
  if (typeof p === 'string') return p
  if (!p || typeof p !== 'object') return 'predicate'
  const key = Object.keys(p)[0]
  if (!key) return 'predicate'
  return key
}
