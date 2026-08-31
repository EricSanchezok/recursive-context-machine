use server::manager::MachineManager;
use server::rcm::{DestroyRequest, OpenRequest, StepRequest, rcm_server::Rcm};
use server::service::RcmService;
use storage::Store;
use tonic::Request;

fn new_service() -> RcmService {
    RcmService::new(MachineManager::new())
}

fn open_request(purpose: &str, run_dir: Option<String>) -> OpenRequest {
    OpenRequest {
        purpose: purpose.to_string(),
        run_dir,
        ..Default::default()
    }
}

#[tokio::test]
async fn open_step_destroy_produces_restorable_wal() {
    let run_dir = tempfile::tempdir().unwrap();
    let runtime = new_service();

    let response = runtime
        .open(Request::new(open_request(
            "trajectory integration",
            Some(run_dir.path().to_string_lossy().into_owned()),
        )))
        .await
        .unwrap()
        .into_inner();
    let machine_id = response.machine_id.clone();

    // State carries the observation channel: obs is present, budget exists
    // (zero-valued until a model with limits is registered).
    let state = response.state.unwrap();
    assert!(state.obs.is_some(), "State.obs must be populated");

    for _ in 0..3 {
        runtime
            .step(Request::new(StepRequest {
                machine_id: machine_id.clone(),
                command: Some(server::rcm::ActionCommand {
                    verb: "Halt".into(),
                    ..Default::default()
                }),
            }))
            .await
            .unwrap();
    }

    runtime
        .destroy(Request::new(DestroyRequest { machine_id }))
        .await
        .unwrap();

    // The WAL lands under run_dir/trajectory/<machine_id> — the same
    // layout the CLI writes — and restores to the server-side state.
    let wal_dir = run_dir.path().join("trajectory");
    let machine_wal: Vec<_> = std::fs::read_dir(&wal_dir)
        .expect("trajectory subdirectory must exist under the run dir")
        .map(|entry| entry.unwrap().path())
        .collect();
    assert_eq!(machine_wal.len(), 1, "exactly one per-machine WAL");

    let store = Store::open(&machine_wal[0]).unwrap();
    let restored = store.restore().await.unwrap().expect("recorded state");
    assert_eq!(restored.run.purpose.text, "trajectory integration");
    assert_eq!(restored.frame.step, 3, "three Halt steps recorded");

    let trajectories = store.trajectories().await.unwrap();
    assert_eq!(trajectories.len(), 3);
    assert!(
        trajectories
            .iter()
            .all(|trajectory| trajectory.event.action == machine::Action::Halt)
    );
}

#[tokio::test]
async fn open_without_run_dir_records_under_server_root() {
    let server_root = tempfile::tempdir().unwrap();
    // Scoped env var swap; nextest isolates each test in its own process,
    // and the value is restored for plain `cargo test` runs.
    let previous = std::env::var("RCM_SERVER_TRAJECTORY_DIR").ok();
    // Environment mutation is process-global and unsafe in edition 2024.
    unsafe {
        std::env::set_var("RCM_SERVER_TRAJECTORY_DIR", server_root.path().as_os_str());
    }

    let runtime = new_service();
    let response = runtime
        .open(Request::new(open_request("default root recording", None)))
        .await
        .unwrap()
        .into_inner();

    // The machine directory exists under the server root.
    let machine_dir = server_root.path().join(&response.machine_id);
    assert!(
        machine_dir.is_dir(),
        "machine trajectory directory must exist under the server root"
    );

    unsafe {
        if let Some(previous_value) = previous {
            std::env::set_var("RCM_SERVER_TRAJECTORY_DIR", previous_value);
        } else {
            std::env::remove_var("RCM_SERVER_TRAJECTORY_DIR");
        }
    }
}
