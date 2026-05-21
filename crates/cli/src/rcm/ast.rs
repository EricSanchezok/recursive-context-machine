use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RcmFile {
    pub name: String,
    pub uses: Vec<UseDef>,
    pub models: Vec<ModelDef>,
    pub mcps: Vec<McpDef>,
    pub body: AcceleratorBodyDef,
}

#[derive(Debug, Clone)]
pub struct UseDef {
    pub path: String,
    pub alias: String,
}

#[derive(Debug, Clone)]
pub enum AcceleratorBodyDef {
    Primitive(PrimitiveDef),
    Graph(GraphDef),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PromptSourceDef {
    Inline(String),
    File(String),
}

#[derive(Debug, Clone, Default)]
pub struct PrimitiveDef {
    pub purpose: Option<String>,
    pub models: Vec<String>,
    pub prompts: Option<HashMap<String, PromptSourceDef>>,
    pub tools: Option<Vec<String>>,
    pub mcps: Option<Vec<String>>,
    pub policy: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GraphDef {
    pub accelerators: Vec<GraphAcceleratorDef>,
    pub fluxes: Vec<FluxDef>,
    pub conditions: Vec<ConditionDef>,
    pub wires: Vec<WireDef>,
}

#[derive(Debug, Clone)]
pub struct GraphAcceleratorDef {
    pub id: String,
    pub source: AcceleratorSourceDef,
}

#[derive(Debug, Clone)]
pub enum AcceleratorSourceDef {
    Inline(PrimitiveDef),
    Import {
        alias: String,
        overrides: PrimitiveDef,
    },
}

#[derive(Debug, Clone)]
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
}

#[derive(Debug, Clone)]
pub struct FluxDef {
    pub id: String,
    pub name: Option<String>,
    pub channel: String,
    pub mode: String,
    pub arity: usize,
}

#[derive(Debug, Clone)]
pub struct ConditionDef {
    pub id: String,
    pub name: Option<String>,
    pub predicate: Predicate,
}

#[derive(Debug, Clone)]
pub struct WireDef {
    pub from: PortDef,
    pub to: PortDef,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortDef {
    pub owner: PortOwnerDef,
    pub endpoint: EndpointDef,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortOwnerDef {
    Input,
    Output,
    Component(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EndpointDef {
    Trigger,
    Done,
    State(String),
    FluxOut,
    FluxSlot(usize),
    ConditionTrue,
    ConditionFalse,
}

#[derive(Debug, Clone)]
pub struct McpDef {
    pub label: String,
    pub transport: McpTransportDef,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum McpValueDef {
    Literal(String),
    Env(String),
}

#[derive(Debug, Clone)]
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
