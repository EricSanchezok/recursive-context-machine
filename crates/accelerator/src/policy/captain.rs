use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicU8, Ordering};

use machine::{Action, Content, Context, Environment, Fragment, Inbox, Policy, Resources, Role};
use serde_json::Value;

/// Captain — the default steering policy.
///
/// A finite-state machine with six explicit states.
///
/// ```text
/// Boot → Halt → Drain → React → (Digest → Halt → Drain → React)* → Done
/// ```
pub struct Captain {
    state: AtomicU8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum State {
    Boot = 1,
    Halt = 2,
    Drain = 3,
    React = 4,
    Digest = 5,
    Done = 6,
}

impl State {
    fn from_u8(v: u8) -> Self {
        match v {
            1 => Self::Boot,
            2 => Self::Halt,
            3 => Self::Drain,
            4 => Self::React,
            5 => Self::Digest,
            _ => Self::Done,
        }
    }
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
            state: AtomicU8::new(State::Boot as u8),
        }
    }

    fn load_state(&self) -> State {
        State::from_u8(self.state.load(Ordering::Relaxed))
    }

    fn store_state(&self, state: State) {
        self.state.store(state as u8, Ordering::Relaxed);
    }
}

impl Policy for Captain {
    fn decide<'a>(
        &'a self,
        ctx: &'a Context,
        env: &'a Environment,
        resources: &'a Resources,
        inbox: &'a Inbox,
    ) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>> {
        Box::pin(async move {
            let state = self.load_state();
            let (next, action) = match state {
                State::Boot => transition::boot(ctx, resources),
                State::Halt => transition::halt(),
                State::Drain => transition::drain(inbox),
                State::React => transition::react(ctx, env, resources).await,
                State::Digest => transition::digest(),
                State::Done => transition::done(),
            };
            self.store_state(next);
            action
        })
    }
}

// ── Transitions ──
//
// Each function maps (current inputs) → (next_state, action).
// No side effects, no mutable state — pure state transitions.

mod transition {
    use super::*;

    /// Boot — inject system prompt if the context lacks one.
    pub fn boot(ctx: &Context, resources: &Resources) -> (State, Action) {
        if ctx.fragments().iter().any(|f| f.role == Role::System) {
            return (State::Halt, Action::Halt);
        }
        let prompt = resources
            .prompts
            .get("default")
            .cloned()
            .unwrap_or_default();
        (State::Halt, Action::Append(Fragment::system(prompt)))
    }

    /// Halt — trigger LLM completion.
    pub fn halt() -> (State, Action) {
        (State::Drain, Action::Halt)
    }

    /// Drain — pop the inbox into context until empty.
    pub fn drain(inbox: &Inbox) -> (State, Action) {
        if inbox.is_empty() {
            // Inbox drained — advance to React. Take on an empty inbox
            // is a no-op, so this is effectively an internal transition.
            (State::React, Action::Take)
        } else {
            (State::Drain, Action::Take)
        }
    }

    /// React — scan for unanswered ToolCalls and execute the first one.
    pub async fn react(ctx: &Context, env: &Environment, resources: &Resources) -> (State, Action) {
        let unanswered = find_unanswered_tool_call(ctx);

        match unanswered {
            None => (State::Done, Action::Done),
            Some((call_id, name, args)) => {
                let tool = resources
                    .active_tools()
                    .into_iter()
                    .find(|t| t.name() == name);

                match tool {
                    None => (
                        State::Digest,
                        Action::Append(Fragment::hitch(format!(
                            "tool '{}' not found in active tools",
                            name
                        ))),
                    ),
                    Some(tool) => match tool.execute(args, env).await {
                        Ok(result) => (
                            State::Digest,
                            Action::Append(Fragment::tool_result(call_id, result.content)),
                        ),
                        Err(msg) => (
                            State::Digest,
                            Action::Append(Fragment::hitch(format!(
                                "tool '{}' error: {}",
                                name, msg
                            ))),
                        ),
                    },
                }
            }
        }
    }

    /// Digest — let the LLM absorb the tool result before deciding next steps.
    pub fn digest() -> (State, Action) {
        (State::Drain, Action::Halt)
    }

    /// Done — terminal state.
    pub fn done() -> (State, Action) {
        (State::Done, Action::Done)
    }

    // ── Helpers ──

    fn find_unanswered_tool_call(ctx: &Context) -> Option<(&str, &str, Value)> {
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

        tc_entries
            .iter()
            .find(|(id, _, _)| !tr_ids.contains(id))
            .map(|(id, name, args)| (*id, *name, (*args).clone()))
    }
}
