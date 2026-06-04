use machine::hook;
use machine::{Action, Fragment, Inbox, Machine, Purpose, Role};
use serde_json::Value;
use std::future::Future;
use std::pin::Pin;
use utils::{AcceleratorId, Name};

use crate::flux::{ContextFlux, FluxMode};
use crate::graph::Graph;
use crate::state::State;
use crate::wire::{Channel, ComponentId, Endpoint, Port};

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

    /// A `Map` accelerator: fans out an inner accelerator over a runtime-sized
    /// list of items derived from the input, runs them concurrently, and gathers
    /// the results into one output. From the enclosing graph's perspective it is
    /// an ordinary accelerator node (one input, one output); the dynamic fan-out
    /// happens entirely inside [`Accelerator::run_with`].
    pub fn map_named(
        name: impl Into<String>,
        inner: Accelerator,
        scatter: ScatterSpec,
        gather: GatherSpec,
    ) -> Self {
        Self {
            id: AcceleratorId::new(),
            name: Name::new(name).expect("accelerator name must be valid"),
            body: AcceleratorBody::Map(MapAccelerator {
                inner: Box::new(inner),
                scatter,
                gather,
            }),
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
                AcceleratorBody::Map(map) => map.run(input).await,
            }
        })
    }

    pub fn internal_state(&self) -> Option<&State> {
        match &self.body {
            AcceleratorBody::Primitive(primitive) => Some(&primitive.state),
            AcceleratorBody::Composite(_) => None,
            AcceleratorBody::Map(_) => None,
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
    Map(MapAccelerator),
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

/// How a [`MapAccelerator`] splits one input [`State`] into per-item states.
#[derive(Clone, Copy)]
pub enum ScatterSpec {
    /// Parse the input context's last assistant message as a JSON array; each
    /// element becomes one item, carried as a tagged user fragment appended to a
    /// clone of the shared input context. A non-array (or empty) message yields a
    /// single item equal to the input, so a Map never hard-fails on malformed
    /// upstream output.
    Json,
}

/// How a [`MapAccelerator`] merges per-item output states into one output. The
/// merge is performed by an ordinary k-arity [`Flux`](crate::flux::Flux) in the
/// inner graph — see [`merge_mode`].
#[derive(Clone, Copy)]
pub enum GatherSpec {
    /// Digest each item's output context into one compact roll-up
    /// ([`ContextFlux::Digest`]). Per-item artifacts live on disk, so the
    /// gathered context is a summary, not the full bodies.
    Digest,
}

/// Tag for the per-item fragment a Map injects into each inner run's context.
const ITEM_TAG: &str = "item";

#[derive(Clone)]
struct MapAccelerator {
    inner: Box<Accelerator>,
    scatter: ScatterSpec,
    gather: GatherSpec,
}

impl MapAccelerator {
    /// Fan out by building an inner graph at run time and running it with the
    /// ordinary [`GraphRun`](crate::graph): `k` seeded workers (clones of the
    /// inner accelerator, one per scattered item) run concurrently as a single
    /// frontier, then a k-arity merge [`Flux`](crate::flux::Flux) gathers them.
    /// Externally the Map is one node; its children are added dynamically here.
    async fn run(self, input: State) -> State {
        let items = scatter(self.scatter, &input);
        let k = items.len();
        hook!(event = "map_start", items = k);

        let mut graph = Graph::named("map");
        let merge = graph.add_flux("merge", merge_mode(self.gather), k);
        let mut seeds: Vec<(ComponentId, State)> = Vec::with_capacity(k);
        for (index, item) in items.into_iter().enumerate() {
            let worker = graph.add_accelerator(format!("item_{index}"), (*self.inner).clone());
            graph.wire(worker.context(), merge.slot(index, Channel::Context));
            seeds.push((worker.id(), item));
        }
        graph.wire(
            merge.flux_out(Channel::Context),
            Port::output(Endpoint::State(Channel::Context)),
        );

        let result = graph.run_seeded(State::default(), seeds).await;
        hook!(event = "map_done", items = k);
        result
    }
}

fn scatter(spec: ScatterSpec, input: &State) -> Vec<State> {
    match spec {
        ScatterSpec::Json => scatter_json(input),
    }
}

fn scatter_json(input: &State) -> Vec<State> {
    let Some(text) = last_assistant_text(input) else {
        return vec![input.clone()];
    };
    let Ok(Value::Array(elements)) = serde_json::from_str::<Value>(text.trim()) else {
        return vec![input.clone()];
    };
    if elements.is_empty() {
        return vec![input.clone()];
    }
    elements
        .into_iter()
        .map(|element| {
            let item_text = match &element {
                Value::String(s) => s.clone(),
                other => serde_json::to_string_pretty(other).unwrap_or_else(|_| other.to_string()),
            };
            let mut item = input.clone();
            item.ctx.append(
                Fragment::user(format!("Your assigned work item:\n{item_text}")).with_tag(ITEM_TAG),
            );
            item
        })
        .collect()
}

/// The flux mode used to merge the per-item worker outputs in the inner graph.
fn merge_mode(spec: GatherSpec) -> FluxMode {
    match spec {
        GatherSpec::Digest => FluxMode::Context(ContextFlux::Digest),
    }
}

/// The text of the most recent assistant fragment in a state's context, if any.
fn last_assistant_text(state: &State) -> Option<&str> {
    state
        .ctx
        .fragments()
        .iter()
        .rev()
        .find(|fragment| fragment.role == Role::Assistant)
        .and_then(|fragment| fragment.as_text())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state_with_assistant(text: &str) -> State {
        let mut state = State::default();
        state.ctx.append(Fragment::assistant(text));
        state
    }

    #[test]
    fn json_array_of_strings_scatters_into_one_item_each() {
        let input = state_with_assistant(r#"["alpha", "beta", "gamma"]"#);
        let items = scatter(ScatterSpec::Json, &input);
        assert_eq!(items.len(), 3);
        for (item, expected) in items.iter().zip(["alpha", "beta", "gamma"]) {
            let last = item.ctx.fragments().last().expect("item fragment");
            assert_eq!(last.role, Role::User);
            assert_eq!(last.tag, ITEM_TAG);
            assert!(last.as_text().unwrap().contains(expected));
        }
    }

    #[test]
    fn json_array_of_objects_serializes_each_element() {
        let input = state_with_assistant(r#"[{"section":"A"},{"section":"B"}]"#);
        let items = scatter(ScatterSpec::Json, &input);
        assert_eq!(items.len(), 2);
        let first = items[0].ctx.fragments().last().unwrap().as_text().unwrap();
        assert!(first.contains("section"));
        assert!(first.contains('A'));
    }

    #[test]
    fn non_array_message_falls_back_to_a_single_item() {
        let input = state_with_assistant("just a normal handoff, not JSON");
        assert_eq!(scatter(ScatterSpec::Json, &input).len(), 1);
    }

    #[test]
    fn empty_context_falls_back_to_a_single_item() {
        let input = State::default();
        let items = scatter(ScatterSpec::Json, &input);
        assert_eq!(items.len(), 1);
    }

    #[test]
    fn empty_json_array_falls_back_to_a_single_item() {
        let input = state_with_assistant("[]");
        assert_eq!(scatter(ScatterSpec::Json, &input).len(), 1);
    }
}
