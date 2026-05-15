use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};

use machine::{Action, Content, Context, Environment, Fragment, Inbox, Policy, Resources};
use serde_json::Value;

/// Default policy — a 6-phase state machine that injects system/user
/// prompts, drains the inbox, and executes tool calls in a loop.
///
/// ```text
/// Phase 1 (Boot sys):   Append(system prompt from resources) → phase 2
/// Phase 2 (Boot user):  Append(user intent) → phase 3
/// Phase 3 (Halt):       Halt → phase 4
/// Phase 4 (Drain):      inbox empty → phase 5; else Take (stay)
/// Phase 5 (Check):      Scan for unanswered ToolCalls:
///                         - should_halt set → clear, Halt → phase 4
///                         - unanswered TC → execute, Append, set should_halt
///                         - none → Done
/// ```
pub struct DefaultPolicy {
    phase: AtomicU8,
    should_halt: AtomicBool,
    intent: String,
}

impl DefaultPolicy {
    /// Create a new default policy with the given user intent.
    pub fn new(intent: impl Into<String>) -> Self {
        Self {
            phase: AtomicU8::new(1),
            should_halt: AtomicBool::new(false),
            intent: intent.into(),
        }
    }
}

impl Policy for DefaultPolicy {
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
                // ── Phase 1: Boot sys ──
                1 => {
                    let prompt = resources
                        .prompts
                        .get("default")
                        .cloned()
                        .unwrap_or_default();
                    self.phase.store(2, Ordering::Relaxed);
                    Action::Append(Fragment::system(prompt))
                }

                // ── Phase 2: Boot user ──
                2 => {
                    self.phase.store(3, Ordering::Relaxed);
                    Action::Append(Fragment::user(self.intent.clone()))
                }

                // ── Phase 3: Halt ──
                3 => {
                    self.phase.store(4, Ordering::Relaxed);
                    Action::Halt
                }

                // ── Phase 4: Drain inbox ──
                4 => {
                    if inbox.is_empty() {
                        self.phase.store(5, Ordering::Relaxed);
                        // Recurse: re-evaluate at phase 5 in this same step.
                        return self.evaluate_phase_5(ctx, resources).await;
                    }
                    Action::Take
                }

                // ── Phase 5: Check context for unanswered tool calls ──
                5 => self.evaluate_phase_5(ctx, resources).await,

                // ── Phase 6: Halt after exec (entry via phase 5 should_halt) ──
                6 => {
                    self.phase.store(4, Ordering::Relaxed);
                    Action::Halt
                }

                _ => Action::Done,
            }
        })
    }
}

impl DefaultPolicy {
    /// Evaluate phase 5: scan for unanswered tool calls, handle should_halt.
    async fn evaluate_phase_5<'a>(&'a self, ctx: &'a Context, resources: &'a Resources) -> Action {
        // ── should_halt check ──
        if self.should_halt.swap(false, Ordering::Relaxed) {
            // Transition through phase 6: Halt then go to phase 4.
            self.phase.store(4, Ordering::Relaxed);
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
                // No unanswered tool calls — we're done.
                Action::Done
            }
            Some((call_id, name, args)) => {
                // Find and execute the tool.
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
