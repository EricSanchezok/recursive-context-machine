use http::{HeaderMap, HeaderName, HeaderValue};
use rig::OneOrMany;
use rig::client::CompletionClient;
use rig::completion::message::Reasoning;
use rig::completion::message::UserContent;
use rig::completion::{
    AssistantContent, CompletionError, CompletionModel, Message, ToolDefinition,
};
use rig::http_client;
use tokio::time::{Duration, timeout};

use crate::context::Context;
use crate::fragment::{Content, Fragment, Role};
use crate::model::{Model, Protocol};
use crate::resources::Resources;
use crate::usage::Usage;
use tracing::{debug, warn};

/// Call the active LLM and return the response fragments or an error.
pub async fn complete(ctx: &Context, resources: &Resources) -> (Vec<Fragment>, Usage) {
    let Some(model) = resources.active_model() else {
        warn!("completion requested but no active model is set");
        let hitch = Fragment::hitch(
            "no active model set; register and activate a model before completing",
            None,
            Role::System,
            None::<&str>,
        );
        return (
            vec![hitch],
            Usage {
                input_tokens: 0,
                output_tokens: 0,
                total_tokens: 0,
                cached_input_tokens: 0,
                cache_creation_input_tokens: 0,
                fragment_ids: Vec::new(),
            },
        );
    };

    let messages: Vec<Message> = ctx
        .fragments()
        .iter()
        .filter_map(|frag| encode(frag))
        .collect();

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
            builder = apply_headers(builder, &model.headers);
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
            builder = apply_headers(builder, &model.headers);
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
            builder = apply_headers(builder, &model.headers);
            let endpoint = builder
                .build()
                .expect("failed to build gemini client")
                .completion_model(&model.name);
            send(&endpoint, model, &messages, &tools).await
        }
    };

    match result {
        Ok((choice, usage)) => {
            let text_fragments = choice
                .iter()
                .filter(|c| matches!(c, AssistantContent::Text(_)))
                .count();
            let tool_calls = choice
                .iter()
                .filter(|c| matches!(c, AssistantContent::ToolCall(_)))
                .count();
            debug!(text_fragments, tool_calls, "completion response");
            (decode(choice.iter()), usage)
        }
        Err(hitch) => {
            warn!(?hitch, "completion failed");
            (
                vec![hitch],
                Usage {
                    input_tokens: 0,
                    output_tokens: 0,
                    total_tokens: 0,
                    cached_input_tokens: 0,
                    cache_creation_input_tokens: 0,
                    fragment_ids: Vec::new(),
                },
            )
        }
    }
}

async fn send(
    endpoint: &impl CompletionModel,
    model: &Model,
    messages: &[Message],
    tools: &[ToolDefinition],
) -> Result<(OneOrMany<AssistantContent>, Usage), Fragment> {
    // Use the first context message as the initial prompt for rig's builder API.
    // When messages is empty (should never happen in practice), use a system
    // placeholder — system role messages carry no conversational intent and
    // won't trigger spurious LLM responses like "user sent a period".
    let (initial_prompt, remaining_messages) = split_messages(messages);

    let mut request = endpoint
        .completion_request(initial_prompt)
        .messages(remaining_messages)
        .tools(tools.to_vec());

    if let Some(temp) = model.temperature {
        request = request.temperature(temp);
    }
    if let Some(limit) = &model.limit {
        request = request.max_tokens(limit.output);
    }

    match timeout(Duration::from_secs(model.timeout), request.send()).await {
        Ok(Ok(response)) => Ok((
            response.choice,
            Usage {
                input_tokens: response.usage.input_tokens,
                output_tokens: response.usage.output_tokens,
                total_tokens: response.usage.total_tokens,
                cached_input_tokens: response.usage.cached_input_tokens,
                cache_creation_input_tokens: response.usage.cache_creation_input_tokens,
                fragment_ids: Vec::new(),
            },
        )),
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

fn decode<'a>(choice: impl Iterator<Item = &'a AssistantContent>) -> Vec<Fragment> {
    use rig::completion::message::MimeType;
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
            AssistantContent::Image(content_image) => {
                let source = match &content_image.data {
                    rig::completion::message::DocumentSourceKind::Url(url) => {
                        crate::fragment::DataSource::Url(url.clone())
                    }
                    rig::completion::message::DocumentSourceKind::Base64(data) => {
                        crate::fragment::DataSource::Base64(data.clone())
                    }
                    _ => {
                        warn!(
                            "unrecognized DocumentSourceKind in Image, falling back to empty data"
                        );
                        crate::fragment::DataSource::String(String::new())
                    }
                };
                let media_type = content_image
                    .media_type
                    .as_ref()
                    .map(|m| m.to_mime_type().to_string());
                fragments.push(Fragment::image(source, media_type));
            }
            _ => {} // non-exhaustive: other AssistantContent variants
        }
    }
    fragments
}

/// Split messages into (initial_prompt, remaining). The first message
/// becomes the initial prompt; the rest are returned for `.messages()`.
/// When the list is empty, returns (Message::system("_"), Vec::new()).
fn split_messages(messages: &[Message]) -> (Message, Vec<Message>) {
    let initial = messages
        .first()
        .cloned()
        .unwrap_or_else(|| Message::system("_"));
    let remaining = messages.get(1..).unwrap_or_default().to_vec();
    (initial, remaining)
}

fn apply_headers<Ext, ApiKey, H>(
    mut builder: rig::client::ClientBuilder<Ext, ApiKey, H>,
    headers: &Option<std::collections::HashMap<String, String>>,
) -> rig::client::ClientBuilder<Ext, ApiKey, H>
where
    Ext: Clone,
{
    if let Some(headers_map) = headers {
        let mut header_map = HeaderMap::new();
        for (key, value) in headers_map {
            if let (Ok(name), Ok(val)) = (
                HeaderName::from_bytes(key.as_bytes()),
                HeaderValue::from_str(value),
            ) {
                header_map.insert(name, val);
            }
        }
        if !header_map.is_empty() {
            builder = builder.http_headers(header_map);
        }
    }
    builder
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_messages_uses_first_message_as_initial() {
        let msgs = vec![
            Message::system("you are a helper"),
            Message::user("search for papers"),
            Message::assistant("let me check"),
        ];
        let (initial, remaining) = split_messages(&msgs);
        assert!(matches!(&initial, Message::System { content } if content == "you are a helper"));
        assert_eq!(remaining.len(), 2);
        assert!(matches!(&remaining[0], Message::User { .. }));
        assert!(matches!(&remaining[1], Message::Assistant { .. }));
    }

    #[test]
    fn split_messages_single_message_no_remaining() {
        let msgs = vec![Message::user("hello")];
        let (initial, remaining) = split_messages(&msgs);
        assert!(matches!(&initial, Message::User { .. }));
        assert!(remaining.is_empty());
    }

    #[test]
    fn split_messages_empty_falls_back_to_system_placeholder() {
        let msgs: Vec<Message> = vec![];
        let (initial, remaining) = split_messages(&msgs);
        assert!(matches!(&initial, Message::System { content } if content == "_"));
        assert!(remaining.is_empty());
    }
}

/// Resolve a `fragment::DataSource` into a rig `UserContent`, converting
/// `Raw` bytes to Base64.
fn encode_user_content(
    kind: &str,
    source: &crate::fragment::DataSource,
    media_type: &Option<String>,
) -> UserContent {
    use crate::fragment::DataSource;
    use base64::Engine as _;
    use rig::completion::message::MimeType;
    use rig::completion::message::{AudioMediaType, DocumentMediaType, ImageMediaType};

    let image_mime = media_type
        .as_deref()
        .and_then(ImageMediaType::from_mime_type);
    let audio_mime = media_type
        .as_deref()
        .and_then(AudioMediaType::from_mime_type);
    let doc_mime = media_type
        .as_deref()
        .and_then(DocumentMediaType::from_mime_type);

    match source {
        DataSource::Url(url) => UserContent::image_url(url.clone(), image_mime, None),
        DataSource::Base64(data) | DataSource::String(data) => match kind {
            "audio" => UserContent::audio(data.clone(), audio_mime),
            "document" => UserContent::document(data.clone(), doc_mime),
            _ => UserContent::image_base64(data.clone(), image_mime, None),
        },
        DataSource::Raw(bytes) => {
            let b64 = base64::engine::general_purpose::STANDARD.encode(bytes);
            match kind {
                "audio" => UserContent::audio(b64, audio_mime),
                _ => UserContent::image_base64(b64, image_mime, None),
            }
        }
    }
}

/// Encode a context fragment into a rig message.
///
/// Assistant tool-call messages always include a Reasoning placeholder so
/// providers that enable thinking mode (DeepSeek V4, Kimi Coding) receive
/// the required `reasoning_content` on tool-call turns. Providers that do
/// not use thinking ignore the field.
pub fn encode(frag: &Fragment) -> Option<Message> {
    match frag.role {
        Role::System => {
            if let Content::Hitch { message, .. } = &frag.content {
                Some(Message::system(message))
            } else {
                frag.as_text().map(Message::system)
            }
        }
        Role::User => match &frag.content {
            Content::Image(img) => Some(Message::User {
                content: OneOrMany::one(encode_user_content("image", &img.source, &img.media_type)),
            }),
            Content::Audio(audio) => Some(Message::User {
                content: OneOrMany::one(encode_user_content(
                    "audio",
                    &audio.source,
                    &audio.media_type,
                )),
            }),
            Content::Video(video) => Some(Message::User {
                content: OneOrMany::one(encode_user_content(
                    "video",
                    &video.source,
                    &video.media_type,
                )),
            }),
            Content::Document(document) => Some(Message::User {
                content: OneOrMany::one(encode_user_content(
                    "document",
                    &document.source,
                    &document.media_type,
                )),
            }),
            _ => frag.as_text().map(Message::user),
        },
        Role::Assistant => {
            if let Content::ToolCall(tc) = &frag.content {
                let mut content = OneOrMany::one(AssistantContent::tool_call(
                    &tc.id,
                    &tc.name,
                    tc.arguments.clone(),
                ));
                // Always attach a Reasoning placeholder to assistant tool-call
                // messages. Providers that enable thinking mode (DeepSeek V4,
                // Kimi Coding) require reasoning_content to be present on
                // tool-call turns; providers that don't use it will ignore the
                // field.
                content.push(AssistantContent::Reasoning(Reasoning::new(".")));
                Some(Message::Assistant { id: None, content })
            } else if let Content::Hitch { message, .. } = &frag.content {
                Some(Message::assistant(message))
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
