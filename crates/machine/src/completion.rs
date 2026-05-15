use rig::OneOrMany;
use rig::client::CompletionClient;
use rig::completion::{AssistantContent, CompletionModel, Message, ToolDefinition};
use tokio::time::{Duration, timeout};

use crate::context::Context;
use crate::fragment::{Content, Fragment, Role};
use crate::model::{Model, Protocol};
use crate::resources::Resources;

/// Call the active LLM and return the response fragments or an error.
///
/// Dispatches by `Protocol` (3 arms) to the corresponding rig module.
/// `endpoint` optionally overrides the provider's default base URL.
///
/// On failure, returns `Content::Hitch` so the caller (Policy) can
/// decide to retry, switch model, or abort.
pub async fn complete(ctx: &Context, resources: &Resources) -> Vec<Fragment> {
    let model = match resources.active_model() {
        Some(m) => m,
        None => return vec![],
    };

    let messages = prepare_messages(ctx);
    let tools = prepare_tools(resources);
    let api_key = model.credentials.as_deref().unwrap_or("");
    let endpoint = model.endpoint.as_deref();

    let result = match model.protocol {
        Protocol::OpenAI => {
            let mut b = rig::providers::openai::Client::builder().api_key(api_key);
            if let Some(ep) = endpoint {
                b = b.base_url(ep);
            }
            let provider = b
                .build()
                .expect("failed to build openai client")
                .completion_model(&model.name);
            request(&provider, model, &messages, &tools).await
        }
        Protocol::Anthropic => {
            let mut b = rig::providers::anthropic::Client::builder().api_key(api_key);
            if let Some(ep) = endpoint {
                b = b.base_url(ep);
            }
            let provider = b
                .build()
                .expect("failed to build anthropic client")
                .completion_model(&model.name);
            request(&provider, model, &messages, &tools).await
        }
        Protocol::Gemini => {
            let c = match endpoint {
                Some(ep) => rig::providers::gemini::Client::builder()
                    .api_key(api_key)
                    .base_url(ep)
                    .build()
                    .expect("failed to build gemini client"),
                None => rig::providers::gemini::Client::new(api_key)
                    .expect("failed to build gemini client"),
            };
            let provider = c.completion_model(&model.name);
            request(&provider, model, &messages, &tools).await
        }
    };

    match result {
        Ok(choice) => collect_fragments(choice.iter()),
        Err(hitch) => vec![hitch],
    }
}

// ── Internal ──

/// Dispatch a completion call with a deadline.
///
/// The call is cancelled if it exceeds `model.timeout` seconds.
async fn request<M: CompletionModel>(
    provider: &M,
    model: &Model,
    messages: &[Message],
    tools: &[ToolDefinition],
) -> Result<OneOrMany<AssistantContent>, Fragment> {
    let mut call = provider
        .completion_request(Message::user(""))
        .messages(messages.to_vec())
        .tools(tools.to_vec());

    if let Some(temp) = model.temperature {
        call = call.temperature(temp);
    }
    if let Some(limit) = &model.limit {
        call = call.max_tokens(limit.output);
    }

    let deadline = Duration::from_secs(model.timeout);

    match timeout(deadline, call.send()).await {
        Ok(Ok(response)) => Ok(response.choice),
        Ok(Err(e)) => Err(Fragment::hitch(format!("{}", e))),
        Err(_) => Err(Fragment::hitch(format!(
            "request timed out after {}s",
            model.timeout
        ))),
    }
}

/// Extract fragments from a rig response's `AssistantContent` items.
fn collect_fragments<'a>(choice: impl Iterator<Item = &'a AssistantContent>) -> Vec<Fragment> {
    let mut fragments = Vec::new();
    for content in choice {
        match content {
            AssistantContent::Text(text) => {
                fragments.push(Fragment::assistant(&text.text));
            }
            AssistantContent::ToolCall(tc) => {
                fragments.push(Fragment::tool_call(
                    &tc.id,
                    &tc.function.name,
                    tc.function.arguments.clone(),
                ));
            }
            _ => {}
        }
    }
    fragments
}

/// Convert context fragments into rig messages.
fn prepare_messages(ctx: &Context) -> Vec<Message> {
    ctx.fragments()
        .iter()
        .filter_map(fragment_to_message)
        .collect()
}

/// Convert active tools into rig tool definitions.
fn prepare_tools(resources: &Resources) -> Vec<ToolDefinition> {
    resources
        .active_tools()
        .iter()
        .map(|t| ToolDefinition {
            name: t.name().to_string(),
            description: t.description().to_string(),
            parameters: t.parameters(),
        })
        .collect()
}

/// Convert a context fragment into a rig message.
fn fragment_to_message(frag: &Fragment) -> Option<Message> {
    match frag.role {
        Role::System => frag.as_text().map(Message::system),
        Role::User => frag.as_text().map(Message::user),
        Role::Assistant => {
            if let Content::ToolCall(tc) = &frag.content {
                Some(Message::Assistant {
                    id: None,
                    content: OneOrMany::one(AssistantContent::tool_call(
                        &tc.id,
                        &tc.name,
                        tc.arguments.clone(),
                    )),
                })
            } else {
                frag.as_text().map(Message::assistant)
            }
        }
        Role::Tool => {
            if let Content::ToolResult(tr) = &frag.content {
                Some(Message::tool_result(&tr.call_id, &tr.content))
            } else {
                None
            }
        }
    }
}
