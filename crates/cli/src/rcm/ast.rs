use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
pub struct RcmFile {
    pub name: String,
    pub uses: Vec<UseDef>,
    pub models: Vec<ModelDef>,
    pub mcps: Vec<McpDef>,
    pub body: AcceleratorBodyDef,
}

#[derive(Debug, Clone, Serialize)]
pub struct UseDef {
    pub path: String,
    pub alias: String,
}

#[derive(Debug, Clone, Serialize)]
pub enum AcceleratorBodyDef {
    Primitive(PrimitiveDef),
    Graph(GraphDef),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum PromptSourceDef {
    Inline(String),
    File(String),
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PrimitiveDef {
    pub purpose: Option<String>,
    pub models: Vec<String>,
    pub prompts: Option<HashMap<String, PromptSourceDef>>,
    pub tools: Option<Vec<String>>,
    pub mcps: Option<Vec<String>>,
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GraphDef {
    pub accelerators: Vec<GraphAcceleratorDef>,
    pub maps: Vec<MapDef>,
    pub fluxes: Vec<FluxDef>,
    pub conditions: Vec<ConditionDef>,
    pub wires: Vec<WireDef>,
}

/// A `map` node: fans an imported accelerator out over a runtime-sized list.
/// Wires like an ordinary accelerator; the fan-out happens inside the engine.
#[derive(Debug, Clone, Serialize)]
pub struct MapDef {
    pub id: String,
    /// Alias of a `use`-imported accelerator to run per item.
    pub inner_alias: String,
    /// How to split the input into items (e.g. `json`).
    pub scatter: String,
    /// How to merge per-item outputs (e.g. `digest`).
    pub gather: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GraphAcceleratorDef {
    pub id: String,
    pub source: AcceleratorSourceDef,
}

#[derive(Debug, Clone, Serialize)]
pub enum AcceleratorSourceDef {
    Inline(PrimitiveDef),
    Import {
        alias: String,
        overrides: PrimitiveDef,
    },
}

#[derive(Debug, Clone, Serialize)]
pub struct ModelDef {
    pub id: String,
    pub protocol: String,
    pub endpoint: Option<String>,
    pub credentials_env: Option<String>,
    pub credentials_key: Option<String>,
    pub limit_context: Option<u64>,
    pub limit_input: Option<u64>,
    pub limit_output: u64,
    pub modalities_input: Vec<String>,
    pub modalities_output: Vec<String>,
    pub headers: HashMap<String, String>,
    pub thinking: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct FluxDef {
    pub id: String,
    pub name: Option<String>,
    pub channel: String,
    pub mode: String,
    pub arity: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConditionDef {
    pub id: String,
    pub name: Option<String>,
    pub predicate: Predicate,
}

#[derive(Debug, Clone, Serialize)]
pub struct WireDef {
    pub from: PortDef,
    pub to: PortDef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PortDef {
    pub owner: PortOwnerDef,
    pub endpoint: EndpointDef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum PortOwnerDef {
    Input,
    Output,
    Component(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum EndpointDef {
    Trigger,
    Done,
    State(String),
    FluxOut,
    FluxSlot(usize),
    ConditionTrue,
    ConditionFalse,
}

#[derive(Debug, Clone, Serialize)]
pub struct McpDef {
    pub label: String,
    pub transport: McpTransportDef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum McpTransportDef {
    Stdio {
        command: String,
        args: Vec<String>,
        env: HashMap<String, McpValueDef>,
        cwd: Option<String>,
    },
    Http {
        url: String,
        headers: HashMap<String, McpValueDef>,
    },
    Sse {
        url: String,
        headers: HashMap<String, McpValueDef>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum McpValueDef {
    Literal(String),
    Env(String),
}

#[derive(Debug, Clone, Serialize)]
pub enum Predicate {
    PurposeContains(String),
    PurposeEquals(String),
    PurposeStartsWith(String),
    PurposeEndsWith(String),
    PurposeIsEmpty,
    ContextHasTag(String),
    ContextHasRole(String),
    ContextContains(String),
    ContextIsEmpty,
    EnvVarExists(String),
    EnvVarEquals(String, String),
    EnvCwdContains(String),
    EnvPlatformIs(String),
    ResHasModel(String),
    ResActiveModelIs(String),
    ResHasTool(String),
    ResToolEnabled(String),
    ResHasPrompt(String),
    All(Vec<Predicate>),
    Any(Vec<Predicate>),
    Not(Box<Predicate>),
}
