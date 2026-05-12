//! RIMA — Recursive Intelligence Machine Accelerator
//!
//! RIMA is a Turing-machine model for LLM agents. A machine (`Head`) operates
//! on a `Context` of `Fragment`s, driven by an `Engine` that decides what action
//! to take next.
//!
//! This crate only defines primitives: context cells, machine register, head,
//! actions, engine interface, channels, trace, and composition interfaces.
//! Concrete transition strategies, communication algorithms, and production
//! tools belong in downstream crates.

pub mod channel;
pub mod compose;
pub mod context;
pub mod engine;
pub mod fragment;
pub mod head;
pub mod register;
pub mod rica;
pub mod tool;
pub mod trace;

use std::time::Instant;

use serde_json::Value;

pub use channel::Channel;
pub use compose::{Ensemble, Parallel, Pipeline};
pub use context::Context;
pub use engine::{Action as EngineAction, Engine};
pub use fragment::{Content, Fragment, Role as FragmentRole, ToolCallDef};
pub use head::{HaltCondition, Head, LlmBackend, LlmResponse, MaxCycles, RigBackend};
pub use register::Register;
pub use rica::{Intent, Output, Rica};
pub use tool::Tool;
pub use trace::{Cycle as TraceCycle, TokenUsage, Trace};

/// Run a machine to completion.
///
/// This free function is the scheduler: it drives a `Head` over a mutable
/// `Context` and `Register` until the engine or a halt condition stops it.
pub async fn accelerate(
    head: &Head,
    intent: Intent,
    register: Register,
) -> (Output, Register, Trace) {
    let mut ctx = Context::new();
    let register = register;
    let mut trace = Trace::new();

    ctx.write(0, Fragment::system("You are a helpful assistant."));
    ctx.write(1, Fragment::user_text(&intent.prompt));

    loop {
        let action = head.engine.decide(&ctx, &register);

        match action {
            EngineAction::CallLlm => {
                let frags = ctx.fragments();
                let response = match head.llm.complete(&frags, &register.tools).await {
                    Ok(response) => response,
                    Err(error) => {
                        let pos = ctx.len();
                        ctx.write(pos, Fragment::assistant_text(format!("Error: {error}")));
                        trace.record(TraceCycle::Halt {
                            reason: format!("LLM error: {error}"),
                        });
                        break;
                    }
                };

                trace.record(TraceCycle::LlmCall {
                    tokens: response.tokens.clone(),
                });

                if response.tool_calls.is_empty() {
                    if let Some(text) = response.text {
                        let pos = ctx.len();
                        ctx.write(pos, Fragment::assistant_text(text));
                    }
                } else {
                    let pos = ctx.len();
                    ctx.write(
                        pos,
                        Fragment::assistant_tool_calls(response.tool_calls.clone()),
                    );

                    for call in &response.tool_calls {
                        let tool = register.tools.iter().find(|tool| tool.name == call.name);
                        let (output, duration_ms) = if let Some(tool) = tool {
                            let args: Value =
                                serde_json::from_str(&call.arguments).unwrap_or(Value::Null);
                            let start = Instant::now();
                            let result = (tool.run)(args, register.clone()).await;
                            (result, start.elapsed().as_millis() as u64)
                        } else {
                            (Err(format!("tool '{}' not found", call.name)), 0)
                        };

                        trace.record(TraceCycle::ToolCall {
                            tool: call.name.clone(),
                            input: serde_json::from_str(&call.arguments).unwrap_or(Value::Null),
                            output: output.clone(),
                            duration_ms,
                        });

                        let result_text = match &output {
                            Ok(value) => value.to_string(),
                            Err(error) => error.clone(),
                        };
                        let pos = ctx.len();
                        ctx.write(pos, Fragment::tool_result(&call.id, &result_text));
                    }
                }
            }
            EngineAction::Prune { from, to, reason } => {
                for pos in from..to {
                    ctx.erase(pos);
                }
                trace.record(TraceCycle::Prune { from, to, reason });
            }
            EngineAction::Compact { from, to } => {
                let frags = ctx.fragments();
                let range: Vec<&Fragment> = frags
                    .iter()
                    .skip(from)
                    .take(to.saturating_sub(from))
                    .copied()
                    .collect();

                let summary_text = head
                    .llm
                    .summarize(&range)
                    .await
                    .unwrap_or_else(|error| format!("[compaction failed: {error}]"));

                for pos in from..to {
                    ctx.erase(pos);
                }
                ctx.write(from, Fragment::system(summary_text));
                trace.record(TraceCycle::Compact { from, to });
            }
            EngineAction::Halt { reason } => {
                trace.record(TraceCycle::Halt { reason });
                break;
            }
        }

        if head
            .halt_conditions
            .iter()
            .any(|condition| condition.check(&ctx, &register, &trace))
        {
            break;
        }
    }

    let text = ctx
        .fragments()
        .iter()
        .rev()
        .find(|fragment| fragment.role == FragmentRole::Assistant)
        .and_then(|fragment| fragment.as_text())
        .unwrap_or("")
        .to_string();

    let output = Output {
        text,
        cycles: trace
            .cycles
            .iter()
            .filter(|cycle| !matches!(cycle, TraceCycle::Halt { .. }))
            .count() as u32,
        tokens: trace.total_tokens(),
    };

    (output, register, trace)
}
