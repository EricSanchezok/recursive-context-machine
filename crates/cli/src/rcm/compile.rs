use std::collections::HashMap;

use accelerator::state::kit;
use accelerator::{
    ContextFlux, ContextPredicate, EnvFlux, EnvironmentPredicate, FluxMode, Graph, PolicyFlux,
    Predicate as AccelPredicate, PurposeFlux, PurposePredicate, ResFlux, ResourcesPredicate, State,
};

use super::ast::{self, PortDef, Predicate, RcmFile};

/// Compile a parsed `.rcm` file into a `Graph`.
pub fn compile(file: &RcmFile) -> Result<Graph, String> {
    let catalog = Catalog::default();

    let mut graph = Graph::named(file.name.as_str());
    let mut agent_map: HashMap<String, _> = HashMap::new();
    let mut flux_map: HashMap<String, _> = HashMap::new();
    let mut condition_map: HashMap<String, _> = HashMap::new();

    for agent_def in &file.agents {
        let state = catalog.build_state(agent_def)?;
        let ref_ = graph.spawn_named(
            agent_def.name.as_deref().unwrap_or(agent_def.id.as_str()),
            state,
        );
        agent_map.insert(agent_def.id.clone(), ref_);
    }

    for flux_def in &file.fluxes {
        let mode = resolve_flux_mode(flux_def)?;
        let ref_ = graph.weave_named(
            flux_def.name.as_deref().unwrap_or(flux_def.id.as_str()),
            2,
            mode,
        );
        flux_map.insert(flux_def.id.clone(), ref_);
    }

    for cond_def in &file.conditions {
        let predicate = convert_predicate(&cond_def.predicate);
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

fn resolve_flux_mode(def: &ast::FluxDef) -> Result<FluxMode, String> {
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

fn convert_predicate(predicate: &Predicate) -> AccelPredicate {
    match predicate {
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
            AccelPredicate::Context(ContextPredicate::HasRole(parse_role(v)))
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
        Predicate::All(preds) => AccelPredicate::All(preds.iter().map(convert_predicate).collect()),
        Predicate::Any(preds) => AccelPredicate::Any(preds.iter().map(convert_predicate).collect()),
        Predicate::Not(pred) => AccelPredicate::Not(Box::new(convert_predicate(pred))),
    }
}

fn parse_role(role: &str) -> machine::Role {
    match role {
        "system" => machine::Role::System,
        "user" => machine::Role::User,
        "assistant" => machine::Role::Assistant,
        "tool" => machine::Role::Tool,
        _ => machine::Role::System,
    }
}

struct Catalog {
    models: HashMap<String, machine::Model>,
}

impl Default for Catalog {
    fn default() -> Self {
        let mut models = HashMap::new();
        let ds = accelerator::model::deepseek_v4_flash();
        models.insert(ds.name.clone(), ds);
        let gpt = accelerator::model::gpt4_1();
        models.insert(gpt.name.clone(), gpt);
        Self { models }
    }
}

impl Catalog {
    fn build_state(&self, def: &ast::AgentDef) -> Result<State, String> {
        let model_name = def.model.as_deref().unwrap_or("deepseek-v4-flash");
        let model = self
            .models
            .get(model_name)
            .ok_or_else(|| format!("unknown model: {}", model_name))?;

        let mut resources = kit();
        resources = resources.with_model(model.clone());
        resources.use_model(model_name);

        for tool_name in &def.tools {
            resources.enable(tool_name);
        }

        Ok(State {
            purpose: def.purpose.clone().unwrap_or_default(),
            policy: Box::new(accelerator::policy::Captain::new()),
            res: resources,
            ..State::default()
        })
    }
}
