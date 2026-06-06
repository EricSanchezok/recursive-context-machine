use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;

use accelerator::mcp::{McpServerConfig, McpTransportConfig};
use accelerator::tools::SpawnTool;
use accelerator::{
    Accelerator, BridgeKind, Catalog, Channel, ComponentRef, ContextFlux, ContextPredicate,
    Endpoint, EnvFlux, EnvironmentPredicate, FluxMode, Graph, Port, Predicate as AccelPredicate,
    PurposeFlux, PurposePredicate, ResFlux, ResourceSelection, ResourcesPredicate,
};
use machine::{Limit, Modalities, Modality, Model, Policy, Protocol, Purpose, RunState};

use super::ast::{
    self, AcceleratorBodyDef, AcceleratorSourceDef, EndpointDef, McpTransportDef, McpValueDef,
    PortDef, PortOwnerDef, Predicate, PrimitiveDef, PromptSourceDef, RcmFile,
};

pub fn compile_file(
    path: impl AsRef<Path>,
) -> Pin<Box<dyn Future<Output = Result<Accelerator, String>> + Send>> {
    compile_file_with_catalog(path, Catalog::new())
}

pub fn compile_file_with_catalog(
    path: impl AsRef<Path>,
    catalog: Catalog,
) -> Pin<Box<dyn Future<Output = Result<Accelerator, String>> + Send>> {
    let path = path.as_ref().to_path_buf();
    Box::pin(async move {
        let mut compiler = Compiler::new(PathBuf::from("."), catalog);
        compiler.compile_path(&path).await
    })
}

struct Compiler {
    root: PathBuf,
    base_catalog: Catalog,
    cache: HashMap<PathBuf, Accelerator>,
    active: HashSet<PathBuf>,
}

impl Compiler {
    fn new(root: PathBuf, catalog: Catalog) -> Self {
        Self {
            root,
            base_catalog: catalog,
            cache: HashMap::new(),
            active: HashSet::new(),
        }
    }

    fn compile_path<'a>(
        &'a mut self,
        path: &'a Path,
    ) -> Pin<Box<dyn Future<Output = Result<Accelerator, String>> + Send + 'a>> {
        Box::pin(async move {
            let full_path = if path.is_absolute() {
                path.to_path_buf()
            } else {
                self.root.join(path)
            };
            let key = full_path
                .canonicalize()
                .map_err(|error| format!("failed to resolve {}: {error}", full_path.display()))?;
            if let Some(accelerator) = self.cache.get(&key) {
                return Ok(accelerator.clone());
            }
            if !self.active.insert(key.clone()) {
                return Err(format!("cyclic rcm import: {}", key.display()));
            }
            let source = std::fs::read_to_string(&key)
                .map_err(|error| format!("failed to read {}: {error}", key.display()))?;
            let file = crate::rcm::parse(&source)?;
            let previous_root = std::mem::replace(
                &mut self.root,
                key.parent().unwrap_or_else(|| Path::new(".")).to_path_buf(),
            );
            let compiled = self.compile_file_ast(&file).await;
            self.root = previous_root;
            self.active.remove(&key);
            let accelerator = compiled?;
            self.cache.insert(key, accelerator.clone());
            Ok(accelerator)
        })
    }

    async fn compile_file_ast(&mut self, file: &RcmFile) -> Result<Accelerator, String> {
        let mut catalog = self.base_catalog.clone();
        for model in build_models(&file.models)? {
            catalog.register_model(model)?;
        }
        for mcp_def in &file.mcps {
            catalog.register_mcp_server(mcp_config_from_def(mcp_def, &self.root)?)?;
        }

        let mut imports = HashMap::new();
        for use_def in &file.uses {
            let accelerator = self.compile_path(Path::new(&use_def.path)).await?;
            if imports.insert(use_def.alias.clone(), accelerator).is_some() {
                return Err(format!("duplicate import alias: {}", use_def.alias));
            }
        }

        match &file.body {
            AcceleratorBodyDef::Primitive(primitive) => {
                let (state, policy, tool_runtime) =
                    build_state(&catalog, primitive, &self.root).await?;
                Ok(Accelerator::primitive(
                    state,
                    policy,
                    tool_runtime,
                    file.name.as_str(),
                ))
            }
            AcceleratorBodyDef::Graph(graph_def) => {
                let graph = self
                    .compile_graph(file.name.as_str(), graph_def, &catalog, &imports)
                    .await?;
                Ok(Accelerator::composite_named(file.name.as_str(), graph))
            }
        }
    }

    async fn compile_graph(
        &mut self,
        name: &str,
        graph_def: &ast::GraphDef,
        catalog: &Catalog,
        imports: &HashMap<String, Accelerator>,
    ) -> Result<Graph, String> {
        let mut graph = Graph::named(name);
        let mut symbols = HashMap::new();
        let mut component_kinds = HashMap::new();

        for accelerator_def in &graph_def.accelerators {
            let accelerator = match &accelerator_def.source {
                AcceleratorSourceDef::Inline(primitive) => {
                    let (state, policy, tool_runtime) =
                        build_state(catalog, primitive, &self.root).await?;
                    let mut acc = Accelerator::primitive(
                        state,
                        policy,
                        tool_runtime,
                        accelerator_def.id.as_str(),
                    );
                    inject_spawns(&primitive.spawns, imports, &mut acc).await?;
                    acc
                }
                AcceleratorSourceDef::Import { alias, overrides } => {
                    let mut acc = imports
                        .get(alias)
                        .ok_or_else(|| format!("unknown accelerator import: {}", alias))?
                        .clone();
                    inject_spawns(&overrides.spawns, imports, &mut acc).await?;
                    acc
                }
            };
            let component = graph.add_accelerator(accelerator_def.id.as_str(), accelerator);
            insert_symbol(&mut symbols, accelerator_def.id.as_str(), component)?;
            component_kinds.insert(accelerator_def.id.clone(), ComponentTag::Accelerator);
        }

        for flux_def in &graph_def.fluxes {
            if flux_def.arity == 0 {
                return Err(format!(
                    "flux {} requires arity greater than zero",
                    flux_def.id
                ));
            }
            let mode = flux_mode_from_def(flux_def)?;
            let input_channel = mode.input_channel();
            let output_channel = mode.output_channel();
            let component = graph.add_flux(
                flux_def.name.as_deref().unwrap_or(flux_def.id.as_str()),
                mode,
                flux_def.arity,
            );
            insert_symbol(&mut symbols, flux_def.id.as_str(), component)?;
            component_kinds.insert(
                flux_def.id.clone(),
                ComponentTag::Flux {
                    input_channel,
                    output_channel,
                    arity: flux_def.arity,
                },
            );
        }

        for condition_def in &graph_def.conditions {
            let predicate = convert_predicate(&condition_def.predicate)?;
            let component = graph.add_condition(
                condition_def
                    .name
                    .as_deref()
                    .unwrap_or(condition_def.id.as_str()),
                predicate,
            );
            insert_symbol(&mut symbols, condition_def.id.as_str(), component)?;
            component_kinds.insert(condition_def.id.clone(), ComponentTag::Condition);
        }

        let mut flux_slots = flux_slot_map(graph_def);
        for wire in &graph_def.wires {
            let from = port_from_def(&wire.from, &symbols, &component_kinds)?;
            let to = port_from_def(&wire.to, &symbols, &component_kinds)?;
            validate_wire(&from, &to)?;
            mark_flux_slot(&wire.to, &mut flux_slots)?;
            graph.wire(from, to);
        }
        validate_flux_slots(&flux_slots)?;
        graph.validate()?;

        Ok(graph)
    }
}

#[derive(Clone, Copy)]
enum ComponentTag {
    Accelerator,
    Flux {
        input_channel: Channel,
        output_channel: Channel,
        arity: usize,
    },
    Condition,
}

fn insert_symbol(
    symbols: &mut HashMap<String, ComponentRef>,
    name: &str,
    component: ComponentRef,
) -> Result<(), String> {
    if symbols.insert(name.to_string(), component).is_some() {
        return Err(format!("duplicate graph component: {}", name));
    }
    Ok(())
}

fn mcp_config_from_def(def: &ast::McpDef, root: &Path) -> Result<McpServerConfig, String> {
    let transport = match &def.transport {
        McpTransportDef::Stdio {
            command,
            args,
            env,
            cwd,
        } => McpTransportConfig::Stdio {
            command: command.clone(),
            args: args.clone(),
            env: mcp_env_from_def(env)?,
            cwd: cwd.as_ref().map(|path| root.join(path)),
        },
        McpTransportDef::Http { url, headers } => McpTransportConfig::Http {
            url: url.clone(),
            headers: mcp_headers_from_def(headers)?,
        },
        McpTransportDef::Sse { url, headers } => McpTransportConfig::Sse {
            url: url.clone(),
            headers: mcp_headers_from_def(headers)?,
        },
    };
    Ok(McpServerConfig {
        label: def.label.clone(),
        transport,
    })
}

fn mcp_env_from_def(
    values: &HashMap<String, McpValueDef>,
) -> Result<HashMap<String, String>, String> {
    values
        .iter()
        .map(|(name, value)| Ok((name.clone(), mcp_value_text(value)?)))
        .collect()
}

fn mcp_headers_from_def(
    values: &HashMap<String, McpValueDef>,
) -> Result<Vec<(String, String)>, String> {
    values
        .iter()
        .map(|(name, value)| Ok((name.clone(), mcp_value_text(value)?)))
        .collect()
}

fn mcp_value_text(value: &McpValueDef) -> Result<String, String> {
    match value {
        McpValueDef::Literal(value) => expand_env_placeholders(value),
        McpValueDef::Env(name) => {
            std::env::var(name).map_err(|_| format!("environment variable '{}' is not set", name))
        }
    }
}

fn expand_env_placeholders(value: &str) -> Result<String, String> {
    let mut result = String::new();
    let mut rest = value;
    while let Some(start) = rest.find("${") {
        result.push_str(&rest[..start]);
        let after_start = &rest[start + 2..];
        let Some(end) = after_start.find('}') else {
            return Err(format!("unclosed environment placeholder in '{value}'"));
        };
        let name = &after_start[..end];
        let replacement = std::env::var(name)
            .map_err(|_| format!("environment variable '{}' is not set", name))?;
        result.push_str(&replacement);
        rest = &after_start[end + 1..];
    }
    result.push_str(rest);
    Ok(result)
}

fn build_models(defs: &[ast::ModelDef]) -> Result<Vec<Model>, String> {
    let mut model_names = HashSet::new();
    let mut models = Vec::new();
    for def in defs {
        let protocol = parse_protocol(&def.protocol)?;
        let modalities = build_modalities(&def.modalities_input, &def.modalities_output)?;
        let credentials = match (&def.credentials_env, &def.credentials_key) {
            (Some(env_var), None) => std::env::var(env_var).ok(),
            (None, Some(key)) => Some(key.clone()),
            (None, None) => None,
            (Some(_), Some(_)) => {
                return Err("credentials cannot have both 'env' and 'key'".to_string());
            }
        };
        let (input, context) = match (def.limit_context, def.limit_input) {
            (Some(ctx), None) => (ctx, ctx),
            (None, Some(inp)) => (inp, inp),
            (Some(ctx), Some(inp)) => (inp, ctx),
            (None, None) => {
                return Err("model limit requires at least one of context or input".to_string());
            }
        };
        let headers = if def.headers.is_empty() {
            None
        } else {
            Some(def.headers.clone())
        };
        let mut model = Model {
            name: def.id.clone(),
            protocol,
            endpoint: def.endpoint.clone(),
            credentials,
            limit: Some(Limit {
                context,
                input: Some(input),
                output: def.limit_output,
            }),
            cost: None,
            modalities: Some(modalities),
            headers,
            thinking: def.thinking,
            ..Default::default()
        };
        if let Some(timeout) = def.timeout {
            model.timeout = timeout;
        }
        if !model_names.insert(def.id.clone()) {
            return Err(format!("duplicate model: {}", def.id));
        }
        models.push(model);
    }
    Ok(models)
}

fn parse_protocol(name: &str) -> Result<Protocol, String> {
    match name {
        "openai" => Ok(Protocol::OpenAI),
        "anthropic" => Ok(Protocol::Anthropic),
        "gemini" => Ok(Protocol::Gemini),
        _ => Err(format!("unknown protocol: {}", name)),
    }
}

fn build_modalities(input: &[String], output: &[String]) -> Result<Modalities, String> {
    Ok(Modalities {
        input: parse_modalities(input)?,
        output: parse_modalities(output)?,
    })
}

fn parse_modalities(names: &[String]) -> Result<Vec<Modality>, String> {
    names
        .iter()
        .map(|name| match name.as_str() {
            "text" => Ok(Modality::Text),
            "audio" => Ok(Modality::Audio),
            "image" => Ok(Modality::Image),
            "video" => Ok(Modality::Video),
            "pdf" => Ok(Modality::Pdf),
            _ => Err(format!("unknown modality: {}", name)),
        })
        .collect()
}

async fn build_state(
    catalog: &Catalog,
    def: &PrimitiveDef,
    root: &Path,
) -> Result<(RunState, Box<dyn Policy>, machine::ToolRuntime), String> {
    if def.models.is_empty() {
        return Err("accelerator requires at least one model".to_string());
    }

    let policy_name = def.policy.as_deref().unwrap_or("captain");
    let policy = catalog.policy(policy_name)?;
    let runtime_resources = catalog
        .build_runtime_resources(ResourceSelection {
            models: def.models.clone(),
            tools: def.tools.clone().unwrap_or_default(),
            mcp_servers: def.mcps.clone().unwrap_or_default(),
            prompt_texts: match &def.prompts {
                Some(prompt_sources) => prompt_texts_from_sources(prompt_sources, root)?,
                None => HashMap::new(),
            },
        })
        .await?;
    let mut resources = runtime_resources.resources;
    resources.deactivate_model();
    resources.deactivate_tools();

    Ok((
        RunState {
            purpose: Purpose::new(def.purpose.clone().unwrap_or_default()),
            environment: catalog.environment(def.environment.as_deref().unwrap_or("local"))?,
            resources,
            ..RunState::default()
        },
        policy,
        runtime_resources.tool_runtime,
    ))
}

fn prompt_texts_from_sources(
    prompt_sources: &HashMap<String, PromptSourceDef>,
    root: &Path,
) -> Result<HashMap<String, String>, String> {
    let mut prompts = HashMap::new();
    for (name, source) in prompt_sources {
        let content = match source {
            PromptSourceDef::Inline(content) => content.clone(),
            PromptSourceDef::File(path) => {
                let prompt_path = root.join(path);
                std::fs::read_to_string(&prompt_path).map_err(|error| {
                    format!("failed to read prompt {}: {error}", prompt_path.display())
                })?
            }
        };
        prompts.insert(name.clone(), content);
    }
    Ok(prompts)
}

/// Inject `spawn_<alias>` tools into a planner accelerator for each name in
/// `spawn_names`. Each tool wraps the imported worker accelerator.
/// Aliases not found in `imports` produce a compile error.
async fn inject_spawns(
    spawn_names: &[String],
    imports: &HashMap<String, Accelerator>,
    planner: &mut Accelerator,
) -> Result<(), String> {
    for alias in spawn_names {
        let worker = imports
            .get(alias)
            .ok_or_else(|| format!("spawns references unknown accelerator import: {alias}"))?;
        let tool = Arc::new(SpawnTool::new(format!("spawn_{alias}"), worker.clone()));
        planner.inject_tool(tool);
    }
    Ok(())
}

fn flux_mode_from_def(def: &ast::FluxDef) -> Result<FluxMode, String> {
    match (def.channel.as_str(), def.mode.as_str()) {
        ("purpose", "concat") => Ok(FluxMode::Purpose(PurposeFlux::Concat)),
        ("context", "append") => Ok(FluxMode::Context(ContextFlux::Append)),
        ("context", "last") => Ok(FluxMode::Context(ContextFlux::Last)),
        ("context", "digest") => Ok(FluxMode::Context(ContextFlux::Digest)),
        ("context", "thread") => Ok(FluxMode::Context(ContextFlux::Thread)),
        ("environment", "overlay") => Ok(FluxMode::Environment(EnvFlux::Overlay)),
        ("resources", "merge") => Ok(FluxMode::Resources(ResFlux::Merge)),
        ("bridge", "flatten") => {
            let from_str = def
                .from
                .as_deref()
                .ok_or_else(|| "bridge flux requires 'from'".to_string())?;
            let to_str = def
                .to
                .as_deref()
                .ok_or_else(|| "bridge flux requires 'to'".to_string())?;
            let from = parse_channel(from_str)?;
            let to = parse_channel(to_str)?;
            match (from, to) {
                (Channel::Context, Channel::Purpose) => Ok(FluxMode::Bridge {
                    from,
                    to,
                    kind: BridgeKind::ContextToPurpose,
                }),
                _ => Err(format!(
                    "unsupported bridge direction: {} → {} (only context → purpose is implemented)",
                    from_str, to_str
                )),
            }
        }
        _ => Err(format!("unknown flux mode: {} {}", def.channel, def.mode)),
    }
}

fn port_from_def(
    def: &PortDef,
    symbols: &HashMap<String, ComponentRef>,
    kinds: &HashMap<String, ComponentTag>,
) -> Result<Port, String> {
    match &def.owner {
        PortOwnerDef::Input => Ok(boundary_port(true, &def.endpoint)?),
        PortOwnerDef::Output => Ok(boundary_port(false, &def.endpoint)?),
        PortOwnerDef::Component(name) => {
            let component = symbols
                .get(name)
                .ok_or_else(|| format!("unknown graph component: {}", name))?;
            let kind = kinds
                .get(name)
                .ok_or_else(|| format!("unknown graph component: {}", name))?;
            component_port(component, *kind, &def.endpoint)
        }
    }
}

fn boundary_port(is_input: bool, endpoint: &EndpointDef) -> Result<Port, String> {
    let endpoint = match endpoint {
        EndpointDef::Trigger => Endpoint::Trigger,
        EndpointDef::Done => Endpoint::Done,
        EndpointDef::State(channel) => Endpoint::State(parse_channel(channel)?),
        _ => {
            return Err(
                "boundary ports only support trigger, done, and state channels".to_string(),
            );
        }
    };
    if is_input {
        Ok(Port::input(endpoint))
    } else {
        Ok(Port::output(endpoint))
    }
}

fn component_port(
    component: &ComponentRef,
    kind: ComponentTag,
    endpoint: &EndpointDef,
) -> Result<Port, String> {
    match (kind, endpoint) {
        (ComponentTag::Accelerator, EndpointDef::Trigger) => Ok(component.trigger()),
        (ComponentTag::Accelerator, EndpointDef::Done) => Ok(component.done()),
        (ComponentTag::Accelerator, EndpointDef::State(channel)) => {
            Ok(component.port_state(parse_channel(channel)?))
        }
        (ComponentTag::Flux { output_channel, .. }, EndpointDef::FluxOut) => {
            Ok(component.flux_out(output_channel))
        }
        (
            ComponentTag::Flux {
                input_channel,
                arity,
                ..
            },
            EndpointDef::FluxSlot(slot),
        ) => {
            if *slot >= arity {
                return Err(format!("flux slot {} is out of range", slot));
            }
            Ok(component.slot(*slot, input_channel))
        }
        (ComponentTag::Condition, EndpointDef::Trigger) => Ok(component.condition_in()),
        (ComponentTag::Condition, EndpointDef::ConditionTrue) => {
            Ok(component.condition_out(accelerator::ConditionBranch::True))
        }
        (ComponentTag::Condition, EndpointDef::ConditionFalse) => {
            Ok(component.condition_out(accelerator::ConditionBranch::False))
        }
        _ => Err("endpoint does not match component type".to_string()),
    }
}

fn validate_wire(from: &Port, to: &Port) -> Result<(), String> {
    if !is_output_port(from) {
        return Err("wire source is not an output port".to_string());
    }
    if !is_input_port(to) {
        return Err("wire target is not an input port".to_string());
    }
    if from.channel() != to.channel() {
        return Err("wire channel mismatch".to_string());
    }
    Ok(())
}

fn is_output_port(port: &Port) -> bool {
    match (&port.owner, port.endpoint) {
        (accelerator::PortOwner::BoundaryInput, _) => true,
        (accelerator::PortOwner::BoundaryOutput, _) => false,
        (
            _,
            Endpoint::Done | Endpoint::State(_) | Endpoint::FluxOut(_) | Endpoint::ConditionOut(_),
        ) => true,
        _ => false,
    }
}

fn is_input_port(port: &Port) -> bool {
    match (&port.owner, port.endpoint) {
        (accelerator::PortOwner::BoundaryOutput, _) => true,
        (accelerator::PortOwner::BoundaryInput, _) => false,
        (
            _,
            Endpoint::Trigger
            | Endpoint::State(_)
            | Endpoint::FluxSlot { .. }
            | Endpoint::ConditionIn,
        ) => true,
        _ => false,
    }
}

fn flux_slot_map(graph: &ast::GraphDef) -> HashMap<String, Vec<bool>> {
    graph
        .fluxes
        .iter()
        .map(|flux| (flux.id.clone(), vec![false; flux.arity]))
        .collect()
}

fn mark_flux_slot(port: &PortDef, slots: &mut HashMap<String, Vec<bool>>) -> Result<(), String> {
    let PortOwnerDef::Component(component) = &port.owner else {
        return Ok(());
    };
    let EndpointDef::FluxSlot(slot) = port.endpoint else {
        return Ok(());
    };
    let Some(component_slots) = slots.get_mut(component) else {
        return Ok(());
    };
    if slot >= component_slots.len() {
        return Err(format!("flux slot {} is out of range", slot));
    }
    component_slots[slot] = true;
    Ok(())
}

fn validate_flux_slots(slots: &HashMap<String, Vec<bool>>) -> Result<(), String> {
    for (flux, filled) in slots {
        if let Some(slot) = filled.iter().position(|has_input| !has_input) {
            return Err(format!("flux {} missing input for slot {}", flux, slot));
        }
    }
    Ok(())
}

fn parse_channel(channel: &str) -> Result<Channel, String> {
    match channel {
        "purpose" => Ok(Channel::Purpose),
        "context" => Ok(Channel::Context),
        "environment" => Ok(Channel::Environment),
        "resources" => Ok(Channel::Resources),
        _ => Err(format!("unknown state channel: {}", channel)),
    }
}

fn convert_predicate(predicate: &Predicate) -> Result<AccelPredicate, String> {
    Ok(match predicate {
        Predicate::PurposeContains(value) => {
            AccelPredicate::Purpose(PurposePredicate::Contains(value.clone()))
        }
        Predicate::PurposeEquals(value) => {
            AccelPredicate::Purpose(PurposePredicate::Equals(value.clone()))
        }
        Predicate::PurposeStartsWith(value) => {
            AccelPredicate::Purpose(PurposePredicate::StartsWith(value.clone()))
        }
        Predicate::PurposeEndsWith(value) => {
            AccelPredicate::Purpose(PurposePredicate::EndsWith(value.clone()))
        }
        Predicate::PurposeIsEmpty => AccelPredicate::Purpose(PurposePredicate::IsEmpty),
        Predicate::ContextHasTag(value) => {
            AccelPredicate::Context(ContextPredicate::HasTag(value.clone()))
        }
        Predicate::ContextHasRole(value) => {
            AccelPredicate::Context(ContextPredicate::HasRole(parse_role(value)?))
        }
        Predicate::ContextContains(value) => {
            AccelPredicate::Context(ContextPredicate::Contains(value.clone()))
        }
        Predicate::ContextIsEmpty => AccelPredicate::Context(ContextPredicate::IsEmpty),
        Predicate::EnvVarExists(value) => {
            AccelPredicate::Environment(EnvironmentPredicate::VarExists(value.clone()))
        }
        Predicate::EnvVarEquals(key, value) => {
            AccelPredicate::Environment(EnvironmentPredicate::VarEquals(key.clone(), value.clone()))
        }
        Predicate::EnvCwdContains(value) => {
            AccelPredicate::Environment(EnvironmentPredicate::CwdContains(value.clone()))
        }
        Predicate::EnvPlatformIs(value) => {
            AccelPredicate::Environment(EnvironmentPredicate::PlatformIs(value.clone()))
        }
        Predicate::ResHasModel(value) => {
            AccelPredicate::Resources(ResourcesPredicate::HasModel(value.clone()))
        }
        Predicate::ResActiveModelIs(value) => {
            AccelPredicate::Resources(ResourcesPredicate::ActiveModelIs(value.clone()))
        }
        Predicate::ResHasTool(value) => {
            AccelPredicate::Resources(ResourcesPredicate::HasTool(value.clone()))
        }
        Predicate::ResToolEnabled(value) => {
            AccelPredicate::Resources(ResourcesPredicate::ToolEnabled(value.clone()))
        }
        Predicate::ResHasPrompt(value) => {
            AccelPredicate::Resources(ResourcesPredicate::HasPrompt(value.clone()))
        }
        Predicate::All(predicates) => AccelPredicate::All(
            predicates
                .iter()
                .map(convert_predicate)
                .collect::<Result<_, _>>()?,
        ),
        Predicate::Any(predicates) => AccelPredicate::Any(
            predicates
                .iter()
                .map(convert_predicate)
                .collect::<Result<_, _>>()?,
        ),
        Predicate::Not(predicate) => AccelPredicate::Not(Box::new(convert_predicate(predicate)?)),
    })
}

fn parse_role(role: &str) -> Result<machine::Role, String> {
    match role {
        "system" => Ok(machine::Role::System),
        "user" => Ok(machine::Role::User),
        "assistant" => Ok(machine::Role::Assistant),
        "tool" => Ok(machine::Role::Tool),
        _ => Err(format!("unknown role: {}", role)),
    }
}
