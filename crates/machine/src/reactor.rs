use crate::completion;
use crate::context::Context;
use crate::env::Environment;
use crate::fragment::{Content, Fragment};
use crate::inbox::Inbox;
use crate::resources::Resources;
use tokio::time::{Duration, timeout};
use tracing::{debug, info, warn};

/// Call the LLM and execute ToolCalls. All fragments (original LLM text,
/// ToolCalls, and their ToolResults/Hitches) are pushed into the inbox.
/// The Policy drains them via [`Take`](crate::Action::Take).
pub async fn react(ctx: &Context, env: &Environment, resources: &Resources, inbox: &mut Inbox) {
    let fragments = completion::complete(ctx, resources).await;

    for frag in fragments {
        let mut result: Option<Fragment> = None;

        if let Content::ToolCall(tc) = &frag.content {
            debug!(tool = tc.name, args = %tc.arguments, "tool call");

            tracing::debug!(
                target: "hook",
                event = "tool_call",
                tool = tc.name,
                arguments = %tc.arguments,
            );

            result = Some(match resources.lookup(&tc.name) {
                None => Fragment::hitch(format!("tool '{}' not found", tc.name)),
                Some(tool) => {
                    let deadline = Duration::from_secs(tool.timeout().as_secs());
                    match timeout(deadline, tool.execute(tc.arguments.clone(), env)).await {
                        Ok(Ok(tool_result)) => {
                            info!(
                                tool = tc.name,
                                result = tool_result.content,
                                "tool executed"
                            );
                            tracing::debug!(
                                target: "hook",
                                event = "tool_result",
                                tool = tc.name,
                                result = %tool_result.content,
                            );
                            Fragment::tool_result(tc.id.clone(), tool_result.content)
                        }
                        Ok(Err(msg)) => {
                            warn!(tool = tc.name, msg, "tool failed");
                            tracing::debug!(
                                target: "hook",
                                event = "tool_error",
                                tool = tc.name,
                                error = %msg,
                            );
                            Fragment::hitch(format!("tool '{}' error: {}", tc.name, msg))
                        }
                        Err(_) => {
                            warn!(
                                tool = tc.name,
                                timeout = tool.timeout().as_secs(),
                                "tool timed out"
                            );
                            tracing::debug!(
                                target: "hook",
                                event = "tool_timeout",
                                tool = tc.name,
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
