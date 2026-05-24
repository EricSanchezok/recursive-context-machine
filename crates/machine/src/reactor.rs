use std::time::Instant;

use crate::completion;
use crate::context::Context;
use crate::env::Environment;
use crate::fragment::{Content, Fragment, Role};
use crate::hook;
use crate::inbox::Inbox;
use crate::resources::Resources;
use crate::usage::Usage;
use tokio::time::{Duration, timeout};
use tracing::{debug, info, warn};

pub async fn react(
    machine_id: &str,
    ctx: &Context,
    env: &Environment,
    resources: &Resources,
    inbox: &mut Inbox,
) -> Usage {
    let t0 = Instant::now();

    hook!(event = "completion_start", machine_id);

    let (fragments, usage) = completion::complete(ctx, resources).await;

    hook!(
        event = "completion_end",
        machine_id,
        duration = %humantime(t0.elapsed()),
        fragments = fragments.len(),
        input_tokens = usage.input_tokens,
        output_tokens = usage.output_tokens,
        total_tokens = usage.total_tokens,
        cached_input_tokens = usage.cached_input_tokens,
        cache_creation_input_tokens = usage.cache_creation_input_tokens,
    );

    for frag in fragments {
        let mut result: Option<Fragment> = None;

        if let Content::ToolCall(tc) = &frag.content {
            debug!(tool = tc.name, args = %tc.arguments, "tool call");

            hook!(
                event = "tool_call",
                machine_id,
                call_id = tc.id,
                tool = tc.name,
                arguments = %tc.arguments,
            );

            result = Some(match resources.lookup(&tc.name) {
                None => Fragment::hitch(
                    format!("tool '{}' not found", tc.name),
                    None,
                    Role::Tool,
                    Some(tc.id.clone()),
                ),
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
                                machine_id,
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
                                machine_id,
                                call_id = tc.id,
                                tool = tc.name,
                                error = %msg,
                                duration = %humantime(t1.elapsed()),
                            );
                            Fragment::hitch(
                                format!("tool '{}' error: {}", tc.name, msg),
                                None,
                                Role::Tool,
                                Some(tc.id.clone()),
                            )
                        }
                        Err(_) => {
                            warn!(
                                tool = tc.name,
                                timeout = tool.timeout().as_secs(),
                                "tool timed out"
                            );
                            hook!(
                                event = "tool_error",
                                machine_id,
                                call_id = tc.id,
                                tool = tc.name,
                                error = "timeout",
                                timeout = tool.timeout().as_secs(),
                            );
                            Fragment::hitch(
                                format!(
                                    "tool '{}' timed out after {}s",
                                    tc.name,
                                    tool.timeout().as_secs()
                                ),
                                None,
                                Role::Tool,
                                Some(tc.id.clone()),
                            )
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

    usage
}

fn humantime(d: Duration) -> String {
    let ms = d.as_millis();
    if ms < 1000 {
        format!("{}ms", ms)
    } else {
        format!("{:.1}s", ms as f64 / 1000.0)
    }
}
