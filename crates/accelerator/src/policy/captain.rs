use std::collections::HashSet;
use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;

use chrono::Local;
use machine::{
    Action, Content, Context, Environment, Fragment, Inbox, Policy, Purpose, Resources, Role,
};
use tracing::{trace, warn};

use super::retry::{HTTP_FORBIDDEN, HTTP_UNAUTHORIZED, Retry};

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
    /// Run the next setup step, returning an Action if one is needed.
    /// Returns `None` when setup is complete.
    fn setup_step(
        &self,
        ctx: &Context,
        _env: &Environment,
        resources: &Resources,
    ) -> Option<Action> {
        loop {
            let step = self
                .setup
                .fetch_add(0, std::sync::atomic::Ordering::Relaxed);
            match step {
                // Step 0: Bootstrap — inject/capture system prompt (tag="agent")
                0 => {
                    let desired = resources
                        .prompts
                        .get("captain")
                        .cloned()
                        .unwrap_or_default();
                    if let Some(existing) = ctx
                        .fragments()
                        .iter()
                        .find(|f| f.role == Role::System && f.tag == "agent")
                    {
                        if existing.as_text() == Some(&desired) {
                            self.setup
                                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            continue;
                        }
                        return Some(Action::Replace {
                            id: existing.id(),
                            fragment: Fragment::system(desired).with_tag("agent"),
                        });
                    }
                    self.setup
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    return Some(Action::Append(Fragment::system(desired).with_tag("agent")));
                }
                // Step 1: Instructions — inject instruction files (tag="instruction")
                1 => {
                    self.setup
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if ctx
                        .fragments()
                        .iter()
                        .any(|f| f.role == Role::System && f.tag == "instruction")
                    {
                        continue;
                    }
                    let files = find_instruction_files();
                    if files.is_empty() {
                        continue;
                    }
                    let parts: Vec<String> = files
                        .iter()
                        .filter(|(_, content)| !content.trim().is_empty())
                        .map(|(path, content)| {
                            let name = path
                                .file_name()
                                .unwrap_or(path.as_os_str())
                                .to_string_lossy();
                            format!(
                                "=== {name} (from {}) ===\n{}",
                                path.display(),
                                content.trim()
                            )
                        })
                        .collect();
                    if parts.is_empty() {
                        continue;
                    }
                    return Some(Action::Append(
                        Fragment::system(parts.join("\n\n")).with_tag("instruction"),
                    ));
                }
                // Step 2: Skip (Purpose — already injected from outside)
                2 => {
                    self.setup
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    continue;
                }
                // Step 3: ResourceSetup — activate first model and all tools
                // Retryable: stays at step 3 until both model and tools are set up.
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
                    // All done — advance to running.
                    self.setup
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    continue;
                }
                // Step 4+: Running — setup is done
                _ => return None,
            }
        }
    }

    /// Inject or replace the env fragment (tag="env") — called before every Halt.
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
        _purpose: &'a Purpose,
        ctx: &'a Context,
        env: &'a Environment,
        resources: &'a Resources,
        inbox: &'a Inbox,
    ) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>> {
        Box::pin(async move {
            // Run setup steps until one emits an action or all are done.
            if let Some(action) = self.setup_step(ctx, env, resources) {
                return action;
            }

            if inbox.peek().is_some() {
                return Action::Take;
            }

            if !self
                .first_call
                .swap(true, std::sync::atomic::Ordering::Relaxed)
            {
                // First real Halt — inject env and go
                if let Some(action) = self.env_action(ctx, env) {
                    return action;
                }
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

// ── Instruction file scanning (moved from phases/instruct.rs) ──

const FILE_NAMES: [&str; 3] = ["AGENTS.md", "CLAUDE.md", "CONTEXT.md"];

fn global_paths() -> Vec<PathBuf> {
    let Some(home) = std::env::var("HOME").ok() else {
        return Vec::new();
    };
    vec![
        Path::new(&home).join(".synergy/config/AGENTS.md"),
        Path::new(&home).join(".claude/CLAUDE.md"),
    ]
}

fn find_instruction_files() -> Vec<(PathBuf, String)> {
    let mut seen = HashSet::new();
    let mut results = Vec::new();

    if let Ok(cwd) = std::env::current_dir() {
        let mut dir = Some(cwd);
        while let Some(d) = dir {
            if !seen.insert(d.clone()) {
                break;
            }
            for name in &FILE_NAMES {
                let path = d.join(name);
                if path.is_file()
                    && let Ok(content) = std::fs::read_to_string(&path)
                {
                    results.push((path, content));
                }
            }
            dir = d.parent().map(|p| p.to_path_buf());
        }
    }

    for path in global_paths() {
        if path.is_file()
            && !results.iter().any(|(p, _)| *p == path)
            && let Ok(content) = std::fs::read_to_string(&path)
        {
            results.push((path, content));
        }
    }

    results
}
