//! LLM completion driver.
//!
//! ## Contract
//!
//! [`send`] constructs a [`rig::completion::CompletionRequest`] directly from
//! the encoded fragments. The `chat_history` slot receives messages in the
//! exact order produced by [`encode`] — no rotation, no placeholder, no
//! "initial prompt".
//!
//! ### Why direct construction (vs. the builder)
//!
//! `rig`'s `CompletionRequestBuilder` treats one message as the "prompt" and
//! [appends it to the END of `chat_history`][build_appends_prompt] at
//! `build()` time. Picking which message is the prompt has been a persistent
//! source of bugs:
//!
//! - **#43 family**: using `Message::user(".")` as a stub prompt caused the
//!   LLM to interpret the bare period as a user reply, leading to
//!   `"looks like you sent a period"` hallucination loops.
//! - **#86 / #88**: switching the stub to `messages[0]` rotated the leading
//!   system message to the END of the request — `[Sys, User]` became
//!   `[User, Sys]` on the wire.
//!
//! `CompletionRequest` has no `prompt` field; `chat_history: OneOrMany<Message>`
//! is literally what gets serialized. [`OneOrMany::many`] errors on an empty
//! iterator, so the degenerate "empty context" case surfaces as a `Fragment::hitch`
//! Result rather than a fabricated message.
//!
//! [build_appends_prompt]: https://docs.rs/rig-core/0.36/rig/completion/struct.CompletionRequest.html

use http::{HeaderMap, HeaderName, HeaderValue};
use rig::OneOrMany;
use rig::client::CompletionClient;
use rig::completion::message::Reasoning;
use rig::completion::message::UserContent;
use rig::completion::{
    AssistantContent, CompletionError, CompletionModel, CompletionRequest, Message, ToolDefinition,
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
        .filter_map(|frag| encode(frag, model.thinking))
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
    let request = build_request(messages, tools, model)?;

    match timeout(
        Duration::from_secs(model.timeout),
        endpoint.completion(request),
    )
    .await
    {
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

/// Construct a `CompletionRequest` whose `chat_history` is the encoded
/// fragments **in their original order**.
///
/// Returns a `Fragment::hitch` when `messages` is empty — an empty context
/// has no meaningful prompt to send to the LLM, and we refuse to fabricate
/// one (see module-level contract).
pub fn build_request(
    messages: &[Message],
    tools: &[ToolDefinition],
    model: &Model,
) -> Result<CompletionRequest, Fragment> {
    let chat_history = OneOrMany::many(messages.iter().cloned()).map_err(|_| {
        Fragment::hitch(
            "completion called with empty context — no messages to send to LLM",
            None,
            Role::System,
            None::<&str>,
        )
    })?;

    Ok(CompletionRequest {
        model: None,
        preamble: None,
        chat_history,
        documents: Vec::new(),
        tools: tools.to_vec(),
        temperature: model.temperature,
        max_tokens: model.limit.as_ref().map(|l| l.output),
        tool_choice: None,
        additional_params: None,
        output_schema: None,
    })
}

pub fn decode<'a>(choice: impl Iterator<Item = &'a AssistantContent>) -> Vec<Fragment> {
    use rig::completion::message::MimeType;
    let mut fragments = Vec::new();
    // Thinking-mode providers (DeepSeek, Kimi) emit one reasoning block per
    // assistant turn and require it to be echoed back on every assistant
    // message we resend from that turn. A turn may contain multiple parallel
    // tool_calls — we replicate the same reasoning onto each of them, so the
    // turn survives being split into one Fragment per call.
    //
    // Buffer accumulates Reasoning blocks (rare multi-block case) but is
    // *not* cleared on ToolCall — only on Text or Image, which mark the
    // start of a new logical turn.
    let mut pending_reasoning: Vec<String> = Vec::new();
    for content in choice {
        match content {
            AssistantContent::Text(text) => {
                pending_reasoning.clear();
                fragments.push(Fragment::assistant(&text.text));
            }
            AssistantContent::Reasoning(reasoning) => {
                let text = reasoning.display_text();
                if !text.is_empty() {
                    pending_reasoning.push(text);
                }
            }
            AssistantContent::ToolCall(tc) => {
                let mut frag =
                    Fragment::tool_call(&tc.id, &tc.function.name, tc.function.arguments.clone());
                if !pending_reasoning.is_empty() {
                    frag = frag.with_reasoning(pending_reasoning.join("\n"));
                }
                fragments.push(frag);
            }
            AssistantContent::Image(content_image) => {
                pending_reasoning.clear();
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
        }
    }
    fragments
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
pub fn encode(frag: &Fragment, thinking: bool) -> Option<Message> {
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
                // Echo the model's original reasoning when present — thinking-mode
                // providers (DeepSeek, Kimi) validate that assistant tool-call
                // turns carry the same `reasoning_content` they previously emitted.
                // Falls back to a single-dot placeholder only when `thinking=true`
                // and no reasoning was captured (legacy Kimi path where the model
                // accepts any non-empty reasoning).
                if let Some(reasoning_text) = tc.reasoning.as_deref() {
                    content.push(AssistantContent::Reasoning(Reasoning::new(reasoning_text)));
                } else if thinking {
                    content.push(AssistantContent::Reasoning(Reasoning::new(".")));
                }
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
