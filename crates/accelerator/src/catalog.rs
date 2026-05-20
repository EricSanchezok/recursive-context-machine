use std::collections::HashMap;
use std::sync::Arc;

use machine::{Environment, Model, Policy, Resources, Tool};

/// A registry of named, built-in components.
#[derive(Default)]
pub struct Catalog {
    pub models: HashMap<String, Model>,
    pub policies: HashMap<String, fn() -> Box<dyn Policy>>,
    pub tools: HashMap<String, Arc<dyn Tool>>,
    pub prompts: HashMap<String, String>,
    pub environments: HashMap<String, Environment>,
    pub resources: HashMap<String, Resources>,
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

    /// Build a `Resources` from a named preset, injecting catalog tools, models, and prompts.
    pub fn build_resources(&self, preset: &str) -> Result<Resources, String> {
        let mut res = self
            .resources
            .get(preset)
            .cloned()
            .ok_or_else(|| format!("unknown resource preset: {}", preset))?;

        for tool in self.tools.values() {
            let name = tool.name().to_string();
            res = res.with_tool(tool.clone());
        }

        for (name, model) in &self.models {
            res = res.with_model(model.clone());
        }

        for (name, content) in &self.prompts {
            res.prompts.entry(name.clone()).or_insert(content.clone());
        }

        Ok(res)
    }
}
