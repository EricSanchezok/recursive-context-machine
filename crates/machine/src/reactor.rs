use std::time::Instant;

use crate::completion;
use crate::context::Context;
use crate::env::Environment;
use crate::fragment::{Content, Fragment};
use crate::hook;
use crate::inbox::Inbox;
use crate::resources::Resources;
use tokio::time::{Duration, timeout};
use tracing::{debug, info, warn};

/// Call the LLM and execute ToolCalls. All fragments (original LLM text,
/// ToolCalls, and their ToolResults/Hitches) are pushed into the inbox.
/// The Policy drains them via [`Take`](crate::Action::Take).
pub async fn react(ctx: &Context, env: &Environment, resources: &Resources, inbox: &mut Inbox) {
    let t0 = Instant::now();

    hook!(event = "completion_start");

    let fragments = completion::complete(ctx, resources).await;

    hook!(
        event = "completion_end",
        duration = %humantime(t0.elapsed()),
        fragments = fragments.len(),
    );

    for frag in fragments {
        let mut result: Option<Fragment> = None;

        if let Content::ToolCall(tc) = &frag.content {
            debug!(tool = tc.name, args = %tc.arguments, "tool call");

            hook!(
                event = "tool_call",
                call_id = tc.id,
                tool = tc.name,
                arguments = %tc.arguments,
            );

            result = Some(match resources.lookup(&tc.name) {
                None => Fragment::hitch(format!("tool '{}' not found", tc.name)),
                Some(tool) => {
                    let deadline = Duration::from_secs(tool.timeout().as_secs());
                    let t1 = Instant::now();
                    match timeout(deadline, tool.execute(tc.arguments.clone(), env)).await {
                        Ok(Ok(tool_result)) => {
                            info!(
                                tool = tc.name,
                                result = tool_result.content,
                                "tool executed"
                            );
                            hook!(
                                event = "tool_result",
                                call_id = tc.id,
                                tool = tc.name,
                                result = %tool_result.content,
                                duration = %humantime(t1.elapsed()),
                            );
                            Fragment::tool_result(
                                tc.id.clone(),
                                tool_result.content,
                                tool_result.title,
                            )
                        }
                        Ok(Err(msg)) => {
                            warn!(tool = tc.name, msg, "tool failed");
                            hook!(
                                event = "tool_error",
                                call_id = tc.id,
                                tool = tc.name,
                                error = %msg,
                                duration = %humantime(t1.elapsed()),
                            );
                            Fragment::hitch(format!("tool '{}' error: {}", tc.name, msg))
                        }
                        Err(_) => {
                            warn!(
                                tool = tc.name,
                                timeout = tool.timeout().as_secs(),
                                "tool timed out"
                            );
                            hook!(
                                event = "tool_error",
                                call_id = tc.id,
                                tool = tc.name,
                                error = "timeout",
                                timeout = tool.timeout().as_secs(),
                            );
                            Fragment::hitch(format!(
                                "tool '{}' timed out after {}s",
                                tc.name,
                                tool.timeout().as_secs()
                            ))
                        }
                    }
                }
            });
        }

        inbox.push(frag);
        if let Some(f) = result {
            inbox.push(f);
        }
    }
}

fn humantime(d: Duration) -> String {
    let ms = d.as_millis();
    if ms < 1000 {
        format!("{}ms", ms)
    } else {
        format!("{:.1}s", ms as f64 / 1000.0)
    }
}
