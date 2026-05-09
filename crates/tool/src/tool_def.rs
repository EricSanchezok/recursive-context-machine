/// A tool definition — metadata about a tool, not the tool itself.
///
/// The actual tool implementation lives elsewhere (e.g. in a rig Tool, a WASM
/// module, or an external service). ToolDef is the descriptor that cells use
/// to discover and link to tools.
#[derive(Debug, Clone)]
pub struct ToolDef {
    pub name: String,
    pub description: String,
    /// JSON Schema for the tool's input parameters
    pub input_schema: serde_json::Value,
}

impl ToolDef {
    pub fn new(name: impl Into<String>, description: impl Into<String>, input_schema: serde_json::Value) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            input_schema,
        }
    }
}