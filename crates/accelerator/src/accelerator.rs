use machine::hook;
use machine::{Action, Fragment, Inbox, Machine, Purpose, Role};
use std::future::Future;
use std::pin::Pin;
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
        name: impl Into<String>,
    ) -> Self {
        Self {
            id: AcceleratorId::new(),
            name: Name::new(name).expect("accelerator name must be valid"),
            body: AcceleratorBody::Primitive(PrimitiveAccelerator { state, policy }),
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

    pub fn id(&self) -> &AcceleratorId {
        &self.id
    }

    pub fn run_with(self, input: State) -> Pin<Box<dyn Future<Output = State> + Send>> {
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
            if state.purpose.is_empty() {
                state.purpose.clone_from(&base.purpose);
            }
            if state.ctx.is_empty() {
                // Only fill fold_payload from base when the input didn't
                // provide one (e.g. Fold mode provides fold_payload via
                // the Context channel even though ctx is empty).
                if state.fold_payload.is_empty() {
                    state.fold_payload.clone_from(&base.fold_payload);
                }
                state.ctx = base.ctx.clone();
            }
            if state.env.cwd.as_os_str().is_empty() {
                state.env = base.env.clone();
            }
            state.res.models.clone_from(&base.res.models);
            state.res.model_order.clone_from(&base.res.model_order);
            state.res.tools.clone_from(&base.res.tools);
            state.res.prompts.clone_from(&base.res.prompts);
            state.res.active_model.clone_from(&base.res.active_model);
            state.res.active_tools.clone_from(&base.res.active_tools);
        }
        state
    }
}

#[derive(Clone)]
// Graph is inherently larger than PrimitiveAccelerator; boxing the
// Composite variant would add a pointer traversal at every match site
// with no measurable performance benefit for this enum.
#[allow(clippy::large_enum_variant)]
enum AcceleratorBody {
    Primitive(PrimitiveAccelerator),
    Composite(Graph),
}

#[derive(Clone)]
struct PrimitiveAccelerator {
    state: State,
    policy: Box<dyn machine::Policy>,
}

impl PrimitiveAccelerator {
    async fn fire(self, mut state: State) -> State {
        let base_purpose = state.purpose.clone();
        let fold_payload = state.fold_payload.clone();

        // Fold mode: merge fold_payload into purpose so that Captain's
        // setup_step injects a purpose that already contains upstream info.
        let combined_purpose = if !fold_payload.is_empty() {
            format!("{}\n\n{}", fold_payload, base_purpose)
        } else {
            base_purpose.clone()
        };
        let purpose = Purpose::new(&combined_purpose);

        // Determine whether we need to reorder on first Halt.
        // For non-fold modes with upstream content: scaffolding comes from
        // Captain setup, but non-scaffolding fragments from flux may sit
        // before or among the scaffolding. On first Halt we move them after
        // the env fragment and inject purpose_b.
        let has_content_to_move = !fold_payload.is_empty() // fold: info is in purpose, no ctx content
            || state.ctx.fragments().iter().any(|f| {
                !is_scaffolding(f) && !is_purpose_tag(f)
            });
        let needs_reorder = has_content_to_move && fold_payload.is_empty();

        let mut inbox = Inbox::new();
        let mut step = 0u64;
        let mut machine = Machine::new("ephemeral", "ephemeral");
        let policy = self.policy;
        let mut reorder_pending = needs_reorder;

        hook!(event = "machine_start", purpose = %purpose.text);

        loop {
            step += 1;
            let action = policy
                .decide(&purpose, &state.ctx, &state.env, &state.res, &inbox)
                .await;

            // Intercept the first Halt (after Captain setup completes) to
            // reorder context: move upstream non-scaffolding content to
            // after the env fragment, then insert purpose_b.
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

            if machine
                .apply(
                    action,
                    step,
                    &mut state.ctx,
                    &mut state.env,
                    &mut state.res,
                    &mut inbox,
                )
                .await
            {
                break;
            }
        }

        state
    }
}
