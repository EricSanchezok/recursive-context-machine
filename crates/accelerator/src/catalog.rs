use std::collections::HashMap;
use std::sync::Arc;

use machine::{Environment, Policy, Resources, Tool};

/// A registry of named, built-in components.
///
/// Built at startup by calling each module's `register()` function.
/// The compiler resolves `.rcm` names against this table at compile time.
/// The gRPC server uses default_*() methods for fallback values.
#[derive(Default)]
pub struct Catalog {
    pub policies: HashMap<String, fn() -> Box<dyn Policy>>,
    pub tools: HashMap<String, Arc<dyn Tool>>,
    pub prompts: HashMap<String, String>,
    pub environments: HashMap<String, Environment>,
    pub resources: HashMap<String, Resources>,
}

impl Catalog {
    pub fn new() -> Self {
        let mut catalog = Self::default();

        crate::policy::register(&mut catalog);
        crate::tools::register(&mut catalog);
        crate::prompts::register(&mut catalog);
        crate::environment::register(&mut catalog);
        crate::resources::register(&mut catalog);

        catalog
    }

    pub fn default_environment(&self) -> Environment {
        self.environments
            .get("local")
            .cloned()
            .unwrap_or_else(|| Environment::new("."))
    }

    pub fn default_resources(&self) -> Resources {
        self.build_resources("kit")
            .unwrap_or_else(|_| Resources::new())
    }

    pub fn default_policy(&self) -> Box<dyn Policy> {
        self.policies
            .get("captain")
            .map(|factory| factory())
            .expect("captain policy must be registered")
    }

    /// Build a `Resources` from a named preset, injecting catalog tools and prompts.
    pub fn build_resources(&self, preset: &str) -> Result<Resources, String> {
        let mut res = self
            .resources
            .get(preset)
            .cloned()
            .ok_or_else(|| format!("unknown resource preset: {}", preset))?;

        for tool in self.tools.values() {
            res = res.with_tool(tool.clone());
        }

        for (name, content) in &self.prompts {
            res.prompts.entry(name.clone()).or_insert(content.clone());
        }

        Ok(res)
    }
}
