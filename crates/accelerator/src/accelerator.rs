use std::sync::Arc;

use machine::hook;
use machine::{
    Action, ApplyContext, ApplyMode, Fragment, Inbox, Machine, Purpose, Role, ToolRuntime,
};
use utils::{AcceleratorId, Name};

use crate::graph::Graph;
use crate::state::State;

/// Scaffolding tags that Captain injects/replaces during setup.
fn is_scaffolding(f: &Fragment) -> bool {
    f.role == Role::System && (f.tag == "agent" || f.tag == "instruction" || f.tag == "env")
}

/// Purpose tags injected by Captain or `fire()`.
fn is_purpose_tag(f: &Fragment) -> bool {
    f.role == Role::User
        && (f.tag == "purpose" || f.tag == "purpose_initial" || f.tag == "purpose_b")
}

#[derive(Clone)]
pub struct Accelerator {
    id: AcceleratorId,
    pub name: Name,
    body: AcceleratorBody,
}

impl Accelerator {
    pub fn primitive(
        state: State,
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
    pub fn inject_tool(&mut self, tool: Arc<dyn machine::Tool>) {
        if let AcceleratorBody::Primitive(ref mut primitive) = self.body {
            let name = tool.name().to_string();
            primitive
                .state
                .res
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
        input: State,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = State> + Send>> {
        Box::pin(async move {
            let input = self.merge_input(input);
            match self.body {
                AcceleratorBody::Primitive(primitive) => primitive.fire(input).await,
                AcceleratorBody::Composite(graph) => graph.run(input).await,
            }
        })
    }

    pub fn internal_state(&self) -> Option<&State> {
        match &self.body {
            AcceleratorBody::Primitive(primitive) => Some(&primitive.state),
            AcceleratorBody::Composite(_) => None,
        }
    }

    fn merge_input(&self, input: State) -> State {
        let mut state = input;
        if let AcceleratorBody::Primitive(primitive) = &self.body {
            let base = &primitive.state;
            // Concatenate wired purpose with the accelerator's base purpose.
            let base_purpose = base.purpose.clone();
            if !state.purpose.is_empty()
                && !base_purpose.is_empty()
                && state.purpose != base_purpose
            {
                state.purpose = format!("{}\n\n{}", state.purpose, base_purpose);
            } else if state.purpose.is_empty() {
                state.purpose.clone_from(&base_purpose);
            }
            if state.ctx.is_empty() {
                state.ctx = base.ctx.clone();
            }
            if state.env.cwd.as_os_str().is_empty() {
                state.env = base.env.clone();
            }
            state.res.models.clone_from(&base.res.models);
            state.res.model_order.clone_from(&base.res.model_order);
            state
                .res
                .tool_definitions
                .clone_from(&base.res.tool_definitions);
            state.res.prompts.clone_from(&base.res.prompts);
            state.res.active_model.clone_from(&base.res.active_model);
            state.res.active_tools.clone_from(&base.res.active_tools);
        }
        state
    }
}

#[derive(Clone)]
enum AcceleratorBody {
    Primitive(Box<PrimitiveAccelerator>),
    Composite(Graph),
}

#[derive(Clone)]
struct PrimitiveAccelerator {
    state: State,
    policy: Box<dyn machine::Policy>,
    tool_runtime: ToolRuntime,
}

impl PrimitiveAccelerator {
    async fn fire(self, mut state: State) -> State {
        let base_purpose = state.purpose.clone();
        let purpose = Purpose::new(&base_purpose);

        // Reorder non-scaffolding context content on first Halt.
        let needs_reorder = state
            .ctx
            .fragments()
            .iter()
            .any(|f| !is_scaffolding(f) && !is_purpose_tag(f));

        let mut inbox = Inbox::new();
        let mut step = 0u64;
        let mut machine = Machine::new("ephemeral", "ephemeral");
        let policy = self.policy;
        let tool_runtime = self.tool_runtime;
        let mut reorder_pending = needs_reorder;

        hook!(event = "machine_start", purpose = %purpose.text);

        loop {
            step += 1;
            let action = policy
                .decide(&purpose, &state.ctx, &state.env, &state.res, &inbox)
                .await;

            // Intercept the first Halt to reorder and inject purpose_b.
            if matches!(&action, Action::Halt) && reorder_pending {
                reorder_pending = false;
                let env_pos = state
                    .ctx
                    .fragments()
                    .iter()
                    .position(|f| is_scaffolding(f) && f.tag == "env");
                if let Some(env_pos) = env_pos {
                    let env_id = state.ctx.fragments()[env_pos].id();

                    // Collect non-scaffolding, non-purpose fragments before env.
                    let before_env: Vec<(u64, Fragment)> = state.ctx.fragments()[..env_pos]
                        .iter()
                        .filter(|f| !is_scaffolding(f) && !is_purpose_tag(f))
                        .map(|f| (f.id(), f.clone()))
                        .collect();

                    if !before_env.is_empty() {
                        // Remove them.
                        for (id, _) in &before_env {
                            let _ = state.ctx.remove(*id);
                        }
                        // Re-insert after env.
                        let mut cursor = env_id;
                        for (_, frag) in &before_env {
                            if let Ok(nid) = state.ctx.insert(cursor, frag.clone()) {
                                cursor = nid;
                            }
                        }

                        // Insert purpose_b after the last re-inserted fragment.
                        if !base_purpose.is_empty() {
                            let _ = state.ctx.insert(
                                cursor,
                                Fragment::user(&base_purpose).with_tag("purpose_b"),
                            );
                        }
                    }
                }
            }

            let result = machine
                .apply(
                    action,
                    step,
                    ApplyContext {
                        ctx: &mut state.ctx,
                        env: &mut state.env,
                        resources: &mut state.res,
                        inbox: &mut inbox,
                        usages: &mut state.usages,
                        counts: &mut state.counts,
                    },
                    ApplyMode::Live {
                        tool_runtime: &tool_runtime,
                    },
                )
                .await;
            if result.done {
                break;
            }
        }

        state
    }
}
