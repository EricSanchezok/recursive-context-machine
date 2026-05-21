import type { NodeData, Wire } from '../../types/graph'

function indent(level: number): string {
  return '    '.repeat(level)
}

export function graphToRcm(
  name: string,
  nodes: NodeData[],
  wires: Wire[],
  models: Array<{ id: string; protocol: string; endpoint?: string; credentials_env?: string }>,
): string {
  let rcm = `name = "${name}"\n\n`

  for (const model of models) {
    rcm += `model ${model.id} {\n`
    rcm += `${indent(1)}protocol = "${model.protocol}"\n`
    if (model.endpoint) {
      rcm += `${indent(1)}endpoint = "${model.endpoint}"\n`
    }
    if (model.credentials_env) {
      rcm += `${indent(1)}credentials = { env = "${model.credentials_env}" }\n`
    }
    rcm += `${indent(1)}limit = { context = "1000000", output = "393216" }\n`
    rcm += `${indent(1)}modalities = { input = ["text"], output = ["text"] }\n`
    rcm += `}\n\n`
  }

  const accelNodes = nodes.filter((n) => n.kind === 'accelerator')
  const fluxNodes = nodes.filter((n) => n.kind === 'flux')
  const condNodes = nodes.filter((n) => n.kind === 'condition')

  const hasGraph = fluxNodes.length > 0 || condNodes.length > 0 || wires.length > 0

  if (!hasGraph) {
    const accel = accelNodes[0]
    if (accel) {
      rcm += `accelerator {\n`
      rcm += writePrimitive(accel, 1)
      rcm += `}\n`
    }
    return rcm
  }

  rcm += `graph {\n`

  for (const accel of accelNodes) {
    rcm += `${indent(1)}accelerator ${accel.name} {\n`
    rcm += writePrimitive(accel, 2)
    rcm += `${indent(1)}}\n`
  }

  for (const flux of fluxNodes) {
    rcm += `${indent(1)}flux ${flux.name} {\n`
    if (flux.fluxChannel) rcm += `${indent(2)}channel = ${flux.fluxChannel}\n`
    if (flux.fluxMode) rcm += `${indent(2)}mode = ${flux.fluxMode}\n`
    if (flux.fluxArity) rcm += `${indent(2)}arity = ${flux.fluxArity}\n`
    rcm += `${indent(1)}}\n`
  }

  for (const cond of condNodes) {
    rcm += `${indent(1)}condition ${cond.name} {\n`
    rcm += `${indent(2)}// predicate\n`
    rcm += `${indent(1)}}\n`
  }

  for (const wire of wires) {
    rcm += `${indent(1)}${portString(wire.from)} -> ${portString(wire.to)}\n`
  }

  rcm += `}\n`
  return rcm
}

function writePrimitive(node: NodeData, level: number): string {
  let out = ''
  if (node.purpose) out += `${indent(level)}purpose = "${node.purpose}"\n`
  if (node.model) out += `${indent(level)}models = ["${node.model}"]\n`
  if (node.tools.length > 0) {
    out += `${indent(level)}tools = [${node.tools.map((t) => `"${t}"`).join(', ')}]\n`
  }
  if (node.mcps.length > 0) {
    out += `${indent(level)}mcps = [${node.mcps.map((m) => `"${m}"`).join(', ')}]\n`
  }
  if (node.policy) out += `${indent(level)}policy = "${node.policy}"\n`
  return out
}

function portString(port: { nodeId: string; name: string }): string {
  if (port.name === 'start') return 'input.trigger'
  if (port.name === 'output') return 'output.done'
  return `${port.nodeId}.${port.name}`
}
