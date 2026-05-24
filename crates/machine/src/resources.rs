use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::Arc;

use utils::{Name, ResourcesId};

use crate::model::Model;
use crate::tool::Tool;

/// Resources — the pool of available tools and models with activation state.
///
/// The Policy switches models and toggles tools via [`Action`](crate::Action).
/// The completion reads the active state directly.
#[derive(Clone)]
pub struct Resources {
    id: ResourcesId,
    pub name: Name,
    pub tools: HashMap<String, Arc<dyn Tool>>,
    pub models: HashMap<String, Model>,
    pub model_order: Vec<String>,
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
    pub fn id(&self) -> &ResourcesId {
        &self.id
    }

    pub fn new() -> Self {
        Self::named("resources")
    }

    pub fn named(name: impl Into<String>) -> Self {
        Self {
            id: ResourcesId::new(),
            name: Name::new(name).expect("resources name must be valid"),
            tools: HashMap::new(),
            models: HashMap::new(),
            model_order: Vec::new(),
            active_model: String::new(),
            active_tools: HashSet::new(),
            prompts: HashMap::new(),
        }
    }

    /// Register a tool. Overwrites any tool with the same name.
    pub fn with_tool(mut self, tool: Arc<dyn Tool>) -> Self {
        let name = tool.name().to_string();
        self.tools.insert(name, tool);
        self
    }

    /// Register a model. Overwrites any model with the same name.
    pub fn with_model(mut self, model: Model) -> Self {
        if !self.models.contains_key(&model.name) {
            self.model_order.push(model.name.clone());
        }
        self.models.insert(model.name.clone(), model);
        self
    }

    pub fn replace_tools(mut self, tools: HashMap<String, Arc<dyn Tool>>) -> Self {
        self.active_tools.retain(|name| tools.contains_key(name));
        self.tools = tools;
        self
    }

    pub fn replace_prompts(mut self, prompts: HashMap<String, String>) -> Self {
        self.prompts = prompts;
        self
    }

    pub fn deactivate_model(&mut self) {
        self.active_model.clear();
    }

    pub fn deactivate_tools(&mut self) {
        self.active_tools.clear();
    }

    /// Enable a tool. Idempotent.
    pub fn enable(&mut self, name: impl Into<String>) -> Result<(), ToolNotRegistered> {
        let name = name.into();
        if self.tools.contains_key(&name) {
            self.active_tools.insert(name);
            Ok(())
        } else {
            Err(ToolNotRegistered(name))
        }
    }

    /// Disable a tool.
    pub fn disable(&mut self, name: impl Into<String>) {
        self.active_tools.remove(&name.into());
    }

    /// Switch the active model.
    pub fn use_model(&mut self, name: impl Into<String>) -> Result<(), ModelNotRegistered> {
        let name = name.into();
        if self.models.contains_key(&name) {
            self.active_model = name;
            Ok(())
        } else {
            Err(ModelNotRegistered(name))
        }
    }

    /// The currently active model, if any.
    pub fn active_model(&self) -> Option<&Model> {
        self.models.get(&self.active_model)
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

/// Error returned when switching to a model that has not been registered.
#[derive(Debug)]
pub struct ModelNotRegistered(pub String);

impl fmt::Display for ModelNotRegistered {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "model '{}' not registered", self.0)
    }
}

impl std::error::Error for ModelNotRegistered {}

/// Error returned when enabling a tool that has not been registered.
#[derive(Debug)]
pub struct ToolNotRegistered(pub String);

impl fmt::Display for ToolNotRegistered {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "tool '{}' not registered", self.0)
    }
}

impl std::error::Error for ToolNotRegistered {}
