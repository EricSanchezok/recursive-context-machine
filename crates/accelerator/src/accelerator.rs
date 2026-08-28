use std::sync::Arc;

use machine::hook;
use machine::{
    Action, ExecutionMode, Fragment, Machine, MachineFrame, MachineState, PolicyView, Role,
    RunState, Tool, ToolRuntime,
};
use utils::{AcceleratorId, Name};

use crate::graph::Graph;

fn is_scaffolding(fragment: &Fragment) -> bool {
    fragment.role == Role::System
        && (fragment.tag == "agent" || fragment.tag == "instruction" || fragment.tag == "env")
}

fn is_purpose_tag(fragment: &Fragment) -> bool {
    fragment.role == Role::User
        && (fragment.tag == "purpose"
            || fragment.tag == "purpose_initial"
            || fragment.tag == "purpose_b")
}

#[derive(Clone)]
pub struct Accelerator {
    id: AcceleratorId,
    pub name: Name,
    body: AcceleratorBody,
}

impl Accelerator {
    pub fn primitive(
        state: RunState,
        policy: Box<dyn machine::Policy>,
        tool_runtime: ToolRuntime,
        name: impl Into<String>,
    ) -> Self {
        Self {
            id: AcceleratorId::new(),
            name: Name::new(name).expect("accelerator name must be valid"),
            body: AcceleratorBody::Primitive(Box::new(PrimitiveAccelerator {
                state,
                policy,
                tool_runtime,
            })),
        }
    }

    pub fn composite(graph: Graph) -> Self {
        let name = graph.name.clone();
        Self::composite_named(name.as_str(), graph)
    }

    pub fn composite_named(name: impl Into<String>, graph: Graph) -> Self {
        Self {
            id: AcceleratorId::new(),
            name: Name::new(name).expect("accelerator name must be valid"),
            body: AcceleratorBody::Composite(graph),
        }
    }

    /// Inject a tool into this accelerator's tool runtime and tool definitions.
    /// Used by the DSL compiler to wire spawn tools into a planner.
    pub fn inject_tool(&mut self, tool: Arc<dyn Tool>) {
        if let AcceleratorBody::Primitive(ref mut primitive) = self.body {
            let name = tool.name().to_string();
            primitive
                .state
                .resources
                .tool_definitions
                .entry(name.clone())
                .or_insert_with(|| machine::ToolDefinition::from_tool(tool.as_ref()));
            primitive.tool_runtime.insert(tool);
        }
    }

    pub fn id(&self) -> &AcceleratorId {
        &self.id
    }

    pub fn run_with(
        self,
        input: RunState,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = RunState> + Send>> {
        Box::pin(async move {
            let input = self.merge_input(input);
            match self.body {
                AcceleratorBody::Primitive(primitive) => primitive.fire(input).await,
                AcceleratorBody::Composite(graph) => graph.run(input).await,
            }
        })
    }

    pub fn internal_state(&self) -> Option<&RunState> {
        match &self.body {
            AcceleratorBody::Primitive(primitive) => Some(&primitive.state),
            AcceleratorBody::Composite(_) => None,
        }
    }

    fn merge_input(&self, input: RunState) -> RunState {
        let mut state = input;
        if let AcceleratorBody::Primitive(primitive) = &self.body {
            let base = &primitive.state;
            let base_purpose = base.purpose.text.clone();
            if !state.purpose.is_empty()
                && !base_purpose.is_empty()
                && state.purpose.text != base_purpose
            {
                state.purpose.text = format!("{}\n\n{}", state.purpose.text, base_purpose);
            } else if state.purpose.is_empty() {
                state.purpose.text.clone_from(&base_purpose);
            }
            if state.run_dir.is_none() {
                state.run_dir.clone_from(&base.run_dir);
                // When a child inherits run_dir, also use the parent's cwd
                // so fs tools resolve paths inside the run directory.
                if let Some(ref dir) = state.run_dir {
                    state.environment.cwd = dir.clone();
                }
            }
            if state.context.is_empty() {
                state.context = base.context.clone();
            }
            if state.environment.cwd.as_os_str().is_empty() {
                state.environment = base.environment.clone();
            } else if state.environment.run_dir.is_none() {
                state
                    .environment
                    .run_dir
                    .clone_from(&base.environment.run_dir);
            }
            state.resources.models.clone_from(&base.resources.models);
            state
                .resources
                .model_order
                .clone_from(&base.resources.model_order);
            state
                .resources
                .tool_definitions
                .clone_from(&base.resources.tool_definitions);
            state.resources.prompts.clone_from(&base.resources.prompts);
            state
                .resources
                .active_model
                .clone_from(&base.resources.active_model);
            state
                .resources
                .active_tools
                .clone_from(&base.resources.active_tools);
        }
        state
    }
}

#[derive(Clone)]
#[allow(clippy::large_enum_variant)]
enum AcceleratorBody {
    Primitive(Box<PrimitiveAccelerator>),
    Composite(Graph),
}

#[derive(Clone)]
struct PrimitiveAccelerator {
    state: RunState,
    policy: Box<dyn machine::Policy>,
    tool_runtime: ToolRuntime,
}

impl PrimitiveAccelerator {
    async fn fire(self, state: RunState) -> RunState {
        let mut machine_state = MachineState {
            run: state,
            frame: MachineFrame::default(),
        };
        let base_purpose = machine_state.run.purpose.text.clone();
        let needs_reorder = machine_state
            .run
            .context
            .fragments()
            .iter()
            .any(|fragment| !is_scaffolding(fragment) && !is_purpose_tag(fragment));

        let mut machine = Machine::new("ephemeral", "ephemeral");
        let policy = self.policy;
        let tool_runtime = self.tool_runtime;
        let mut reorder_pending = needs_reorder;

        hook!(event = "machine_start", purpose = %machine_state.run.purpose.text);

        loop {
            // Derived fresh each step: obs must never be a stale snapshot.
            let mut obs = machine::obs::measure(&machine_state.run);
            let action = policy
                .decide(PolicyView {
                    run: &machine_state.run,
                    inbox: &machine_state.frame.inbox,
                    step: machine_state.frame.step,
                    status: machine_state.frame.status,
                    obs: &obs,
                })
                .await;

            if matches!(&action, Action::Halt) && reorder_pending {
                reorder_pending = false;
                reorder_context_before_first_halt(&mut machine_state.run, &base_purpose);
            }

            // Overlay is declared alongside the decision but consumed only
            // when that decision is Halt; every other action carries an
            // empty declaration.
            let overlay_declared = if matches!(action, Action::Halt) {
                let overlay = policy.overlay(&PolicyView {
                    run: &machine_state.run,
                    inbox: &machine_state.frame.inbox,
                    step: machine_state.frame.step,
                    status: machine_state.frame.status,
                    obs: &obs,
                });
                obs.overlay_status = machine::OverlayStatus {
                    declared: !overlay.is_empty(),
                    system_prefix_count: overlay.system_prefix.len() as u64,
                    tail_count: overlay.tail.len() as u64,
                };
                overlay
            } else {
                machine::Overlay::default()
            };

            let result = machine
                .apply(
                    action,
                    &mut machine_state,
                    ExecutionMode::Live {
                        tool_runtime: &tool_runtime,
                        overlay: &overlay_declared,
                    },
                )
                .await;
            if result.done {
                break;
            }
        }

        machine_state.run
    }
}

fn reorder_context_before_first_halt(state: &mut RunState, base_purpose: &str) {
    let env_position = state
        .context
        .fragments()
        .iter()
        .position(|fragment| is_scaffolding(fragment) && fragment.tag == "env");
    let Some(env_position) = env_position else {
        return;
    };
    let env_id = state.context.fragments()[env_position].id();
    let before_env = state.context.fragments()[..env_position]
        .iter()
        .filter(|fragment| !is_scaffolding(fragment) && !is_purpose_tag(fragment))
        .map(|fragment| (fragment.id(), fragment.clone()))
        .collect::<Vec<_>>();
    if before_env.is_empty() {
        return;
    }
    for (id, _) in &before_env {
        let _ = state.context.remove(*id);
    }
    let mut cursor = env_id;
    for (_, fragment) in &before_env {
        if let Ok(new_id) = state.context.insert(cursor, fragment.clone()) {
            cursor = new_id;
        }
    }
    if !base_purpose.is_empty() {
        let _ = state
            .context
            .insert(cursor, Fragment::user(base_purpose).with_tag("purpose_b"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use machine::Fragment;

    fn state_with_assistant(text: &str) -> RunState {
        let mut state = RunState::default();
        state.context.append(Fragment::assistant(text));
        state
    }

    #[test]
    fn run_dir_recovered_from_handoff() {
        let input = state_with_assistant(
            "run_dir: runs/20260604T110702Z\nartifact: runs/20260604T110702Z/00_card_plan.json\nstatus: ok",
        );
        assert!(input.context.fragments().iter().any(|f| {
            f.as_text()
                .unwrap_or("")
                .contains("run_dir: runs/20260604T110702Z")
        }));
    }
}
