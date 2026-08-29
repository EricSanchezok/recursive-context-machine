use accelerator::{Catalog, ResourceSelection};
use machine::{ExecutionMode, Machine, MachineFrame, MachineState, Purpose, RunState};
use tonic::{Request, Response, Status};

use crate::action_space::build_action_space;
use crate::decode::{build_model, decode_command};
use crate::manager::{MachineManager, Run};
use crate::rcm::{
    DestroyRequest, OpenRequest, OpenResponse, StepRequest, StepResponse, rcm_server::Rcm,
};
use crate::state::build_state;

pub struct RcmService {
    pub manager: std::sync::Arc<tokio::sync::Mutex<MachineManager>>,
    catalog: Catalog,
}

impl RcmService {
    pub fn new(manager: MachineManager) -> Self {
        Self::with_catalog(manager, Catalog::new())
    }

    pub fn with_catalog(manager: MachineManager, catalog: Catalog) -> Self {
        Self {
            manager: std::sync::Arc::new(tokio::sync::Mutex::new(manager)),
            catalog,
        }
    }
}

#[tonic::async_trait]
impl Rcm for RcmService {
    async fn open(&self, request: Request<OpenRequest>) -> Result<Response<OpenResponse>, Status> {
        let request = request.into_inner();
        let mut catalog = self.catalog.clone();

        for spec in &request.model_definitions {
            catalog
                .register_model(build_model(spec)?)
                .map_err(Status::invalid_argument)?;
        }

        for spec in &request.mcp_definitions {
            catalog
                .register_mcp_server(crate::mcp::mcp_config_from_spec(spec)?)
                .map_err(Status::invalid_argument)?;
        }

        let environment_name = if request.environment.is_empty() {
            "local"
        } else {
            request.environment.as_str()
        };
        let environment = catalog
            .environment(environment_name)
            .map_err(Status::invalid_argument)?;
        let runtime_resources = catalog
            .build_runtime_resources(ResourceSelection {
                models: request.models,
                tools: request.tools,
                mcp_servers: request.mcps,
                prompt_texts: request.prompts,
            })
            .await
            .map_err(Status::invalid_argument)?;

        let machine_id = utils::MachineId::new();
        // Trajectory recording is data-collection-on-by-default for the
        // server. An explicit run_dir is honored as the run directory with
        // the WAL under run_dir/trajectory/<machine_id> — the same layout
        // the CLI writes. Without one, the run directory is unset and the
        // WAL lands directly under RCM_SERVER_TRAJECTORY_DIR/<machine_id>.
        let (run_dir, trajectory_dir) = match request.run_dir.filter(|dir| !dir.is_empty()) {
            Some(explicit) => {
                let run_dir = std::path::PathBuf::from(explicit);
                let trajectory_dir = run_dir.join("trajectory").join(machine_id.as_str());
                (Some(run_dir), trajectory_dir)
            }
            None => {
                let root = std::env::var("RCM_SERVER_TRAJECTORY_DIR")
                    .unwrap_or_else(|_| "./rcm-trajectories".to_string());
                (None, std::path::Path::new(&root).join(machine_id.as_str()))
            }
        };
        let store = match storage::Store::open(&trajectory_dir) {
            Ok(store) => Some(store),
            Err(error) => {
                tracing::warn!(
                    dir = %trajectory_dir.display(),
                    ?error,
                    "trajectory store unavailable; run continues without recording"
                );
                None
            }
        };
        let run = Run {
            machine: Machine::new(machine_id.as_str(), "rcm"),
            state: MachineState {
                run: RunState {
                    purpose: Purpose::new(request.purpose),
                    run_dir,
                    context: machine::Context::new(),
                    environment,
                    resources: runtime_resources.resources,
                    telemetry: machine::Telemetry::default(),
                },
                frame: MachineFrame::default(),
            },
            tool_runtime: runtime_resources.tool_runtime,
            store,
        };

        let action_space = build_action_space(&run);
        let state = build_state(&run);

        {
            let mut manager = self.manager.lock().await;
            manager.insert(machine_id.clone(), run);
        }

        Ok(Response::new(OpenResponse {
            machine_id: machine_id.to_string(),
            state: Some(state),
            action_space: Some(action_space),
        }))
    }

    async fn step(&self, request: Request<StepRequest>) -> Result<Response<StepResponse>, Status> {
        let request = request.into_inner();
        let machine_id = utils::MachineId::from_raw(request.machine_id)
            .map_err(|error| Status::invalid_argument(error.to_string()))?;
        let command = request
            .command
            .ok_or(Status::invalid_argument("command required"))?;
        let action = decode_command(&command)?;
        let mut manager = self.manager.lock().await;
        let run = manager
            .get_mut(&machine_id)
            .ok_or(Status::not_found("machine_id not found"))?;

        // Decision-time observation, derived before the action is applied.
        // The gRPC Step path has no policy object to declare an overlay;
        // external controllers stay on the tape-only protocol (v1), so the
        // measured default (no declaration) is already accurate.
        let obs = machine::obs::measure(&run.state.run);
        let overlay_declared = machine::Overlay::default();
        let result = run
            .machine
            .apply(
                action,
                &mut run.state,
                ExecutionMode::Live {
                    tool_runtime: &run.tool_runtime,
                    overlay: &overlay_declared,
                },
            )
            .await;
        if let Some(ref mut store) = run.store {
            let trajectory = storage::TrajectoryEvent {
                step: result.event.step,
                obs,
                ledger_transitions: machine::ledger_transitions_in(&result.event.effects),
                event: result.event.clone(),
            };
            if let Err(error) = store.record_trajectory(&trajectory) {
                tracing::warn!(
                    machine_id = machine_id.as_str(),
                    ?error,
                    "trajectory write failed; step result is unaffected"
                );
            }
        }

        let action_space = build_action_space(run);
        let state = build_state(run);
        Ok(Response::new(StepResponse {
            state: Some(state),
            action_space: Some(action_space),
        }))
    }

    async fn destroy(&self, request: Request<DestroyRequest>) -> Result<Response<()>, Status> {
        let machine_id = utils::MachineId::from_raw(request.into_inner().machine_id)
            .map_err(|error| Status::invalid_argument(error.to_string()))?;
        let mut manager = self.manager.lock().await;
        if let Some(run) = manager.get_mut(&machine_id)
            && let Some(ref mut store) = run.store
            && let Err(error) = store.checkpoint(&run.state)
        {
            tracing::warn!(
                machine_id = machine_id.as_str(),
                ?error,
                "final trajectory checkpoint failed"
            );
        }
        manager.destroy(&machine_id);
        Ok(Response::new(()))
    }
}
