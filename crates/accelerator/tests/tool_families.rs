//! Tool-family end-to-end tests (C3): `context.compact` and `memory.*`
//! driven through the real fire loop by scripted policies and a stub
//! assistant — no LLM, no network. Verifies the full chain the Blueprint
//! requires: tool edits payload → drain channel → document change → WAL
//! drain_effects → deterministic replay.

use std::future::Future;
use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;

use accelerator::Accelerator;
use accelerator::assistant::StubAssistant;
use accelerator::tools::{
    ContextCompactTool, MemorySearchTool, MemoryWriteTool, reset_memory_for_tests,
};
use machine::edit::{ContentSpec, EditOp, Position};
use machine::{
    Action, CompletionAssistant, Fragment, Machine, MachineState, Policy, PolicyView, Purpose,
    RunState, Tool, ToolDefinition, ToolRuntime,
};
use storage::Store;

/// One scripted action per step, then Done. `decide` runs before `apply`
/// increments the frame step, so the action for step N+1 sits at index N.
struct ScriptedPolicy {
    actions: Vec<Action>,
}

impl Policy for ScriptedPolicy {
    fn clone_box(&self) -> Box<dyn Policy> {
        Box::new(ScriptedPolicy {
            actions: self.actions.clone(),
        })
    }

    fn decide<'a>(
        &'a self,
        view: PolicyView<'a>,
    ) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>> {
        let action = self
            .actions
            .get(view.step as usize)
            .cloned()
            .unwrap_or(Action::Done);
        Box::pin(async move { action })
    }
}

fn insert_user_notes(notes: &[&str]) -> Action {
    Action::Edit {
        ops: notes
            .iter()
            .map(|note| EditOp::Insert {
                position: Position::End,
                content: ContentSpec::Literal {
                    text: (*note).to_string(),
                    role: machine::Role::User,
                    tag: None,
                },
                anchor: None,
            })
            .collect(),
        because: None,
    }
}

fn tool_action(name: &str, args: serde_json::Value) -> Action {
    Action::Tool {
        name: name.to_string(),
        args,
        because: None,
    }
}

/// The primitive's base state carries resources and the environment; the
/// input state (see `run_input`) intentionally leaves the environment cwd
/// empty so the merge adopts this environment wholesale (assistant handle
/// and run_dir ride along).
fn base_state(run_dir: &Path, assistant: Option<Arc<dyn CompletionAssistant>>) -> RunState {
    let mut state = RunState {
        run_dir: Some(run_dir.to_path_buf()),
        ..Default::default()
    };
    // The environment's private id keeps nested assignment instead of a
    // full literal; the lint only fires on direct field reassignment.
    state.environment.run_dir = Some(run_dir.to_path_buf());
    state.environment.assistant = assistant;
    state
}

fn register_active(state: &mut RunState, tool: Arc<dyn Tool>) {
    state.resources.tool_definitions.insert(
        tool.name().to_string(),
        ToolDefinition::from_tool(tool.as_ref()),
    );
    let _ = state.resources.enable(tool.name());
}

fn run_input(run_dir: &Path, purpose: &str) -> RunState {
    // Empty cwd makes the merge adopt the base environment (assistant,
    // run_dir) instead of the host-inheriting default.
    RunState {
        purpose: Purpose::new(purpose),
        run_dir: Some(run_dir.to_path_buf()),
        environment: machine::Environment::empty(""),
        ..Default::default()
    }
}

fn tool_runtime_of(tools: Vec<Arc<dyn Tool>>) -> ToolRuntime {
    let mut runtime = ToolRuntime::new();
    for tool in tools {
        runtime.insert(tool);
    }
    runtime
}

/// The single per-machine trajectory WAL directory the recorder created.
fn only_trajectory_dir(run_dir: &Path) -> std::path::PathBuf {
    let entries: Vec<std::path::PathBuf> = std::fs::read_dir(run_dir.join("trajectory"))
        .expect("trajectory directory exists")
        .map(|entry| entry.unwrap().path())
        .collect();
    assert_eq!(entries.len(), 1, "one machine means one WAL");
    entries.into_iter().next().unwrap()
}

fn text_of_anchor(context: &machine::Context, anchor: &str) -> Option<String> {
    context
        .fragments()
        .iter()
        .find(|cell| cell.anchor.as_deref() == Some(anchor))
        .map(Fragment::content_as_text)
}

fn tool_step_contents(envelope: &storage::TrajectoryEvent) -> Vec<String> {
    envelope
        .event
        .effects
        .iter()
        .filter_map(|effect| match effect {
            machine::Effect::InboxPushed { item } => match &item.fragment.content {
                machine::Content::ToolResult(result) => Some(result.content.clone()),
                machine::Content::Hitch { message, .. } => Some(message.clone()),
                _ => None,
            },
            _ => None,
        })
        .collect()
}

#[tokio::test]
async fn compact_full_sets_summary_deletes_range_and_replays_from_wal() {
    let run_dir = tempfile::tempdir().unwrap();
    let stub = Arc::new(StubAssistant::default());
    stub.enqueue("SUMMARY: alpha and beta notes consolidated");

    let compact = Arc::new(ContextCompactTool);
    let mut state = base_state(
        run_dir.path(),
        Some(stub.clone() as Arc<dyn CompletionAssistant>),
    );
    register_active(&mut state, compact.clone());

    let policy = ScriptedPolicy {
        actions: vec![
            insert_user_notes(&["alpha note", "beta note"]),
            tool_action(
                "context.compact",
                serde_json::json!({
                    "range": {"Where": {"role": "user"}},
                    "style": "full",
                }),
            ),
            Action::Done,
        ],
    };
    let accelerator = Accelerator::primitive(
        state,
        Box::new(policy),
        tool_runtime_of(vec![compact]),
        "compact-full",
    );
    let output = accelerator
        .run_with(run_input(run_dir.path(), "compact full e2e"))
        .await;

    // Document: @summary landed (idempotent slot), compacted range gone.
    assert_eq!(
        text_of_anchor(&output.context, "@summary").as_deref(),
        Some("SUMMARY: alpha and beta notes consolidated")
    );
    let remaining: Vec<String> = output
        .context
        .fragments()
        .iter()
        .map(Fragment::content_as_text)
        .collect();
    assert!(
        !remaining
            .iter()
            .any(|text| text == "alpha note" || text == "beta note"),
        "compacted range must be deleted, got {remaining:?}"
    );

    let store = Store::open(only_trajectory_dir(run_dir.path())).unwrap();
    let envelopes = store.trajectories().await.unwrap();
    assert_eq!(envelopes.len(), 3);

    // The tool step records metered usage in ToolCompleted.
    let tool_step = &envelopes[1];
    let tool_completed = tool_step
        .event
        .effects
        .iter()
        .find_map(|effect| match effect {
            machine::Effect::ToolCompleted { tokens, .. } => Some(tokens.clone()),
            _ => None,
        })
        .expect("ToolCompleted effect");
    assert_eq!(
        tool_completed.map(|tokens| tokens.total_tokens),
        Some(15),
        "stub assistant usage must ride the WAL"
    );

    // Drain effects are nested under one DrainEdits entry carrying the ops.
    assert_eq!(tool_step.drain_effects.len(), 1);
    let machine::Effect::DrainEdits { ops, effects } = &tool_step.drain_effects[0] else {
        panic!("expected DrainEdits, got {:?}", tool_step.drain_effects);
    };
    assert_eq!(ops.len(), 2, "set @summary then delete range");
    assert!(effects.iter().any(|effect| matches!(
        effect,
        machine::Effect::ContextSet { anchor, .. } if anchor == "@summary"
    )));

    // Deterministic replay: apply every envelope's event effects and drain
    // effects in recorded order onto a fresh state — same document.
    let machine = Machine::new("replay", "replay");
    let mut replayed = MachineState::default();
    for envelope in &envelopes {
        replayed.frame.step = envelope.event.step;
        machine.replay_effects(&mut replayed, &envelope.event.effects);
        machine.replay_effects(&mut replayed, &envelope.drain_effects);
    }
    assert_eq!(
        replayed.run.context.fragments(),
        output.context.fragments(),
        "replayed document must equal the live one"
    );
    assert_eq!(
        text_of_anchor(&replayed.run.context, "@summary").as_deref(),
        Some("SUMMARY: alpha and beta notes consolidated")
    );
}

#[tokio::test]
async fn compact_rolling_refreshes_summary_without_deleting() {
    let run_dir = tempfile::tempdir().unwrap();
    let stub = Arc::new(StubAssistant::default());
    stub.enqueue("ROLLING: merged state");

    let compact = Arc::new(ContextCompactTool);
    let mut state = base_state(
        run_dir.path(),
        Some(stub.clone() as Arc<dyn CompletionAssistant>),
    );
    register_active(&mut state, compact.clone());

    let policy = ScriptedPolicy {
        actions: vec![
            insert_user_notes(&["kept note"]),
            tool_action(
                "context.compact",
                serde_json::json!({
                    "range": {"Where": {"role": "user"}},
                    "style": "rolling",
                }),
            ),
            Action::Done,
        ],
    };
    let accelerator = Accelerator::primitive(
        state,
        Box::new(policy),
        tool_runtime_of(vec![compact]),
        "compact-rolling",
    );
    let output = accelerator
        .run_with(run_input(run_dir.path(), "compact rolling e2e"))
        .await;

    assert_eq!(
        text_of_anchor(&output.context, "@summary").as_deref(),
        Some("ROLLING: merged state")
    );
    assert!(
        output
            .context
            .fragments()
            .iter()
            .any(|cell| cell.content_as_text() == "kept note"),
        "rolling style must keep the source range"
    );
}

#[tokio::test]
async fn compact_without_model_surfaces_hitch_not_panic() {
    let run_dir = tempfile::tempdir().unwrap();
    // No stub injected: the fire loop installs the production gateway,
    // which has no active model — the documented LLM-unavailable branch.
    let compact = Arc::new(ContextCompactTool);
    let mut state = base_state(run_dir.path(), None);
    register_active(&mut state, compact.clone());

    let policy = ScriptedPolicy {
        actions: vec![
            insert_user_notes(&["note"]),
            tool_action(
                "context.compact",
                serde_json::json!({"range": {"Where": {"role": "user"}}}),
            ),
            Action::Done,
        ],
    };
    let accelerator = Accelerator::primitive(
        state,
        Box::new(policy),
        tool_runtime_of(vec![compact]),
        "compact-nomodel",
    );
    let output = accelerator
        .run_with(run_input(run_dir.path(), "compact without model"))
        .await;

    // The failure is a hitch in the step's effects — visible, not fatal,
    // and no summary slot is created.
    assert!(text_of_anchor(&output.context, "@summary").is_none());
    let store = Store::open(only_trajectory_dir(run_dir.path())).unwrap();
    let envelopes = store.trajectories().await.unwrap();
    let step_payloads = tool_step_contents(&envelopes[1]);
    assert!(
        step_payloads
            .iter()
            .any(|message| message.contains("no active model")),
        "expected a no-model hitch, got {step_payloads:?}"
    );
    // The failed compact contributes no drain edits.
    assert!(envelopes[1].drain_effects.is_empty());
}

#[tokio::test]
async fn memory_write_search_roundtrip_through_fire_loop() {
    reset_memory_for_tests();
    let run_dir = tempfile::tempdir().unwrap();

    let write_tool = Arc::new(MemoryWriteTool);
    let search_tool = Arc::new(MemorySearchTool);
    let mut state = base_state(run_dir.path(), None);
    register_active(&mut state, write_tool.clone());
    register_active(&mut state, search_tool.clone());

    let policy = ScriptedPolicy {
        actions: vec![
            tool_action(
                "memory.write",
                serde_json::json!({"key": "goals", "content": "ship the document model alpha"}),
            ),
            tool_action(
                "memory.write",
                serde_json::json!({"key": "notes", "content": "beta drafts live here"}),
            ),
            tool_action("memory.search", serde_json::json!({"query": "alpha beta"})),
            Action::Done,
        ],
    };
    let accelerator = Accelerator::primitive(
        state,
        Box::new(policy),
        tool_runtime_of(vec![write_tool, search_tool]),
        "memory-e2e",
    );
    accelerator
        .run_with(run_input(run_dir.path(), "memory e2e"))
        .await;

    // Notes persist under the run directory.
    let memory_file = run_dir.path().join("memory.json");
    let raw = std::fs::read_to_string(&memory_file).expect("memory.json persisted");
    let persisted: serde_json::Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(
        persisted["goals"]["content"],
        "ship the document model alpha"
    );
    assert_eq!(persisted["notes"]["content"], "beta drafts live here");

    // The search result rides the step's tool-result fragment.
    let store = Store::open(only_trajectory_dir(run_dir.path())).unwrap();
    let envelopes = store.trajectories().await.unwrap();
    let search_payloads = tool_step_contents(&envelopes[2]);
    let search_result = search_payloads
        .iter()
        .find(|content| content.contains("memory.search"))
        .expect("search result in inbox");
    let payload: serde_json::Value = serde_json::from_str(search_result).unwrap();
    let matches = payload["matches"].as_array().unwrap();
    let keys: Vec<&str> = matches
        .iter()
        .map(|entry| entry["key"].as_str().unwrap())
        .collect();
    assert!(
        keys.contains(&"goals") && keys.contains(&"notes"),
        "got {keys:?}"
    );
}

/// Blueprint acceptance #2: one scripted run through the whole document
/// lifecycle — inbox consumption, idempotent Set, predicate Delete, Move —
/// with the WAL restoring the identical document.
#[tokio::test]
async fn document_lifecycle_replays_identically_from_wal() {
    let run_dir = tempfile::tempdir().unwrap();
    let compact = Arc::new(ContextCompactTool);
    let stub = Arc::new(StubAssistant::default());
    stub.enqueue("merged summary");
    let mut state = base_state(
        run_dir.path(),
        Some(stub.clone() as Arc<dyn CompletionAssistant>),
    );
    register_active(&mut state, compact.clone());

    let policy = ScriptedPolicy {
        actions: vec![
            // Step 1: build a document — anchored slot plus loose cells.
            Action::Edit {
                ops: vec![
                    EditOp::Set {
                        anchor: "@plan".into(),
                        content: ContentSpec::Literal {
                            text: "first plan".into(),
                            role: machine::Role::System,
                            tag: None,
                        },
                    },
                    EditOp::Insert {
                        position: Position::End,
                        content: ContentSpec::Literal {
                            text: "stale scratch".into(),
                            role: machine::Role::User,
                            tag: None,
                        },
                        anchor: None,
                    },
                    EditOp::Insert {
                        position: Position::End,
                        content: ContentSpec::Literal {
                            text: "fresh scratch".into(),
                            role: machine::Role::User,
                            tag: None,
                        },
                        anchor: None,
                    },
                ],
                because: None,
            },
            // Step 2: idempotent Set on the same slot (replace in place,
            // no duplicate @plan), create @reflection, then Move it to sit
            // after @plan — a real relocation ("End" is a refused move
            // target by design, covered in machine tests).
            Action::Edit {
                ops: vec![
                    EditOp::Set {
                        anchor: "@plan".into(),
                        content: ContentSpec::Literal {
                            text: "revised plan".into(),
                            role: machine::Role::System,
                            tag: None,
                        },
                    },
                    EditOp::Set {
                        anchor: "@reflection".into(),
                        content: ContentSpec::Literal {
                            text: "reflections".into(),
                            role: machine::Role::System,
                            tag: None,
                        },
                    },
                    EditOp::Move {
                        anchor: "@reflection".into(),
                        after: Position::Anchor("@plan".into()),
                    },
                ],
                because: None,
            },
            // Step 3: compact consumes a completion and its drain payload
            // writes @summary then deletes the user range (drain channel
            // exercised inside the same WAL).
            tool_action(
                "context.compact",
                serde_json::json!({
                    "range": {"Where": {"role": "user"}},
                    "style": "full",
                }),
            ),
            Action::Done,
        ],
    };
    let accelerator = Accelerator::primitive(
        state,
        Box::new(policy),
        tool_runtime_of(vec![compact]),
        "lifecycle",
    );
    let output = accelerator
        .run_with(run_input(run_dir.path(), "document lifecycle"))
        .await;

    // Live document: one @plan (revised), @summary present, user range gone.
    let plan_cells = output
        .context
        .fragments()
        .iter()
        .filter(|cell| cell.anchor.as_deref() == Some("@plan"))
        .count();
    assert_eq!(plan_cells, 1, "Set must be idempotent");
    assert_eq!(
        text_of_anchor(&output.context, "@plan").as_deref(),
        Some("revised plan")
    );
    assert_eq!(
        text_of_anchor(&output.context, "@summary").as_deref(),
        Some("merged summary")
    );
    let remaining: Vec<String> = output
        .context
        .fragments()
        .iter()
        .map(Fragment::content_as_text)
        .collect();
    assert!(
        !remaining.iter().any(|text| text.contains("scratch")),
        "compacted user range must be gone: {remaining:?}"
    );

    // The WAL restores the identical document — event effects and drain
    // effects applied in recorded order.
    let store = Store::open(only_trajectory_dir(run_dir.path())).unwrap();
    let restored = store.restore().await.unwrap().expect("recorded state");
    assert_eq!(
        restored.run.context.fragments(),
        output.context.fragments(),
        "restored document must equal the live one"
    );
}

/// Blueprint acceptance #5: the recorded envelope caps its obs directory
/// copy at ENVELOPE_DIRECTORY_ROWS rows while `context_directory_total`
/// carries the exact count.
#[tokio::test]
async fn envelope_caps_directory_rows_but_keeps_exact_total() {
    let run_dir = tempfile::tempdir().unwrap();
    let state = base_state(run_dir.path(), None);

    // One Edit inserting 40 loose cells, then Done.
    let cells: Vec<&str> = (1..=40).map(leak_indexed).collect();
    let policy = ScriptedPolicy {
        actions: vec![insert_user_notes(&cells), Action::Done],
    };
    let accelerator = Accelerator::primitive(
        state,
        Box::new(policy),
        tool_runtime_of(Vec::new()),
        "truncation",
    );
    let output = accelerator
        .run_with(run_input(run_dir.path(), "directory truncation"))
        .await;
    assert_eq!(output.context.fragments().len(), 40);

    let store = Store::open(only_trajectory_dir(run_dir.path())).unwrap();
    let envelopes = store.trajectories().await.unwrap();
    // obs is decision-time: envelope #1 (step 2's decide) sees all 40 cells.
    let recorded = &envelopes[1].obs;
    assert_eq!(
        recorded.context_directory.len(),
        machine::obs::ENVELOPE_DIRECTORY_ROWS,
        "envelope keeps only the first rows"
    );
    assert_eq!(
        recorded.context_directory_total, 40,
        "total rides alongside the truncation"
    );
}

/// Leaked string literals keep the test's borrowed slice simple; 40 tiny
/// allocations for one test process are irrelevant.
fn leak_indexed(index: i32) -> &'static str {
    Box::leak(format!("cell {index}").into_boxed_str())
}
