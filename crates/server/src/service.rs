use std::collections::HashMap;

use accelerator::{Catalog, ResourceSelection};
use machine::{ApplyContext, ApplyMode, Inbox, Machine};
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
        let req = request.into_inner();
        let mut catalog = self.catalog.clone();

        for spec in &req.model_definitions {
            catalog
                .register_model(build_model(spec)?)
                .map_err(Status::invalid_argument)?;
        }

        for spec in &req.mcp_definitions {
            catalog
                .register_mcp_server(crate::mcp::mcp_config_from_spec(spec)?)
                .map_err(Status::invalid_argument)?;
        }

        let environment_name = if req.environment.is_empty() {
            "local"
        } else {
            req.environment.as_str()
        };
        let environment = catalog
            .environment(environment_name)
            .map_err(Status::invalid_argument)?;
        let runtime_resources = catalog
            .build_runtime_resources(ResourceSelection {
                models: req.models,
                tools: req.tools,
                mcp_servers: req.mcps,
                prompt_texts: req.prompts,
            })
            .await
            .map_err(Status::invalid_argument)?;

        let machine_id = utils::MachineId::new();

        let run = Run {
            purpose: req.purpose,
            machine: Machine::new(machine_id.as_str(), "rcm"),
            ctx: machine::Context::new(),
            env: environment,
            resources: runtime_resources.resources,
            tool_runtime: runtime_resources.tool_runtime,
            inbox: Inbox::new(),
            usages: Vec::new(),
            counts: HashMap::new(),
            step: 0,
            done: false,
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
        let req = request.into_inner();
        let machine_id = utils::MachineId::from_raw(req.machine_id)
            .map_err(|error| Status::invalid_argument(error.to_string()))?;
        let command = req
            .command
            .ok_or(Status::invalid_argument("command required"))?;
        let action = decode_command(&command)?;
        let mut manager = self.manager.lock().await;
        let run = manager
            .get_mut(&machine_id)
            .ok_or(Status::not_found("machine_id not found"))?;

        run.step += 1;
        let result = run
            .machine
            .apply(
                action,
                run.step,
                ApplyContext {
                    ctx: &mut run.ctx,
                    env: &mut run.env,
                    resources: &mut run.resources,
                    inbox: &mut run.inbox,
                    usages: &mut run.usages,
                    counts: &mut run.counts,
                },
                ApplyMode::Live {
                    tool_runtime: &run.tool_runtime,
                },
            )
            .await;
        run.done = result.done;

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
        manager.destroy(&machine_id);
        Ok(Response::new(()))
    }
}
