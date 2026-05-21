export interface Inventory {
  policies: string[]
  tools: { builtin: string[]; mcp_servers: McpServerSummary[] }
  prompts: { builtin: string[]; external: PromptSummary[] }
  models: { external: ModelSummary[] }
}

export interface McpServerSummary {
  label: string
  transport: { type: string; url?: string; command?: string; args?: string[] }
}

export interface PromptSummary {
  name: string
  path: string
  preview: string
}

export interface ModelSummary {
  name: string
  protocol: string
  endpoint?: string
}

export interface PortOutput {
  direction: 'in' | 'out'
  name: string
  channel: string
}

export interface RcmAst {
  name: string
  body: { Primitive: PrimitiveParse } | { Graph: GraphParse }
  uses: UseParse[]
  models: ModelParse[]
  mcps: McpParse[]
}

export interface UseParse {
  path: string
  alias: string
}

export interface PrimitiveParse {
  purpose?: string
  models: string[]
  prompts?: Record<string, { Inline: string } | { File: string }>
  tools?: string[]
  mcps?: string[]
  policy?: string
}

export interface GraphParse {
  accelerators: GraphAccelParse[]
  fluxes: FluxParse[]
  conditions: ConditionParse[]
  wires: WireParse[]
}

export interface GraphAccelParse {
  id: string
  source: { Inline: PrimitiveParse } | { Import: { alias: string; overrides: PrimitiveParse } }
}

export interface FluxParse {
  id: string
  name?: string
  channel: string
  mode: string
  arity: number
}

export interface ConditionParse {
  id: string
  name?: string
  predicate: PredicateParse
}

export interface WireParse {
  from: PortParse
  to: PortParse
}

export interface PortParse {
  owner: 'Input' | 'Output' | { Component: string }
  endpoint:
    | 'Trigger'
    | 'Done'
    | { State: string }
    | 'FluxOut'
    | { FluxSlot: number }
    | 'ConditionTrue'
    | 'ConditionFalse'
}

export interface ModelParse {
  id: string
  protocol: string
  endpoint?: string
  credentials_env?: string
  credentials_key?: string
  limit_context?: number
  limit_input?: number
  limit_output: number
  modalities_input: string[]
  modalities_output: string[]
}

export interface McpParse {
  label: string
  transport:
    | { Stdio: { command: string; args: string[]; env: Record<string, ValueParse>; cwd?: string } }
    | { Http: { url: string; headers: Record<string, ValueParse> } }
    | { Sse: { url: string; headers: Record<string, ValueParse> } }
}

export type ValueParse = { Literal: string } | { Env: string }

export type PredicateParse =
  | { PurposeContains: string }
  | { PurposeEquals: string }
  | { PurposeStartsWith: string }
  | { PurposeEndsWith: string }
  | 'PurposeIsEmpty'
  | { ContextHasTag: string }
  | { ContextHasRole: string }
  | { ContextContains: string }
  | 'ContextIsEmpty'
  | { EnvVarExists: string }
  | { EnvVarEquals: [string, string] }
  | { EnvCwdContains: string }
  | { EnvPlatformIs: string }
  | { ResHasModel: string }
  | { ResActiveModelIs: string }
  | { ResHasTool: string }
  | { ResToolEnabled: string }
  | { ResHasPrompt: string }
  | { All: PredicateParse[] }
  | { Any: PredicateParse[] }
  | { Not: PredicateParse }
