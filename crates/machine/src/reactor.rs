use crate::completion;
use crate::context::Context;
use crate::env::Environment;
use crate::fragment::{Content, Fragment};
use crate::inbox::Inbox;
use crate::resources::Resources;
use tokio::time::{Duration, timeout};

/// Complete the LLM call and execute all ToolCalls. Returns `true` if any
/// tool was executed, signalling the machine to re-invoke the reactor.
pub async fn react(
    ctx: &Context,
    env: &Environment,
    resources: &Resources,
    inbox: &mut Inbox,
) -> bool {
    let fragments = completion::complete(ctx, resources).await;
    let mut executed = false;

    for frag in fragments {
        if let Content::ToolCall(tc) = &frag.content {
            executed = true;
            let tc_id = tc.id.clone();
            let tc_name = tc.name.clone();
            let tc_args = tc.arguments.clone();

            inbox.push(frag);

            let tool = resources
                .active_tools()
                .into_iter()
                .find(|t| t.name() == tc_name);

            match tool {
                None => {
                    inbox.push(Fragment::hitch(format!("tool '{}' not found", tc_name)));
                }
                Some(tool) => {
                    let deadline = Duration::from_secs(tool.timeout().as_secs());
                    match timeout(deadline, tool.execute(tc_args, env)).await {
                        Ok(Ok(result)) => {
                            inbox.push(Fragment::tool_result(tc_id, result.content));
                        }
                        Ok(Err(msg)) => {
                            inbox.push(Fragment::hitch(format!(
                                "tool '{}' error: {}",
                                tc_name, msg
                            )));
                        }
                        Err(_) => {
                            inbox.push(Fragment::hitch(format!(
                                "tool '{}' timed out after {}s",
                                tc_name,
                                tool.timeout().as_secs()
                            )));
                        }
                    }
                }
            }
        } else {
            inbox.push(frag);
        }
    }

    executed
}
