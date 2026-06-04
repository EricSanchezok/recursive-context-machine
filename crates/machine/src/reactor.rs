use std::sync::Arc;
use std::time::Instant;

use crate::completion;
use crate::context::Context;
use crate::env::Environment;
use crate::fragment::{Content, Fragment, Role};
use crate::hook;
use crate::record::ActionOutcome;
use crate::resources::{LookupResult, Resources};
use crate::tool::ToolRuntime;
use futures_util::future::join_all;
use tokio::time::{Duration, timeout};
use tracing::{debug, info, warn};

pub async fn react(
    machine_id: &str,
    ctx: &Context,
    env: &Environment,
    resources: &Resources,
    tool_runtime: &ToolRuntime,
) -> ActionOutcome {
    let started_at = Instant::now();

    hook!(event = "completion_start", machine_id);

    let (fragments, usage) = completion::complete(ctx, resources).await;

    hook!(
        event = "completion_end",
        machine_id,
        duration = %humantime(started_at.elapsed()),
        fragments = fragments.len(),
        input_tokens = usage.input_tokens,
        output_tokens = usage.output_tokens,
        total_tokens = usage.total_tokens,
        cached_input_tokens = usage.cached_input_tokens,
        cache_creation_input_tokens = usage.cache_creation_input_tokens,
    );

    let mut output = Vec::new();
    let mut tool_call_ids: Vec<String> = Vec::new();
    let mut tool_names: Vec<String> = Vec::new();
    let mut tool_fragments: Vec<Fragment> = Vec::new();
    #[allow(clippy::type_complexity)]
    let mut tool_futures: Vec<
        std::pin::Pin<
            Box<
                dyn std::future::Future<Output = Result<(Fragment, Duration), (String, Duration)>>
                    + Send
                    + '_,
            >,
        >,
    > = Vec::new();

    for fragment in fragments {
        let tool_call = match fragment.content {
            Content::ToolCall(ref tool_call) => Some((
                tool_call.id.clone(),
                tool_call.name.clone(),
                tool_call.arguments.clone(),
            )),
            _ => None,
        };
        let Some((call_id, tool_name, arguments)) = tool_call else {
            output.push(fragment);
            continue;
        };

        debug!(tool = tool_name, args = %arguments, "tool call");

        hook!(
            event = "tool_call",
            machine_id,
            call_id,
            tool = tool_name,
            arguments = %arguments,
        );

        match resources.lookup(&tool_name) {
            LookupResult::NotFound => {
                let mut available: Vec<&str> =
                    resources.active_tools.iter().map(String::as_str).collect();
                available.sort_unstable();
                output.push(fragment);
                output.push(Fragment::hitch(
                    format!(
                        "tool '{}' not found. Available tools: {}",
                        tool_name,
                        available.join(", ")
                    ),
                    None,
                    Role::Tool,
                    Some(call_id),
                ));
            }
            LookupResult::Inactive => {
                output.push(fragment);
                output.push(Fragment::hitch(
                    format!("tool '{}' is disabled — activate it before use", tool_name),
                    None,
                    Role::Tool,
                    Some(call_id),
                ));
            }
            LookupResult::Active => {
                let Some(tool_arc) = tool_runtime.get_arc(&tool_name) else {
                    output.push(fragment);
                    output.push(Fragment::hitch(
                        format!("tool '{}' has no runtime executor", tool_name),
                        None,
                        Role::Tool,
                        Some(call_id),
                    ));
                    continue;
                };
                let deadline = Duration::from_secs(tool_arc.timeout().as_secs());
                let env_arc = Arc::new(env.clone());

                tool_call_ids.push(call_id.clone());
                tool_names.push(tool_name.clone());
                tool_futures.push(Box::pin(async move {
                    let started_at = Instant::now();
                    let handle = tokio::spawn(async move {
                        timeout(deadline, tool_arc.execute(arguments, &env_arc)).await
                    });
                    let result = handle.await;
                    let elapsed = started_at.elapsed();
                    match result {
                        Ok(Ok(Ok(tool_result))) => {
                            info!(
                                tool = tool_name,
                                result = tool_result.content,
                                "tool executed"
                            );
                            Ok((
                                Fragment::tool_result(
                                    call_id,
                                    tool_result.content,
                                    tool_result.title,
                                ),
                                elapsed,
                            ))
                        }
                        Ok(Ok(Err(message))) => {
                            warn!(tool = tool_name, message, "tool failed");
                            Err((format!("tool '{}' error: {}", tool_name, message), elapsed))
                        }
                        Ok(Err(_)) => {
                            warn!(
                                tool = tool_name,
                                timeout = deadline.as_secs(),
                                "tool timed out"
                            );
                            Err((
                                format!(
                                    "tool '{}' timed out after {}s",
                                    tool_name,
                                    deadline.as_secs()
                                ),
                                elapsed,
                            ))
                        }
                        Err(join_error) => {
                            let message = if join_error.is_panic() {
                                let payload = join_error.into_panic();
                                payload
                                    .downcast_ref::<&str>()
                                    .map(|panic_message| {
                                        format!("tool '{}' panicked: {}", tool_name, panic_message)
                                    })
                                    .or_else(|| {
                                        payload.downcast_ref::<String>().map(|panic_message| {
                                            format!(
                                                "tool '{}' panicked: {}",
                                                tool_name, panic_message
                                            )
                                        })
                                    })
                                    .unwrap_or_else(|| format!("tool '{}' panicked", tool_name))
                            } else {
                                format!("tool '{}' cancelled", tool_name)
                            };
                            warn!(message);
                            Err((message, elapsed))
                        }
                    }
                }));
                tool_fragments.push(fragment);
            }
        }
    }

    let results = join_all(tool_futures).await;

    for (((fragment, call_id), tool_name), result) in tool_fragments
        .into_iter()
        .zip(tool_call_ids)
        .zip(tool_names)
        .zip(results)
    {
        output.push(fragment);

        match result {
            Ok((result_fragment, duration)) => {
                hook!(
                    event = "tool_result",
                    machine_id,
                    call_id,
                    tool = tool_name,
                    result = %result_fragment.content_as_text(),
                    duration = %humantime(duration),
                );
                output.push(result_fragment);
            }
            Err((message, duration)) => {
                warn!(tool = tool_name, message, "tool failed");
                hook!(
                    event = "tool_error",
                    machine_id,
                    call_id,
                    tool = tool_name,
                    error = %message,
                    duration = %humantime(duration),
                );
                output.push(Fragment::hitch(message, None, Role::Tool, Some(call_id)));
            }
        }
    }

    ActionOutcome::Reactor {
        fragments: output,
        usage,
    }
}

fn humantime(duration: Duration) -> String {
    let milliseconds = duration.as_millis();
    if milliseconds < 1000 {
        format!("{}ms", milliseconds)
    } else {
        format!("{:.1}s", milliseconds as f64 / 1000.0)
    }
}
