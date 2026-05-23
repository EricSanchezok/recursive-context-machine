use rig::OneOrMany;
use rig::client::CompletionClient;
use rig::completion::{
    AssistantContent, CompletionError, CompletionModel, Message, ToolDefinition,
};
use rig::http_client;
use tokio::time::{Duration, timeout};

use crate::context::Context;
use crate::fragment::{Content, Fragment, Role};
use crate::model::{Model, Protocol};
use crate::resources::Resources;
use tracing::{debug, warn};

/// Call the active LLM and return the response fragments or an error.
///
/// Dispatches by `Protocol` (3 arms) to the corresponding rig module.
/// `endpoint` optionally overrides the provider's default base URL.
///
/// On failure, returns `Content::Hitch` so the caller (Policy) can
/// decide to retry, switch model, or abort.
pub async fn complete(ctx: &Context, resources: &Resources) -> Vec<Fragment> {
    let model = resources.active_model();

    let messages: Vec<Message> = ctx.fragments().iter().filter_map(encode).collect();

    let tools: Vec<ToolDefinition> = resources
        .active_tools()
        .iter()
        .map(|t| ToolDefinition {
            name: t.name().to_string(),
            description: t.description().to_string(),
            parameters: t.parameters(),
        })
        .collect();

    debug!(
        model = %model.name,
        messages = messages.len(),
        tools = tools.len(),
        "completion request"
    );

    let api_key = model.credentials.as_deref().unwrap_or("");
    let endpoint_url = model.endpoint.as_deref();

    let result = match model.protocol {
        Protocol::OpenAI => {
            let mut builder = rig::providers::openai::CompletionsClient::builder().api_key(api_key);
            if let Some(endpoint) = endpoint_url {
                builder = builder.base_url(endpoint);
            }
            let endpoint = builder
                .build()
                .expect("failed to build openai client")
                .completion_model(&model.name);
            send(&endpoint, model, &messages, &tools).await
        }
        Protocol::Anthropic => {
            let mut builder = rig::providers::anthropic::Client::builder().api_key(api_key);
            if let Some(endpoint) = endpoint_url {
                builder = builder.base_url(endpoint);
            }
            let endpoint = builder
                .build()
                .expect("failed to build anthropic client")
                .completion_model(&model.name);
            send(&endpoint, model, &messages, &tools).await
        }
        Protocol::Gemini => {
            let mut builder = rig::providers::gemini::Client::builder().api_key(api_key);
            if let Some(endpoint) = endpoint_url {
                builder = builder.base_url(endpoint);
            }
            let endpoint = builder
                .build()
                .expect("failed to build gemini client")
                .completion_model(&model.name);
            send(&endpoint, model, &messages, &tools).await
        }
    };

    match result {
        Ok(choice) => {
            let text_fragments = choice
                .iter()
                .filter(|c| matches!(c, AssistantContent::Text(_)))
                .count();
            let tool_calls = choice
                .iter()
                .filter(|c| matches!(c, AssistantContent::ToolCall(_)))
                .count();
            debug!(text_fragments, tool_calls, "completion response");
            decode(choice.iter())
        }
        Err(hitch) => {
            warn!(?hitch, "completion failed");
            vec![hitch]
        }
    }
}

/// Send a request with a deadline. Returns an error if exceeding `model.timeout` seconds.
async fn send(
    endpoint: &impl CompletionModel,
    model: &Model,
    messages: &[Message],
    tools: &[ToolDefinition],
) -> Result<OneOrMany<AssistantContent>, Fragment> {
    let mut request = endpoint
        .completion_request(Message::user(""))
        .messages(messages.to_vec())
        .tools(tools.to_vec());

    if let Some(temp) = model.temperature {
        request = request.temperature(temp);
    }
    if let Some(limit) = &model.limit {
        request = request.max_tokens(limit.output);
    }

    match timeout(Duration::from_secs(model.timeout), request.send()).await {
        Ok(Ok(response)) => Ok(response.choice),
        Ok(Err(error)) => {
            let code = match &error {
                CompletionError::HttpError(http_client::Error::InvalidStatusCode(s)) => {
                    Some(s.as_u16())
                }
                CompletionError::HttpError(http_client::Error::InvalidStatusCodeWithMessage(
                    s,
                    _,
                )) => Some(s.as_u16()),
                _ => None,
            };
            Err(Fragment::hitch(
                error.to_string(),
                code,
                Role::Assistant,
                None::<&str>,
            ))
        }
        Err(_) => Err(Fragment::hitch(
            format!("request timed out after {}s", model.timeout),
            None,
            Role::Assistant,
            None::<&str>,
        )),
    }
}

/// Decode rig assistant content into fragments.
fn decode<'a>(choice: impl Iterator<Item = &'a AssistantContent>) -> Vec<Fragment> {
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

/// Encode a context fragment into a rig message.
fn encode(frag: &Fragment) -> Option<Message> {
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
            } else if let Content::Hitch {
                message,
                call_id: Some(call_id),
                ..
            } = &frag.content
            {
                Some(Message::tool_result(call_id, message))
            } else {
                None
            }
        }
    }
}
