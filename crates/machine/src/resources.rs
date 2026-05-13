use crate::model::Model;
use crate::tool::Tool;

/// Resources — the pool of available tools and models.
///
/// The Policy selects from this pool by name. Tools are referenced by
/// [`Tool::name`]; models are referenced by [`Model::name`].
pub struct Resources {
    pub tools: Vec<Box<dyn Tool>>,
    pub models: Vec<Model>,
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
}
