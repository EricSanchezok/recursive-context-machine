use std::collections::HashMap;

use accelerator::Catalog;
use accelerator::{
    ContextFlux, ContextPredicate, EnvFlux, EnvironmentPredicate, FluxMode, Graph, PolicyFlux,
    Predicate as AccelPredicate, PurposeFlux, PurposePredicate, ResFlux, ResourcesPredicate, State,
};
use machine::{Limit, Modalities, Modality, Model, Protocol};

use super::ast::{self, PortDef, Predicate, RcmFile};

/// Compile a parsed `.rcm` file into a `Graph`.
pub fn compile(file: &RcmFile) -> Result<Graph, String> {
    let catalog = Catalog::new();

    // Build model registry from .rcm model declarations.
    let models = build_models(&file.models)?;

    let mut graph = Graph::named(file.name.as_str());
    let mut agent_map: HashMap<String, _> = HashMap::new();
    let mut flux_map: HashMap<String, _> = HashMap::new();
    let mut condition_map: HashMap<String, _> = HashMap::new();

    for agent_def in &file.agents {
        let state = build_state(&catalog, &models, agent_def)?;
        let ref_ = graph.spawn_named(
            agent_def.name.as_deref().unwrap_or(agent_def.id.as_str()),
            state,
        );
        agent_map.insert(agent_def.id.clone(), ref_);
    }

    for flux_def in &file.fluxes {
        let mode = resolve_flux_mode(&catalog, flux_def)?;
        let ref_ = graph.weave_named(
            flux_def.name.as_deref().unwrap_or(flux_def.id.as_str()),
            2,
            mode,
        );
        flux_map.insert(flux_def.id.clone(), ref_);
    }

    for cond_def in &file.conditions {
        let predicate = convert_predicate(&cond_def.predicate)?;
        let ref_ = graph.condition_named(
            cond_def.name.as_deref().unwrap_or(cond_def.id.as_str()),
            predicate,
        );
        condition_map.insert(cond_def.id.clone(), ref_);
    }

    for wire in &file.wires {
        let from = resolve_port(&wire.from, &agent_map, &flux_map, &condition_map)?;
        let to = resolve_port(&wire.to, &agent_map, &flux_map, &condition_map)?;
        graph.wire(from, to);
    }

    Ok(graph)
}

fn build_models(defs: &[ast::ModelDef]) -> Result<HashMap<String, Model>, String> {
    let mut models = HashMap::new();
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
                return Err("model limit requires at least one of 'context' or 'input'".to_string());
            }
        };

        let model = Model {
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
            ..Default::default()
        };

        if models.contains_key(&def.id) {
            return Err(format!("duplicate model: {}", def.id));
        }
        models.insert(def.id.clone(), model);
    }
    Ok(models)
}

fn parse_protocol(name: &str) -> Result<Protocol, String> {
    match name {
        "openai" => Ok(Protocol::OpenAI),
        "anthropic" => Ok(Protocol::Anthropic),
        "gemini" => Ok(Protocol::Gemini),
        _ => Err(format!(
            "unknown protocol '{}' (expected openai, anthropic, or gemini)",
            name
        )),
    }
}

fn build_modalities(input: &[String], output: &[String]) -> Result<Modalities, String> {
    let parse_list = |names: &[String]| -> Result<Vec<Modality>, String> {
        names
            .iter()
            .map(|n| match n.as_str() {
                "text" => Ok(Modality::Text),
                "audio" => Ok(Modality::Audio),
                "image" => Ok(Modality::Image),
                "video" => Ok(Modality::Video),
                "pdf" => Ok(Modality::Pdf),
                _ => Err(format!("unknown modality '{}'", n)),
            })
            .collect()
    };
    Ok(Modalities {
        input: parse_list(input)?,
        output: parse_list(output)?,
    })
}

fn build_state(
    catalog: &Catalog,
    models: &HashMap<String, Model>,
    def: &ast::AgentDef,
) -> Result<State, String> {
    let model_name = def
        .model
        .as_deref()
        .ok_or_else(|| "agent requires a model (e.g. model = \"gpt-4.1\")".to_string())?;
    let model = models.get(model_name).ok_or_else(|| {
        format!(
            "unknown model '{}' (declare it with a 'model' block)",
            model_name
        )
    })?;

    let policy_name = def.policy.as_deref().unwrap_or("captain");
    let policy = catalog
        .policies
        .get(policy_name)
        .ok_or_else(|| format!("unknown policy: {}", policy_name))?;

    let mut resources = catalog.build_resources("kit")?;
    resources = resources.with_model(model.clone());
    resources.use_model(model_name);

    for tool_name in &def.tools {
        resources.enable(tool_name);
    }

    Ok(State {
        purpose: def.purpose.clone().unwrap_or_default(),
        policy: policy(),
        res: resources,
        ..Default::default()
    })
}

fn resolve_flux_mode(_catalog: &Catalog, def: &ast::FluxDef) -> Result<FluxMode, String> {
    match (def.channel.as_str(), def.mode.as_str()) {
        ("purpose", "concat") => Ok(FluxMode::Purpose(PurposeFlux::Concat)),
        ("context", "append") => Ok(FluxMode::Context(ContextFlux::Append)),
        ("context", "replace") => Ok(FluxMode::Context(ContextFlux::Replace)),
        ("environment", "overlay") => Ok(FluxMode::Environment(EnvFlux::Overlay)),
        ("resources", "merge") => Ok(FluxMode::Resources(ResFlux::Merge)),
        ("policy", "replace") => Ok(FluxMode::Policy(PolicyFlux::Replace)),
        _ => Err(format!("unknown flux mode: {} {}", def.channel, def.mode)),
    }
}

fn resolve_port(
    def: &PortDef,
    agents: &HashMap<String, accelerator::AcceleratorRef>,
    fluxes: &HashMap<String, accelerator::FluxRef>,
    conditions: &HashMap<String, accelerator::ConditionRef>,
) -> Result<accelerator::Port, String> {
    match def {
        PortDef::Agent { id, port } => {
            let agent = agents
                .get(id)
                .ok_or_else(|| format!("unknown agent: {}", id))?;
            match port.as_str() {
                "pulse" => Ok(agent.done()),
                "purpose" => Ok(agent.purpose_out()),
                "context" => Ok(agent.ctx_out()),
                "environment" => Ok(agent.env_out()),
                "policy" => Ok(agent.policy_out()),
                "resources" => Ok(agent.res_out()),
                _ => Err(format!("unknown agent port: {}", port)),
            }
        }
        PortDef::Flux { id, port } => {
            let flux = fluxes
                .get(id)
                .ok_or_else(|| format!("unknown flux: {}", id))?;
            match port.as_str() {
                "out" => Ok(flux.out()),
                _ if port.starts_with("slot(") && port.ends_with(')') => {
                    let slot: usize = port[5..port.len() - 1]
                        .parse()
                        .map_err(|_| format!("invalid slot number: {}", port))?;
                    Ok(flux.slot(slot))
                }
                _ => Err(format!("unknown flux port: {}", port)),
            }
        }
        PortDef::Condition { id, port } => {
            let condition = conditions
                .get(id)
                .ok_or_else(|| format!("unknown condition: {}", id))?;
            match port.as_str() {
                "trigger" => Ok(condition.trigger()),
                "true" => Ok(condition.pulse_true()),
                "false" => Ok(condition.pulse_false()),
                _ => Err(format!("unknown condition port: {}", port)),
            }
        }
    }
}

fn convert_predicate(predicate: &Predicate) -> Result<AccelPredicate, String> {
    Ok(match predicate {
        Predicate::PurposeContains(v) => {
            AccelPredicate::Purpose(PurposePredicate::Contains(v.clone()))
        }
        Predicate::PurposeEquals(v) => AccelPredicate::Purpose(PurposePredicate::Equals(v.clone())),
        Predicate::PurposeStartsWith(v) => {
            AccelPredicate::Purpose(PurposePredicate::StartsWith(v.clone()))
        }
        Predicate::PurposeEndsWith(v) => {
            AccelPredicate::Purpose(PurposePredicate::EndsWith(v.clone()))
        }
        Predicate::PurposeIsEmpty => AccelPredicate::Purpose(PurposePredicate::IsEmpty),
        Predicate::ContextHasTag(v) => AccelPredicate::Context(ContextPredicate::HasTag(v.clone())),
        Predicate::ContextHasRole(v) => {
            AccelPredicate::Context(ContextPredicate::HasRole(parse_role(v)?))
        }
        Predicate::ContextContains(v) => {
            AccelPredicate::Context(ContextPredicate::Contains(v.clone()))
        }
        Predicate::ContextIsEmpty => AccelPredicate::Context(ContextPredicate::IsEmpty),
        Predicate::EnvVarExists(v) => {
            AccelPredicate::Environment(EnvironmentPredicate::VarExists(v.clone()))
        }
        Predicate::EnvVarEquals(k, v) => {
            AccelPredicate::Environment(EnvironmentPredicate::VarEquals(k.clone(), v.clone()))
        }
        Predicate::EnvCwdContains(v) => {
            AccelPredicate::Environment(EnvironmentPredicate::CwdContains(v.clone()))
        }
        Predicate::EnvPlatformIs(v) => {
            AccelPredicate::Environment(EnvironmentPredicate::PlatformIs(v.clone()))
        }
        Predicate::ResHasModel(v) => {
            AccelPredicate::Resources(ResourcesPredicate::HasModel(v.clone()))
        }
        Predicate::ResActiveModelIs(v) => {
            AccelPredicate::Resources(ResourcesPredicate::ActiveModelIs(v.clone()))
        }
        Predicate::ResHasTool(v) => {
            AccelPredicate::Resources(ResourcesPredicate::HasTool(v.clone()))
        }
        Predicate::ResToolEnabled(v) => {
            AccelPredicate::Resources(ResourcesPredicate::ToolEnabled(v.clone()))
        }
        Predicate::ResHasPrompt(v) => {
            AccelPredicate::Resources(ResourcesPredicate::HasPrompt(v.clone()))
        }
        Predicate::All(preds) => AccelPredicate::All(
            preds
                .iter()
                .map(convert_predicate)
                .collect::<Result<_, _>>()?,
        ),
        Predicate::Any(preds) => AccelPredicate::Any(
            preds
                .iter()
                .map(convert_predicate)
                .collect::<Result<_, _>>()?,
        ),
        Predicate::Not(pred) => AccelPredicate::Not(Box::new(convert_predicate(pred)?)),
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
