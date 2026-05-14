use crate::model::Model;
use crate::tool::Tool;

/// Resources — the pool of available tools and models with activation state.
///
/// The Policy sets the active model and catches/drops tools via [`Action`](crate::Action).
/// The Completion reads the active state directly from Resources.
pub struct Resources {
    pub tools: Vec<Box<dyn Tool>>,
    pub models: Vec<Model>,
    active_model: Option<String>,
    active_tools: Vec<String>,
}

impl Default for Resources {
    fn default() -> Self {
        Self::new()
    }
}

impl Resources {
    pub fn new() -> Self {
        Self {
            tools: Vec::new(),
            models: Vec::new(),
            active_model: None,
            active_tools: Vec::new(),
        }
    }

    pub fn with_tool(mut self, tool: Box<dyn Tool>) -> Self {
        self.tools.push(tool);
        self
    }

    pub fn with_model(mut self, model: Model) -> Self {
        self.models.push(model);
        self
    }

    // ── Mutation ──

    /// Set the active model by name. Only one model can be active.
    ///
    /// # Panics
    ///
    /// Panics if the model is not registered.
    pub fn set_active_model(&mut self, name: impl Into<String>) {
        let name = name.into();
        assert!(
            self.models.iter().any(|m| m.name == name),
            "model '{}' not registered",
            name
        );
        self.active_model = Some(name);
    }

    /// Activate a tool by adding it to the active set. Idempotent.
    ///
    /// # Panics
    ///
    /// Panics if the tool is not registered.
    pub fn catch_tool(&mut self, name: impl Into<String>) {
        let name = name.into();
        assert!(
            self.tools.iter().any(|t| t.name() == name),
            "tool '{}' not registered",
            name
        );
        if !self.active_tools.contains(&name) {
            self.active_tools.push(name);
        }
    }

    /// Deactivate a tool by removing it from the active set.
    pub fn drop_tool(&mut self, name: impl AsRef<str>) {
        self.active_tools.retain(|t| t != name.as_ref());
    }

    // ── Query ──

    /// The currently active model, if any.
    pub fn active_model(&self) -> Option<&Model> {
        self.active_model
            .as_deref()
            .and_then(|name| self.models.iter().find(|m| m.name == name))
    }

    /// The currently active tools.
    pub fn active_tools(&self) -> Vec<&dyn Tool> {
        self.active_tools
            .iter()
            .filter_map(|name| {
                self.tools
                    .iter()
                    .find(|t| t.name() == name)
                    .map(|t| t.as_ref())
            })
            .collect()
    }
}
