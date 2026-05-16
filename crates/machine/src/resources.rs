use std::collections::{HashMap, HashSet};

use crate::model::Model;
use crate::tool::Tool;

/// Resources — the pool of available tools and models with activation state.
///
/// The Policy switches models and toggles tools via [`Action`](crate::Action).
/// The completion reads the active state directly.
pub struct Resources {
    pub tools: HashMap<String, Box<dyn Tool>>,
    pub models: HashMap<String, Model>,
    pub active_model: String,
    pub active_tools: HashSet<String>,
    pub prompts: HashMap<String, String>,
}

impl Default for Resources {
    fn default() -> Self {
        Self::new()
    }
}

impl Resources {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
            models: HashMap::new(),
            active_model: String::new(),
            active_tools: HashSet::new(),
            prompts: HashMap::new(),
        }
    }

    /// Register a tool. Overwrites any tool with the same name.
    pub fn with_tool(mut self, tool: Box<dyn Tool>) -> Self {
        let name = tool.name().to_string();
        self.tools.insert(name, tool);
        self
    }

    /// Register a model. The first model registered becomes the active model.
    /// Overwrites any model with the same name.
    pub fn with_model(mut self, model: Model) -> Self {
        if self.active_model.is_empty() {
            self.active_model.clone_from(&model.name);
        }
        self.models.insert(model.name.clone(), model);
        self
    }

    /// Enable a tool. Idempotent.
    ///
    /// # Panics
    ///
    /// Panics if the tool is not registered.
    pub fn enable(&mut self, name: impl Into<String>) {
        let name = name.into();
        assert!(
            self.tools.contains_key(&name),
            "tool '{name}' not registered"
        );
        self.active_tools.insert(name);
    }

    /// Disable a tool.
    pub fn disable(&mut self, name: impl AsRef<str>) {
        self.active_tools.remove(name.as_ref());
    }

    /// Switch the active model.
    ///
    /// # Panics
    ///
    /// Panics if the model is not registered.
    pub fn use_model(&mut self, name: impl Into<String>) {
        let name = name.into();
        assert!(
            self.models.contains_key(&name),
            "model '{name}' not registered"
        );
        self.active_model = name;
    }

    /// The currently active model.
    ///
    /// # Panics
    ///
    /// Panics when no model has been registered.
    pub fn active_model(&self) -> &Model {
        self.models
            .get(&self.active_model)
            .expect("active model not found")
    }

    /// All active tools.
    pub fn active_tools(&self) -> Vec<&dyn Tool> {
        self.active_tools
            .iter()
            .filter_map(|name| self.tools.get(name))
            .map(|t| t.as_ref())
            .collect()
    }

    /// Look up an active tool by name.
    pub fn lookup(&self, name: &str) -> Option<&dyn Tool> {
        if !self.active_tools.contains(name) {
            return None;
        }
        self.tools.get(name).map(|t| t.as_ref())
    }
}
