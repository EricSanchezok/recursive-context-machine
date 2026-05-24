use machine::{Action, Content, Fragment, Inbox, Machine, Role};
use tonic::{Request, Response, Status};

use crate::manager::{MachineId, MachineManager, Run};
use crate::rcm::{
    self, ActionCommand, ActionItem, ActionSpace, DestroyRequest, FragmentContent, OpenRequest,
    OpenResponse, State, StepRequest, StepResponse, rcm_server::Rcm,
};

pub struct RcmService {
    pub manager: std::sync::Arc<tokio::sync::Mutex<MachineManager>>,
}

impl RcmService {
    pub fn new(manager: MachineManager) -> Self {
        Self {
            manager: std::sync::Arc::new(tokio::sync::Mutex::new(manager)),
        }
    }
}

// ── State marshalling ──────────────────────────────────────

fn build_state(run: &Run) -> State {
    State {
        purpose: String::new(),
        fragments: run.ctx.fragments().iter().map(fragment_to_proto).collect(),
        workdir: run.env.cwd.to_string_lossy().into_owned(),
        env_vars: run.env.vars.clone(),
        active_model: run.resources.active_model.clone(),
        active_tools: run.resources.active_tools.iter().cloned().collect(),
        available_models: run.resources.model_order.clone(),
        available_tools: run.resources.tools.keys().cloned().collect(),
        done: run.done,
        step: run.step,
        inbox_pending: run.inbox.peek().is_some(),
        inbox_peek: run.inbox.peek().map(fragment_to_proto),
    }
}

fn fragment_to_proto(f: &Fragment) -> rcm::Fragment {
    let kind = match &f.content {
        Content::Text(_) => "text",
        Content::ToolCall(_) => "tool_call",
        Content::ToolResult(_) => "tool_result",
        Content::Hitch { .. } => "hitch",
        Content::Image(_) => "image",
        Content::Audio(_) => "audio",
        Content::Video(_) => "video",
        Content::Document(_) => "document",
    };
    rcm::Fragment {
        id: f.id(),
        role: role_label(f.role).into(),
        kind: kind.into(),
        text_preview: clip(f),
        tag: Some(f.tag.clone()),
    }
}

fn role_label(role: Role) -> &'static str {
    match role {
        Role::System => "system",
        Role::User => "user",
        Role::Assistant => "assistant",
        Role::Tool => "tool",
    }
}

fn clip(f: &Fragment) -> String {
    let full = match &f.content {
        Content::Text(t) => t.text.clone(),
        Content::Hitch { message, .. } => message.clone(),
        Content::ToolCall(tc) => format!("tool_call: {}", tc.name),
        Content::ToolResult(tr) => format!("tool_result: {}", tr.title.as_deref().unwrap_or("")),
        _ => String::new(),
    };
    if full.len() <= 200 {
        full
    } else {
        let mut clipped: String = full.chars().take(200).collect();
        clipped.push_str("...");
        clipped
    }
}

// ── Action space builder ───────────────────────────────────

fn build_action_space(run: &Run) -> ActionSpace {
    let mut actions = Vec::new();

    if run.inbox.is_empty() {
        // Free mode.
        actions.push(ActionItem {
            command: Some(ActionCommand {
                verb: "Halt".into(),
                ..Default::default()
            }),
            label: "Halt".into(),
            sink: None,
        });

        // Append — one per prompt resource
        for (name, text) in &run.resources.prompts {
            let fc = FragmentContent {
                role: "system".into(),
                text: text.clone(),
            };
            actions.push(ActionItem {
                command: Some(ActionCommand {
                    verb: "Append".into(),
                    fragment: Some(fc.clone()),
                    ..Default::default()
                }),
                label: format!("Append {}", name),
                sink: Some(sink_clip(fc)),
            });
        }

        // Remove — one per context fragment
        for frag in run.ctx.fragments().iter() {
            actions.push(ActionItem {
                command: Some(ActionCommand {
                    verb: "Remove".into(),
                    fragment_id: Some(frag.id()),
                    ..Default::default()
                }),
                label: format!("Remove #{}", frag.id()),
                sink: None,
            });
        }

        // Replace — fragment × prompt
        for frag in run.ctx.fragments().iter() {
            for (name, text) in &run.resources.prompts {
                let fc = FragmentContent {
                    role: "system".into(),
                    text: text.clone(),
                };
                actions.push(ActionItem {
                    command: Some(ActionCommand {
                        verb: "Replace".into(),
                        fragment_id: Some(frag.id()),
                        fragment: Some(fc.clone()),
                        ..Default::default()
                    }),
                    label: format!("Replace #{} with {}", frag.id(), name),
                    sink: Some(sink_clip(fc)),
                });
            }
        }

        // Model / Activate / Deactivate
        for model_name in &run.resources.model_order {
            actions.push(ActionItem {
                command: Some(ActionCommand {
                    verb: "Model".into(),
                    name: Some(model_name.clone()),
                    ..Default::default()
                }),
                label: format!("Model {}", model_name),
                sink: None,
            });
        }
        for tool_name in run.resources.tools.keys() {
            actions.push(ActionItem {
                command: Some(ActionCommand {
                    verb: "Activate".into(),
                    name: Some(tool_name.clone()),
                    ..Default::default()
                }),
                label: format!("Activate {}", tool_name),
                sink: None,
            });
        }
        for tool_name in &run.resources.active_tools {
            actions.push(ActionItem {
                command: Some(ActionCommand {
                    verb: "Deactivate".into(),
                    name: Some(tool_name.clone()),
                    ..Default::default()
                }),
                label: format!("Deactivate {}", tool_name),
                sink: None,
            });
        }
    } else {
        // Consumption mode.
        actions.push(ActionItem {
            command: Some(ActionCommand {
                verb: "Take".into(),
                ..Default::default()
            }),
            label: "Take".into(),
            sink: None,
        });
        for frag in run.ctx.fragments().iter() {
            actions.push(ActionItem {
                command: Some(ActionCommand {
                    verb: "Remove".into(),
                    fragment_id: Some(frag.id()),
                    ..Default::default()
                }),
                label: format!("Remove #{}", frag.id()),
                sink: None,
            });
        }
    }

    if !run.done {
        actions.push(ActionItem {
            command: Some(ActionCommand {
                verb: "Done".into(),
                ..Default::default()
            }),
            label: "Done".into(),
            sink: None,
        });
    }

    ActionSpace { actions }
}

fn sink_clip(fc: FragmentContent) -> FragmentContent {
    if fc.text.len() <= 200 {
        fc
    } else {
        let mut clipped: String = fc.text.chars().take(200).collect();
        clipped.push_str("...");
        FragmentContent {
            text: clipped,
            ..fc
        }
    }
}

// ── Action decode ────────────────────────────────────────

fn build_fragment(fc: &FragmentContent) -> Fragment {
    match fc.role.as_str() {
        "user" => Fragment::user(&fc.text),
        _ => Fragment::system(&fc.text),
    }
}

fn decode_command(cmd: &ActionCommand) -> Result<Action, Status> {
    match cmd.verb.as_str() {
        "Halt" => Ok(Action::Halt),
        "Done" => Ok(Action::Done),
        "Take" => Ok(Action::Take),
        "Append" => {
            let fc = cmd
                .fragment
                .as_ref()
                .ok_or(Status::invalid_argument("fragment required"))?;
            Ok(Action::Append(build_fragment(fc)))
        }
        "Remove" => {
            let id = cmd
                .fragment_id
                .ok_or(Status::invalid_argument("fragment_id required"))?;
            Ok(Action::Remove(id))
        }
        "Swap" => {
            let id1 = cmd
                .fragment_id
                .ok_or(Status::invalid_argument("fragment_id required"))?;
            let id2 = cmd
                .fragment_id2
                .ok_or(Status::invalid_argument("fragment_id2 required"))?;
            Ok(Action::Swap(id1, id2))
        }
        "Insert" => {
            let after = cmd
                .fragment_id
                .ok_or(Status::invalid_argument("fragment_id required"))?;
            let fc = cmd
                .fragment
                .as_ref()
                .ok_or(Status::invalid_argument("fragment required"))?;
            Ok(Action::Insert {
                after,
                fragment: build_fragment(fc),
            })
        }
        "Replace" => {
            let id = cmd
                .fragment_id
                .ok_or(Status::invalid_argument("fragment_id required"))?;
            let fc = cmd
                .fragment
                .as_ref()
                .ok_or(Status::invalid_argument("fragment required"))?;
            Ok(Action::Replace {
                id,
                fragment: build_fragment(fc),
            })
        }
        "Model" => {
            let name = cmd
                .name
                .as_ref()
                .ok_or(Status::invalid_argument("name required"))?;
            Ok(Action::Model(name.clone()))
        }
        "Activate" => {
            let name = cmd
                .name
                .as_ref()
                .ok_or(Status::invalid_argument("name required"))?;
            Ok(Action::Activate(name.clone()))
        }
        "Deactivate" => {
            let name = cmd
                .name
                .as_ref()
                .ok_or(Status::invalid_argument("name required"))?;
            Ok(Action::Deactivate(name.clone()))
        }
        other => Err(Status::invalid_argument(format!("unknown verb: {}", other))),
    }
}

// ── gRPC handlers ────────────────────────────────────────

#[tonic::async_trait]
impl Rcm for RcmService {
    async fn open(&self, request: Request<OpenRequest>) -> Result<Response<OpenResponse>, Status> {
        let req = request.into_inner();

        let mut resources = accelerator::state::kit();
        for name in &req.models {
            let model = machine::Model {
                name: name.clone(),
                ..Default::default()
            };
            resources = resources.with_model(model);
        }
        resources.deactivate_model();

        for (name, text) in &req.prompts {
            resources.prompts.insert(name.clone(), text.clone());
        }

        let run = Run {
            machine: Machine::new(),
            ctx: machine::Context::new(),
            env: accelerator::state::local(),
            resources,
            inbox: Inbox::new(),
            step: 0,
            done: false,
        };

        let action_space = build_action_space(&run);
        let state = build_state(&run);

        let machine_id = {
            let mut mgr = self.manager.lock().await;
            mgr.create(run)
        };

        Ok(Response::new(OpenResponse {
            machine_id: machine_id.into(),
            state: Some(state),
            action_space: Some(action_space),
        }))
    }

    async fn step(&self, request: Request<StepRequest>) -> Result<Response<StepResponse>, Status> {
        let req = request.into_inner();
        let mid = MachineId::from(req.machine_id);
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
                mid.as_str(),
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
        let mid = MachineId::from(req.machine_id);
        self.manager.lock().await.destroy(&mid);
        Ok(Response::new(()))
    }
}
