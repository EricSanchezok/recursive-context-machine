use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};

use machine::{Action, Content, Context, Environment, Fragment, Inbox, Policy, Resources};
use serde_json::Value;

/// Captain — the default steering policy.
///
/// A 5-phase state machine that boots the context, drains the inbox,
/// and executes tool calls in a loop until the conversation reaches
/// a stable state.
///
/// ```text
/// Phase 1 (Boot):   ctx has sys prompt? → Halt : Append(sys) → Phase 2
/// Phase 2 (Halt):   Halt → Phase 3
/// Phase 3 (Drain):  inbox empty? → Phase 4 : Take (stay)
/// Phase 4 (React):  Scan for unanswered ToolCalls:
///                     - should_halt set → clear, Halt → Phase 3
///                     - unanswered TC → execute, Append, set should_halt
///                     - none → Done
/// ```
pub struct Captain {
    phase: AtomicU8,
    should_halt: AtomicBool,
}

impl Default for Captain {
    fn default() -> Self {
        Self::new()
    }
}

impl Captain {
    /// Create a new Captain policy.
    pub fn new() -> Self {
        Self {
            phase: AtomicU8::new(1),
            should_halt: AtomicBool::new(false),
        }
    }

    /// Check whether the context already contains a system prompt.
    fn has_system_prompt(&self, ctx: &Context) -> bool {
        ctx.fragments()
            .iter()
            .any(|f| f.role == machine::Role::System)
    }
}

impl Policy for Captain {
    fn decide<'a>(
        &'a self,
        ctx: &'a Context,
        _env: &'a Environment,
        resources: &'a Resources,
        inbox: &'a Inbox,
    ) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>> {
        Box::pin(async move {
            let phase = self.phase.load(Ordering::Relaxed);

            match phase {
                // ── Phase 1: Boot ──
                // Inject the system prompt only if the context is empty
                // of system-level instructions.
                1 => {
                    if self.has_system_prompt(ctx) {
                        self.phase.store(2, Ordering::Relaxed);
                        return Action::Halt;
                    }
                    let prompt = resources
                        .prompts
                        .get("default")
                        .cloned()
                        .unwrap_or_default();
                    self.phase.store(2, Ordering::Relaxed);
                    Action::Append(Fragment::system(prompt))
                }

                // ── Phase 2: Halt ──
                2 => {
                    self.phase.store(3, Ordering::Relaxed);
                    Action::Halt
                }

                // ── Phase 3: Drain inbox ──
                3 => {
                    if inbox.is_empty() {
                        self.phase.store(4, Ordering::Relaxed);
                        return self.react(ctx, resources).await;
                    }
                    Action::Take
                }

                // ── Phase 4: React — check for unanswered tool calls ──
                4 => self.react(ctx, resources).await,

                _ => Action::Done,
            }
        })
    }
}

impl Captain {
    /// React — scan for unanswered ToolCalls and execute them.
    async fn react<'a>(&'a self, ctx: &'a Context, resources: &'a Resources) -> Action {
        // ── should_halt check ──
        // After executing a tool, we loop back through Halt so the LLM
        // can see the result before deciding what to do next.
        if self.should_halt.swap(false, Ordering::Relaxed) {
            self.phase.store(3, Ordering::Relaxed);
            return Action::Halt;
        }

        // ── Scan context for unanswered ToolCalls ──
        let mut tc_entries: Vec<(&str, &str, &Value)> = Vec::new();
        let mut tr_ids: HashSet<&str> = HashSet::new();

        for frag in ctx.fragments() {
            match &frag.content {
                Content::ToolCall(tc) => {
                    tc_entries.push((tc.id.as_str(), tc.name.as_str(), &tc.arguments));
                }
                Content::ToolResult(tr) => {
                    tr_ids.insert(tr.call_id.as_str());
                }
                _ => {}
            }
        }

        let unanswered = tc_entries.iter().find(|(id, _, _)| !tr_ids.contains(id));

        match unanswered {
            None => {
                // No unanswered tool calls — conversation is stable.
                Action::Done
            }
            Some((call_id, name, args)) => {
                let active_tools = resources.active_tools();
                let tool = active_tools.iter().find(|t| t.name() == *name);

                match tool {
                    None => {
                        self.should_halt.store(true, Ordering::Relaxed);
                        Action::Append(Fragment::hitch(format!(
                            "tool '{}' not found in active tools",
                            name
                        )))
                    }
                    Some(tool) => match tool.execute((*args).clone()).await {
                        Ok(result) => {
                            self.should_halt.store(true, Ordering::Relaxed);
                            Action::Append(Fragment::tool_result(*call_id, result.content))
                        }
                        Err(msg) => {
                            self.should_halt.store(true, Ordering::Relaxed);
                            Action::Append(Fragment::hitch(format!(
                                "tool '{}' error: {}",
                                name, msg
                            )))
                        }
                    },
                }
            }
        }
    }
}
