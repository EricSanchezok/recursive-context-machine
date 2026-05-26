use std::sync::Arc;
use std::time::Instant;

use crate::completion;
use crate::context::Context;
use crate::env::Environment;
use crate::fragment::{Content, Fragment, Role};
use crate::hook;
use crate::inbox::Inbox;
use crate::resources::Resources;
use crate::usage::Usage;
use futures_util::future::join_all;
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

    // ── Phase 1: partition ──
    // Text fragments → inbox immediately.
    // Tool calls → collect call_id + tool_name + fragment, spawn future.
    let mut tool_call_ids: Vec<String> = Vec::new();
    let mut tool_names: Vec<String> = Vec::new();
    let mut tool_fragments: Vec<Fragment> = Vec::new();
    let mut tool_futures: Vec<
        std::pin::Pin<
            Box<
                dyn std::future::Future<Output = Result<(Fragment, Duration), (String, Duration)>>
                    + Send
                    + '_,
            >,
        >,
    > = Vec::new();

    for frag in fragments {
        let tc_meta = match frag.content {
            Content::ToolCall(ref tc) => {
                Some((tc.id.clone(), tc.name.clone(), tc.arguments.clone()))
            }
            _ => None,
        };
        let Some((call_id, tool_name, args)) = tc_meta else {
            inbox.push(frag);
            continue;
        };

        debug!(tool = tool_name, args = %args, "tool call");

        hook!(
            event = "tool_call",
            machine_id,
            call_id,
            tool = tool_name,
            arguments = %args,
        );

        match resources.lookup(&tool_name) {
            None => {
                inbox.push(frag);
                inbox.push(Fragment::hitch(
                    format!("tool '{}' not found", tool_name),
                    None,
                    Role::Tool,
                    Some(call_id),
                ));
            }
            Some(tool) => {
                let deadline = Duration::from_secs(tool.timeout().as_secs());
                let tool_arc = resources
                    .tools
                    .get(&tool_name)
                    .expect("tool just confirmed by lookup")
                    .clone();
                let env_arc = Arc::new(env.clone());

                tool_call_ids.push(call_id.clone());
                tool_names.push(tool_name.clone());
                tool_futures.push(Box::pin(async move {
                    let started_at = Instant::now();
                    let handle = tokio::spawn(async move {
                        timeout(deadline, tool_arc.execute(args, &env_arc)).await
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
                        Ok(Ok(Err(msg))) => {
                            warn!(tool = tool_name, msg, "tool failed");
                            Err((format!("tool '{}' error: {}", tool_name, msg), elapsed))
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
                        Err(join_err) => {
                            let msg = if join_err.is_panic() {
                                let payload = join_err.into_panic();
                                payload
                                    .downcast_ref::<&str>()
                                    .map(|s| format!("tool '{}' panicked: {}", tool_name, s))
                                    .or_else(|| {
                                        payload.downcast_ref::<String>().map(|s| {
                                            format!("tool '{}' panicked: {}", tool_name, s)
                                        })
                                    })
                                    .unwrap_or_else(|| format!("tool '{}' panicked", tool_name))
                            } else {
                                format!("tool '{}' cancelled", tool_name)
                            };
                            warn!("{}", msg);
                            Err((msg, elapsed))
                        }
                    }
                }));
                tool_fragments.push(frag);
            }
        }
    }

    // ── Phase 2: execute all tool calls concurrently ──
    let results = join_all(tool_futures).await;

    // ── Phase 3: push results, indexed by position ──
    for (((frag, call_id), tool_name), result) in tool_fragments
        .into_iter()
        .zip(tool_call_ids)
        .zip(tool_names)
        .zip(results)
    {
        inbox.push(frag);

        match result {
            Ok((result_frag, duration)) => {
                hook!(
                    event = "tool_result",
                    machine_id,
                    call_id,
                    tool = tool_name,
                    result = %result_frag.content_as_text(),
                    duration = %humantime(duration),
                );
                inbox.push(result_frag);
            }
            Err((msg, duration)) => {
                warn!(tool = tool_name, msg, "tool failed");
                hook!(
                    event = "tool_error",
                    machine_id,
                    call_id,
                    tool = tool_name,
                    error = %msg,
                    duration = %humantime(duration),
                );
                inbox.push(Fragment::hitch(msg, None, Role::Tool, Some(call_id)));
            }
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
