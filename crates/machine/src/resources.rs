use std::collections::{HashMap, HashSet};
use std::fmt;

use serde::{Deserialize, Serialize};
use utils::{Name, ResourcesId};

use crate::model::Model;
use crate::tool::ToolDefinition;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resources {
    id: ResourcesId,
    pub name: Name,
    pub tools: HashMap<String, ToolDefinition>,
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

    pub fn with_tool_definition(mut self, definition: ToolDefinition) -> Self {
        self.tools.insert(definition.name.clone(), definition);
        self
    }

    pub fn with_model(mut self, model: Model) -> Self {
        if !self.models.contains_key(&model.name) {
            self.model_order.push(model.name.clone());
        }
        self.models.insert(model.name.clone(), model);
        self
    }

    pub fn replace_tool_definitions(mut self, tools: HashMap<String, ToolDefinition>) -> Self {
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

    pub fn enable(&mut self, name: impl Into<String>) -> Result<(), ToolNotRegistered> {
        let name = name.into();
        if self.tools.contains_key(&name) {
            self.active_tools.insert(name);
            Ok(())
        } else {
            Err(ToolNotRegistered(name))
        }
    }

    pub fn disable(&mut self, name: impl Into<String>) {
        self.active_tools.remove(&name.into());
    }

    pub fn use_model(&mut self, name: impl Into<String>) -> Result<(), ModelNotRegistered> {
        let name = name.into();
        if self.models.contains_key(&name) {
            self.active_model = name;
            Ok(())
        } else {
            Err(ModelNotRegistered(name))
        }
    }

    pub fn active_model(&self) -> Option<&Model> {
        self.models.get(&self.active_model)
    }

    pub fn active_tool_definitions(&self) -> Vec<&ToolDefinition> {
        self.active_tools
            .iter()
            .filter_map(|name| self.tools.get(name))
            .collect()
    }

    pub fn lookup(&self, name: &str) -> LookupResult {
        if !self.tools.contains_key(name) {
            LookupResult::NotFound
        } else if self.active_tools.contains(name) {
            LookupResult::Active
        } else {
            LookupResult::Inactive
        }
    }

    pub fn tool_definition(&self, name: &str) -> Option<&ToolDefinition> {
        self.tools.get(name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LookupResult {
    Active,
    Inactive,
    NotFound,
}

#[derive(Debug)]
pub struct ModelNotRegistered(pub String);

impl fmt::Display for ModelNotRegistered {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "model '{}' not registered", self.0)
    }
}

impl std::error::Error for ModelNotRegistered {}

#[derive(Debug)]
pub struct ToolNotRegistered(pub String);

impl fmt::Display for ToolNotRegistered {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "tool '{}' not registered", self.0)
    }
}

impl std::error::Error for ToolNotRegistered {}
