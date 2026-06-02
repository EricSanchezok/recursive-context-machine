use std::future::Future;
use std::pin::Pin;

use chrono::Local;
use machine::{
    Action, Content, Context, Environment, Fragment, Inbox, Policy, Purpose, Resources, Role,
};
use tracing::{trace, warn};

use super::retry::{HTTP_FORBIDDEN, HTTP_UNAUTHORIZED, Retry};

use super::{agent, instruction, purpose};

/// Captain — a simple single-agent Policy.
///
///   Inbox not empty             → Take
///   Inbox empty:
///     first call ever           → Halt
///     last is Hitch:
///       401/403                 → Done
///       transient, budget > 0   → backoff, Halt (retry)
///       budget exhausted        → Done
///     last is Tool              → Halt
///     last is not Tool          → Done
///
/// Before entering the main decide loop, Captain runs a setup state machine
/// that injects system prompts, instructions, purpose, activates tools/models,
/// and injects the env fragment — all via ordinary Action emissions.
pub struct Captain {
    setup: std::sync::atomic::AtomicU8,
    first_call: std::sync::atomic::AtomicBool,
    retry: Retry,
}

impl Clone for Captain {
    fn clone(&self) -> Self {
        Self {
            setup: std::sync::atomic::AtomicU8::new(0),
            first_call: std::sync::atomic::AtomicBool::new(false),
            retry: self.retry.clone(),
        }
    }
}

impl Default for Captain {
    fn default() -> Self {
        Self {
            setup: std::sync::atomic::AtomicU8::new(0),
            first_call: std::sync::atomic::AtomicBool::new(false),
            retry: Retry::default(),
        }
    }
}

impl Captain {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Captain {
    fn setup_step(
        &self,
        ctx: &Context,
        _env: &Environment,
        resources: &Resources,
        purpose: &Purpose,
    ) -> Option<Action> {
        loop {
            let step = self.setup.load(std::sync::atomic::Ordering::Relaxed);
            match step {
                0 => {
                    if let Some(action) = agent::ensure_agent_prompt(ctx, resources, "captain") {
                        return Some(action);
                    }
                    self.setup
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    continue;
                }
                1 => {
                    if let Some(action) = instruction::ensure_instructions(ctx) {
                        return Some(action);
                    }
                    self.setup
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    continue;
                }
                2 => {
                    if let Some(action) = purpose::ensure_purpose(ctx, purpose) {
                        return Some(action);
                    }
                    self.setup
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    continue;
                }
                3 => {
                    if resources.active_model.is_empty() {
                        if let Some(model_name) = resources.model_order.first() {
                            return Some(Action::Model(model_name.clone()));
                        }
                    }
                    if let Some(tool_name) = resources
                        .tools
                        .keys()
                        .find(|tool_name| !resources.active_tools.contains(*tool_name))
                    {
                        return Some(Action::Activate(tool_name.clone()));
                    }
                    self.setup
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    continue;
                }
                _ => return None,
            }
        }
    }

    fn env_action(&self, ctx: &Context, env: &Environment) -> Option<Action> {
        let now = Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false);
        let text = format!(
            "cwd: {}\nplatform: {}\ntime: {}",
            env.cwd.display(),
            env.platform,
            now,
        );

        if let Some(existing) = ctx
            .fragments()
            .iter()
            .find(|f| f.role == Role::System && f.tag == "env")
        {
            if existing.as_text() == Some(&text) {
                return None;
            }
            return Some(Action::Replace {
                id: existing.id(),
                fragment: Fragment::system(text).with_tag("env"),
            });
        }

        Some(Action::Append(Fragment::system(text).with_tag("env")))
    }
}

impl Policy for Captain {
    fn clone_box(&self) -> Box<dyn Policy> {
        Box::new(self.clone())
    }

    fn name(&self) -> &str {
        "captain"
    }

    fn decide<'a>(
        &'a self,
        purpose: &'a Purpose,
        ctx: &'a Context,
        env: &'a Environment,
        resources: &'a Resources,
        inbox: &'a Inbox,
    ) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>> {
        Box::pin(async move {
            if let Some(action) = self.setup_step(ctx, env, resources, purpose) {
                return action;
            }

            if inbox.peek().is_some() {
                return Action::Take;
            }

            if !self.first_call.load(std::sync::atomic::Ordering::Relaxed) {
                if let Some(action) = self.env_action(ctx, env) {
                    return action;
                }
                self.first_call
                    .store(true, std::sync::atomic::Ordering::Relaxed);
                trace!("decide: first call, halting");
                return Action::Halt;
            }

            let last = ctx.fragments().last();

            if let Some(frag) = last {
                if let Content::Hitch { code, .. } = &frag.content {
                    if let Some(c) = code {
                        if *c == HTTP_UNAUTHORIZED || *c == HTTP_FORBIDDEN {
                            warn!(code = *c, "decide: permanent hitch, done");
                            return Action::Done;
                        }
                    }

                    if self.retry.backoff().await {
                        let attempts = self.retry.count();
                        trace!(attempts, "decide: hitched, retrying");
                        if let Some(action) = self.env_action(ctx, env) {
                            return action;
                        }
                        return Action::Halt;
                    }
                    warn!("decide: retry budget exhausted, done");
                    return Action::Done;
                }
            }

            self.retry.reset();

            if let Some(action) = self.env_action(ctx, env) {
                return action;
            }

            match last.map(|f| f.role) {
                Some(Role::Tool) => {
                    trace!("decide: last is Tool, halting");
                    Action::Halt
                }
                _ => {
                    trace!("decide: done");
                    Action::Done
                }
            }
        })
    }
}
