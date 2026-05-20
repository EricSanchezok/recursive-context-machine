/// A parsed `.rcm` file.
#[derive(Debug, Clone)]
pub struct RcmFile {
    pub name: String,
    pub agents: Vec<AgentDef>,
    pub fluxes: Vec<FluxDef>,
    pub conditions: Vec<ConditionDef>,
    pub wires: Vec<WireDef>,
    pub mcps: Vec<McpDef>,
}

#[derive(Debug, Clone)]
pub(crate) struct AgentDef {
    pub id: String,
    pub name: Option<String>,
    pub purpose: Option<String>,
    pub model: Option<String>,
    pub tools: Vec<String>,
    pub policy: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct FluxDef {
    pub id: String,
    pub name: Option<String>,
    pub channel: String,
    pub mode: String,
}

#[derive(Debug, Clone)]
pub(crate) struct ConditionDef {
    pub id: String,
    pub name: Option<String>,
    pub predicate: Predicate,
}

#[derive(Debug, Clone)]
pub(crate) struct WireDef {
    pub from: PortDef,
    pub to: PortDef,
}

#[derive(Debug, Clone)]
pub(crate) enum PortDef {
    Agent { id: String, port: String },
    Flux { id: String, port: String },
    Condition { id: String, port: String },
}

#[derive(Debug, Clone)]
pub(crate) struct McpDef {
    pub label: String,
    pub url: Option<String>,
    pub command: Option<String>,
    pub headers: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub(crate) enum Predicate {
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
