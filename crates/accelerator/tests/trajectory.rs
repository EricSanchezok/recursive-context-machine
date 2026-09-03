use std::future::Future;
use std::pin::Pin;

use accelerator::Accelerator;
use machine::edit::{ContentSpec, EditOp, Position};
use machine::{Action, Policy, PolicyView, Purpose, Role, RunState, ToolRuntime};
use storage::Store;

fn insert_end_action(text: String) -> Action {
    Action::Edit {
        ops: vec![EditOp::Insert {
            position: Position::End,
            content: ContentSpec::Literal {
                text,
                role: Role::User,
                tag: None,
            },
            anchor: None,
        }],
        because: None,
    }
}

/// Scripted policy: append one fragment, then finish. Exercises the
/// trajectory recorder through the real accelerator run loop without any
/// LLM dependency.
#[derive(Clone)]
struct AppendOncePolicy {
    fragment_text: String,
}

impl Policy for AppendOncePolicy {
    fn clone_box(&self) -> Box<dyn Policy> {
        Box::new(self.clone())
    }

    fn decide<'a>(
        &'a self,
        view: PolicyView<'a>,
    ) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>> {
        let already_appended = view
            .run
            .context
            .fragments()
            .iter()
            .any(|fragment| fragment.as_text() == Some(self.fragment_text.as_str()));
        let fragment_text = self.fragment_text.clone();
        Box::pin(async move {
            if already_appended {
                Action::Done
            } else {
                insert_end_action(fragment_text)
            }
        })
    }
}

#[tokio::test]
async fn run_with_run_dir_records_restorable_trajectory() {
    let run_dir = tempfile::tempdir().unwrap();
    let accelerator = Accelerator::primitive(
        RunState::default(),
        Box::new(AppendOncePolicy {
            fragment_text: "recorded step".into(),
        }),
        ToolRuntime::new(),
        "trajectory-test",
    );

    let mut input = RunState {
        purpose: Purpose::new("verify trajectory recording"),
        ..RunState::default()
    };
    input.run_dir = Some(run_dir.path().to_path_buf());
    let output = accelerator.run_with(input).await;

    assert_eq!(
        output.context.fragments().last().and_then(|f| f.as_text()),
        Some("recorded step")
    );

    // The recorder created exactly one per-machine WAL under trajectory/.
    let trajectory_root = run_dir.path().join("trajectory");
    let machine_dirs: Vec<_> = std::fs::read_dir(&trajectory_root)
        .expect("trajectory directory must exist when run_dir is set")
        .map(|entry| entry.unwrap().path())
        .collect();
    assert_eq!(
        machine_dirs.len(),
        1,
        "one machine means exactly one trajectory WAL"
    );
    assert!(
        machine_dirs[0]
            .file_name()
            .unwrap()
            .to_string_lossy()
            .starts_with("trajectory-test-"),
        "WAL directory is labeled with accelerator name and id, got {:?}",
        machine_dirs[0]
    );

    // The recorded WAL restores to the same final state as the live run.
    let store = Store::open(&machine_dirs[0]).unwrap();
    let restored = store.restore().await.unwrap().expect("recorded state");
    assert_eq!(
        restored.run.context.fragments(),
        output.context.fragments(),
        "restored context must equal live run output"
    );
    assert_eq!(restored.frame.step, 2, "append + done = two steps");

    let trajectories = store.trajectories().await.unwrap();
    assert_eq!(trajectories.len(), 2);
    assert_eq!(
        trajectories[0].event.action,
        insert_end_action("recorded step".into())
    );
    assert_eq!(trajectories[1].event.action, Action::Done);
    // Decision-time observation snapshots ride along with every event.
    assert_eq!(trajectories[0].step, 1);
    assert_eq!(trajectories[0].obs.budget.estimated_input, 0);
}

#[tokio::test]
async fn run_without_run_dir_writes_nothing() {
    let accelerator = Accelerator::primitive(
        RunState::default(),
        Box::new(AppendOncePolicy {
            fragment_text: "no recording".into(),
        }),
        ToolRuntime::new(),
        "no-run-dir",
    );
    let output = accelerator
        .run_with(RunState {
            purpose: Purpose::new("no run dir"),
            ..RunState::default()
        })
        .await;

    assert_eq!(
        output.context.fragments().last().and_then(|f| f.as_text()),
        Some("no recording")
    );
    // Recording is opt-in: without run_dir nothing is written anywhere,
    // including the process working directory (no chdir tricks needed —
    // the recorder is simply None).
    assert!(
        !std::path::Path::new("trajectory").exists(),
        "no run_dir means no trajectory directory in the working directory"
    );
}
