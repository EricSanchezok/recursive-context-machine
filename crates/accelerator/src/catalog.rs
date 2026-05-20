use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use machine::{Environment, Model, Policy, Resources, Tool};

/// A registry of named, built-in components.
///
/// Built at startup by calling each module's `register()` function.
/// The compiler resolves `.rcm` names against this table at compile time.
/// MCP servers are attached later (async) and merge into the same catalog.
#[derive(Default)]
pub struct Catalog {
    pub models: HashMap<String, Model>,
    pub policies: HashMap<String, fn() -> Box<dyn Policy>>,
    pub prompts: HashMap<String, String>,
    pub environments: HashMap<String, Environment>,
    pub resources: HashMap<String, Resources>,
    tool_queue: Vec<(String, Arc<dyn Tool>)>,
    enabled_tools: HashSet<String>,
}

impl Catalog {
    pub fn new() -> Self {
        let mut catalog = Self::default();

        crate::model::register(&mut catalog);
        crate::policy::register(&mut catalog);
        crate::tools::register(&mut catalog);
        crate::prompts::register(&mut catalog);
        crate::environment::register(&mut catalog);
        crate::resources::register(&mut catalog);

        catalog
    }

    pub fn register_tool(&mut self, name: impl Into<String>, tool: Arc<dyn Tool>) {
        self.tool_queue.push((name.into(), tool));
    }

    /// Finalise all queued tools into a `Resources` using the named preset as base.
    ///
    /// The preset's own tools (e.g. `kit`) are included first, then MCP or
    /// other queued tools are appended on top.
    pub fn build_resources(&self, preset: &str) -> Result<Resources, String> {
        let mut res = self
            .resources
            .get(preset)
            .cloned()
            .ok_or_else(|| format!("unknown resource preset: {}", preset))?;

        // Inject catalog-registered prompts into this resources instance.
        for (name, content) in &self.prompts {
            res.prompts.entry(name.clone()).or_insert(content.clone());
        }

        for (name, tool) in &self.tool_queue {
            res = res.with_tool(tool.clone());
            res.enable(name);
        }

        Ok(res)
    }

    pub fn tool_enabled(&self, name: &str) -> bool {
        self.enabled_tools.contains(name)
    }
}
