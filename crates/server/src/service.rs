use accelerator::Catalog;
use machine::{Inbox, Machine};
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
        Self {
            manager: std::sync::Arc::new(tokio::sync::Mutex::new(manager)),
            catalog: Catalog::new(),
        }
    }
}

#[tonic::async_trait]
impl Rcm for RcmService {
    async fn open(&self, request: Request<OpenRequest>) -> Result<Response<OpenResponse>, Status> {
        let req = request.into_inner();

        let mut resources = self.catalog.default_resources();

        if !req.tools.is_empty() {
            let selected = req
                .tools
                .iter()
                .cloned()
                .collect::<std::collections::HashSet<_>>();
            resources = resources.retain_tools(|name| selected.contains(name));
        }

        for spec in &req.models {
            let model = build_model(spec)?;
            resources = resources.with_model(model);
        }

        for (name, text) in &req.prompts {
            resources.prompts.insert(name.clone(), text.clone());
        }

        if !req.mcps.is_empty() {
            let configs: Vec<_> = req
                .mcps
                .iter()
                .map(crate::mcp::build_mcp_config)
                .collect::<Result<_, _>>()?;
            let registry = accelerator::mcp::McpRegistry::start(&configs)
                .await
                .map_err(|e| Status::internal(format!("mcp start failed: {}", e)))?;
            for spec in &req.mcps {
                if let Some(tools) = registry.tools_for(&spec.label) {
                    for tool in tools {
                        resources = resources.with_tool(tool);
                    }
                }
            }
        }

        let machine_id = utils::MachineId::new();

        let run = Run {
            purpose: req.purpose,
            machine: Machine::new(machine_id.as_str(), "rcm"),
            ctx: machine::Context::new(),
            env: self.catalog.default_environment(),
            resources,
            inbox: Inbox::new(),
            step: 0,
            done: false,
        };

        let action_space = build_action_space(&run);
        let state = build_state(&run);

        {
            let mut mgr = self.manager.lock().await;
            mgr.insert(machine_id.clone(), run);
        }

        Ok(Response::new(OpenResponse {
            machine_id: machine_id.to_string(),
            state: Some(state),
            action_space: Some(action_space),
        }))
    }

    async fn step(&self, request: Request<StepRequest>) -> Result<Response<StepResponse>, Status> {
        let req = request.into_inner();
        let mid = utils::MachineId::from_raw(req.machine_id)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;
        let command = req
            .command
            .ok_or(Status::invalid_argument("command required"))?;
        let action = decode_command(&command)?;
        let mut mgr = self.manager.lock().await;
        let run = mgr
            .get_mut(&mid)
            .ok_or(Status::not_found("machine_id not found"))?;

        run.step += 1;
        run.done = run
            .machine
            .apply(
                action,
                run.step,
                &mut run.ctx,
                &mut run.env,
                &mut run.resources,
                &mut run.inbox,
            )
            .await;

        let action_space = build_action_space(run);
        let state = build_state(run);

        Ok(Response::new(StepResponse {
            state: Some(state),
            action_space: Some(action_space),
        }))
    }

    async fn destroy(&self, request: Request<DestroyRequest>) -> Result<Response<()>, Status> {
        let req = request.into_inner();
        let mid = utils::MachineId::from_raw(req.machine_id)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;
        self.manager.lock().await.destroy(&mid);
        Ok(Response::new(()))
    }
}
