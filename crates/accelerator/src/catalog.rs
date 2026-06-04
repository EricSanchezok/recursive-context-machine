use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use machine::{Environment, Model, Policy, Resources, Tool, ToolDefinition, ToolRuntime};
use tokio::sync::OnceCell;

use crate::mcp::McpServerConfig;

pub type PolicyFactory = Arc<dyn Fn() -> Box<dyn Policy> + Send + Sync>;
pub type EnvironmentFactory = Arc<dyn Fn() -> Environment + Send + Sync>;

#[derive(Clone, Default)]
pub struct Catalog {
    policies: HashMap<String, PolicyFactory>,
    environments: HashMap<String, EnvironmentFactory>,
    resources: ResourceCatalog,
}

#[derive(Clone, Default)]
struct ResourceCatalog {
    tools: HashMap<String, Arc<dyn Tool>>,
    models: HashMap<String, Model>,
    prompts: HashMap<String, String>,
    mcp_servers: HashMap<String, McpServerEntry>,
}

#[derive(Clone)]
struct McpServerEntry {
    config: McpServerConfig,
    tools: Arc<OnceCell<Vec<Arc<dyn Tool>>>>,
}

#[derive(Clone, Default)]
pub struct ResourceSelection {
    pub models: Vec<String>,
    pub tools: Vec<String>,
    pub mcp_servers: Vec<String>,
    pub prompt_texts: HashMap<String, String>,
}

pub struct RuntimeResources {
    pub resources: Resources,
    pub tool_runtime: ToolRuntime,
}

impl Catalog {
    pub fn new() -> Self {
        let mut catalog = Self::empty();

        crate::policy::register(&mut catalog);
        crate::tools::register(&mut catalog);
        crate::prompts::register(&mut catalog);
        crate::environment::register(&mut catalog);

        catalog
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn register_policy(
        &mut self,
        name: impl Into<String>,
        factory: impl Fn() -> Box<dyn Policy> + Send + Sync + 'static,
    ) -> Result<(), String> {
        let name = name.into();
        ensure_name_is_usable(&name, "policy")?;
        ensure_name_is_free(&self.policies, &name, "policy")?;
        self.policies.insert(name, Arc::new(factory));
        Ok(())
    }

    pub fn register_environment(
        &mut self,
        name: impl Into<String>,
        factory: impl Fn() -> Environment + Send + Sync + 'static,
    ) -> Result<(), String> {
        let name = name.into();
        ensure_name_is_usable(&name, "environment")?;
        ensure_name_is_free(&self.environments, &name, "environment")?;
        self.environments.insert(name, Arc::new(factory));
        Ok(())
    }

    pub fn register_tool(&mut self, tool: Arc<dyn Tool>) -> Result<(), String> {
        let name = tool.name().to_string();
        ensure_name_is_usable(&name, "tool")?;
        ensure_name_is_free(&self.resources.tools, &name, "tool")?;
        self.resources.tools.insert(name, tool);
        Ok(())
    }

    pub fn register_model(&mut self, model: Model) -> Result<(), String> {
        ensure_name_is_usable(&model.name, "model")?;
        ensure_name_is_free(&self.resources.models, &model.name, "model")?;
        self.resources.models.insert(model.name.clone(), model);
        Ok(())
    }

    pub fn register_prompt(
        &mut self,
        name: impl Into<String>,
        content: impl Into<String>,
    ) -> Result<(), String> {
        let name = name.into();
        ensure_name_is_usable(&name, "prompt")?;
        ensure_name_is_free(&self.resources.prompts, &name, "prompt")?;
        self.resources.prompts.insert(name, content.into());
        Ok(())
    }

    pub fn register_mcp_server(&mut self, config: McpServerConfig) -> Result<(), String> {
        ensure_name_is_usable(&config.label, "mcp server")?;
        ensure_name_is_free(&self.resources.mcp_servers, &config.label, "mcp server")?;
        self.resources.mcp_servers.insert(
            config.label.clone(),
            McpServerEntry {
                config,
                tools: Arc::new(OnceCell::new()),
            },
        );
        Ok(())
    }

    pub fn policy(&self, name: &str) -> Result<Box<dyn Policy>, String> {
        self.policies
            .get(name)
            .map(|factory| factory())
            .ok_or_else(|| format!("unknown policy: {name}"))
    }

    pub fn environment(&self, name: &str) -> Result<Environment, String> {
        self.environments
            .get(name)
            .map(|factory| factory())
            .ok_or_else(|| format!("unknown environment: {name}"))
    }

    pub async fn build_runtime_resources(
        &self,
        selection: ResourceSelection,
    ) -> Result<RuntimeResources, String> {
        let mut resources = Resources::new();
        let mut tool_runtime = ToolRuntime::new();
        let mut selected_tool_names = HashSet::new();

        for (name, content) in selection.prompt_texts {
            ensure_name_is_usable(&name, "prompt")?;
            resources.prompts.insert(name, content);
        }

        for model_name in selection.models {
            let model = self
                .resources
                .models
                .get(&model_name)
                .cloned()
                .ok_or_else(|| format!("unknown model: {model_name}"))?;
            resources = resources.with_model(model);
        }

        for tool_name in selection.tools {
            let tool = self
                .resources
                .tools
                .get(&tool_name)
                .cloned()
                .ok_or_else(|| format!("unknown tool: {tool_name}"))?;
            remember_selected_tool(&mut selected_tool_names, tool.name())?;
            resources = resources.with_tool_definition(ToolDefinition::from_tool(tool.as_ref()));
            tool_runtime.insert(tool);
        }

        for server_name in selection.mcp_servers {
            let entry = self
                .resources
                .mcp_servers
                .get(&server_name)
                .ok_or_else(|| format!("unknown mcp server: {server_name}"))?;
            let tools = entry
                .tools
                .get_or_try_init(|| async {
                    let registry =
                        crate::mcp::McpRegistry::start(std::slice::from_ref(&entry.config)).await?;
                    Ok::<_, String>(registry.tools_for(&server_name).unwrap_or_default())
                })
                .await?;
            for tool in tools {
                ensure_name_is_usable(tool.name(), "mcp tool")?;
                remember_selected_tool(&mut selected_tool_names, tool.name())?;
                resources =
                    resources.with_tool_definition(ToolDefinition::from_tool(tool.as_ref()));
                tool_runtime.insert(tool.clone());
            }
        }

        Ok(RuntimeResources {
            resources,
            tool_runtime,
        })
    }

    pub fn policy_names(&self) -> Vec<String> {
        sorted_names(self.policies.keys())
    }

    pub fn tool_names(&self) -> Vec<String> {
        sorted_names(self.resources.tools.keys())
    }

    pub fn prompt_names(&self) -> Vec<String> {
        sorted_names(self.resources.prompts.keys())
    }

    pub fn model_names(&self) -> Vec<String> {
        sorted_names(self.resources.models.keys())
    }

    pub fn environment_names(&self) -> Vec<String> {
        sorted_names(self.environments.keys())
    }

    pub fn mcp_server_names(&self) -> Vec<String> {
        sorted_names(self.resources.mcp_servers.keys())
    }
}

fn sorted_names<'a>(names: impl Iterator<Item = &'a String>) -> Vec<String> {
    let mut names: Vec<String> = names.cloned().collect();
    names.sort();
    names
}

fn remember_selected_tool(
    selected_tool_names: &mut HashSet<String>,
    name: &str,
) -> Result<(), String> {
    if selected_tool_names.insert(name.to_string()) {
        Ok(())
    } else {
        Err(format!("duplicate selected tool: {name}"))
    }
}

fn ensure_name_is_usable(name: &str, kind: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err(format!("{kind} name must not be empty"));
    }
    if name.len() > 64 {
        return Err(format!("{kind} name '{name}' is too long"));
    }
    if !name
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || character == '_' || character == '-')
    {
        return Err(format!(
            "{kind} name '{name}' may only contain ASCII letters, numbers, '_' and '-'"
        ));
    }
    Ok(())
}

fn ensure_name_is_free<Value>(
    entries: &HashMap<String, Value>,
    name: &str,
    kind: &str,
) -> Result<(), String> {
    if entries.contains_key(name) {
        return Err(format!("duplicate {kind}: {name}"));
    }
    Ok(())
}
