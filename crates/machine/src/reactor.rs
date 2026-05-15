use crate::completion;
use crate::context::Context;
use crate::env::Environment;
use crate::fragment::{Content, Fragment};
use crate::inbox::Inbox;
use crate::resources::Resources;
use tokio::time::{Duration, timeout};

/// Call the LLM and execute ToolCalls. All fragments (original LLM text,
/// ToolCalls, and their ToolResults/Hitches) are pushed into the inbox.
/// The Policy drains them via [`Take`](crate::Action::Take).
pub async fn react(ctx: &Context, env: &Environment, resources: &Resources, inbox: &mut Inbox) {
    let fragments = completion::complete(ctx, resources).await;

    for frag in fragments {
        let mut result: Option<Fragment> = None;

        if let Content::ToolCall(tc) = &frag.content {
            result = Some(
                match resources
                    .active_tools()
                    .into_iter()
                    .find(|t| t.name() == tc.name)
                {
                    None => Fragment::hitch(format!("tool '{}' not found", tc.name)),
                    Some(tool) => {
                        let deadline = Duration::from_secs(tool.timeout().as_secs());
                        match timeout(deadline, tool.execute(tc.arguments.clone(), env)).await {
                            Ok(Ok(r)) => Fragment::tool_result(tc.id.clone(), r.content),
                            Ok(Err(msg)) => {
                                Fragment::hitch(format!("tool '{}' error: {}", tc.name, msg))
                            }
                            Err(_) => Fragment::hitch(format!(
                                "tool '{}' timed out after {}s",
                                tc.name,
                                tool.timeout().as_secs()
                            )),
                        }
                    }
                },
            );
        }

        inbox.push(frag);
        if let Some(f) = result {
            inbox.push(f);
        }
    }
}
