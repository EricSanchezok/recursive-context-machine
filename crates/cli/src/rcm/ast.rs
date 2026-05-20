/// A parsed `.rcm` file.
#[derive(Debug, Clone)]
pub struct RcmFile {
    pub name: String,
    pub models: Vec<ModelDef>,
    pub accelerators: Vec<AcceleratorDef>,
    pub fluxes: Vec<FluxDef>,
    pub conditions: Vec<ConditionDef>,
    pub wires: Vec<WireDef>,
    pub mcps: Vec<McpDef>,
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
pub struct AcceleratorDef {
    pub id: String,
    pub name: Option<String>,
    pub purpose: Option<String>,
    pub model: Option<String>,
    pub tools: Vec<String>,
    pub policy: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FluxDef {
    pub id: String,
    pub name: Option<String>,
    pub channel: String,
    pub mode: String,
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

#[derive(Debug, Clone)]
pub enum PortDef {
    Accelerator { id: String, port: String },
    Flux { id: String, port: String },
    Condition { id: String, port: String },
}

#[derive(Debug, Clone)]
pub struct McpDef {
    pub label: String,
    pub url: Option<String>,
    pub command: Option<String>,
    pub headers: Vec<(String, String)>,
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
