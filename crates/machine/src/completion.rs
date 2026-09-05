//! LLM completion driver.
//!
//! ## Contract
//!
//! [`send`] constructs a [`rig::completion::CompletionRequest`] directly from
//! the encoded fragments. The `chat_history` slot preserves their order with
//! no rotation, placeholder, or fabricated "initial prompt". Adjacent system
//! fragments are joined into one system message because OpenAI-compatible
//! gateways commonly optimize or validate a single leading system turn.
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
    AssistantContent, CompletionError, CompletionModel, CompletionRequest, Message,
    ToolDefinition as RigToolDefinition,
};
use rig::http_client;
use tokio::time::{Duration, timeout};

use crate::context::Context;
use crate::event::{CompletionDiagnostics, ProviderFailureDiagnostics, RequestShapeDiagnostics};
use crate::fragment::{Content, Fragment, Role};
use crate::model::{Model, Protocol};
use crate::resources::Resources;
use crate::usage::TokenUsage;
use tracing::{debug, warn};

const ESTIMATED_CHARS_PER_TOKEN: u64 = 4;
const MAX_PROVIDER_IDENTIFIER_BYTES: usize = 128;

struct CompletionFailure {
    hitch: Fragment,
    provider: ProviderFailureDiagnostics,
}

#[derive(Default)]
struct CountingWriter {
    bytes: u64,
}

impl std::io::Write for CountingWriter {
    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        self.bytes = self
            .bytes
            .saturating_add(u64::try_from(buffer.len()).unwrap_or(u64::MAX));
        Ok(buffer.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// The bare model name to send on the wire. When a `Model` is constructed via
/// the accelerator's provider table, `name` is `"<provider>/<model>"` and the
/// bare model name is stashed in `extra["wire_name"]`. Hand-built `Model`s
/// without that extra fall back to `name` directly.
fn wire_name(model: &Model) -> &str {
    model
        .extra
        .get("wire_name")
        .and_then(|value| value.as_str())
        .unwrap_or(&model.name)
}

/// Call the active LLM and return the response fragments or an error.
pub async fn complete(
    ctx: &Context,
    resources: &Resources,
    overlay: &crate::overlay::Overlay,
) -> (Vec<Fragment>, TokenUsage) {
    let (fragments, usage, _) = complete_with_diagnostics(ctx, resources, overlay).await;
    (fragments, usage)
}

/// Call the active LLM and return content-free request/provider diagnostics.
pub async fn complete_with_diagnostics(
    ctx: &Context,
    resources: &Resources,
    overlay: &crate::overlay::Overlay,
) -> (Vec<Fragment>, TokenUsage, CompletionDiagnostics) {
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
            TokenUsage::empty(),
            CompletionDiagnostics::default(),
        );
    };

    let messages = assemble_messages(ctx.fragments(), model.thinking, overlay);

    let tools: Vec<RigToolDefinition> = resources
        .active_tool_definitions()
        .iter()
        .map(|definition| RigToolDefinition {
            name: definition.name.clone(),
            description: definition.description.clone(),
            parameters: definition.parameters.clone(),
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
    let wire_name = wire_name(model);

    let request = match build_request(&messages, &tools, model) {
        Ok(request) => request,
        Err(hitch) => {
            return (
                vec![hitch],
                TokenUsage::empty(),
                CompletionDiagnostics::default(),
            );
        }
    };
    let request_diagnostics = request_shape_diagnostics(&request, model.thinking);
    let mut diagnostics = CompletionDiagnostics {
        request: request_diagnostics,
        provider: ProviderFailureDiagnostics::default(),
    };

    let result = match model.protocol {
        Protocol::OpenAI => {
            let mut builder = rig::providers::openai::CompletionsClient::builder()
                .api_key(api_key)
                .http_client(openai_http_client());
            if let Some(endpoint) = endpoint_url {
                builder = builder.base_url(endpoint);
            }
            builder = apply_headers(builder, &model.headers);
            let endpoint = builder
                .build()
                .expect("failed to build openai client")
                .completion_model(wire_name);
            send(&endpoint, model, request).await
        }
        Protocol::DeepSeek => {
            // DeepSeek documents assistant `content` as string-or-null and
            // requires the complete prior reasoning/tool-call response to be
            // replayed. Its native Rig adapter owns that exact conversion.
            // Sources:
            // https://api-docs.deepseek.com/api/create-chat-completion/
            // https://api-docs.deepseek.com/guides/thinking_mode/
            let mut builder = rig::providers::deepseek::Client::builder()
                .api_key(api_key)
                .http_client(openai_http_client());
            if let Some(endpoint) = endpoint_url {
                builder = builder.base_url(endpoint);
            }
            builder = apply_headers(builder, &model.headers);
            let endpoint = builder
                .build()
                .expect("failed to build deepseek client")
                .completion_model(wire_name);
            send(&endpoint, model, request).await
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
                .completion_model(wire_name);
            send(&endpoint, model, request).await
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
                .completion_model(wire_name);
            send(&endpoint, model, request).await
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
            (decode(choice.iter()), usage, diagnostics)
        }
        Err(failure) => {
            diagnostics.provider = failure.provider;
            let http_status = match &failure.hitch.content {
                Content::Hitch { code, .. } => *code,
                _ => None,
            };
            warn!(
                model = %model.name,
                http_status = http_status.unwrap_or_default(),
                provider_code = diagnostics.provider.provider_code.as_deref().unwrap_or(""),
                provider_type = diagnostics.provider.provider_type.as_deref().unwrap_or(""),
                request_id = diagnostics.provider.request_id.as_deref().unwrap_or(""),
                request_class = diagnostics.provider.request_class.unwrap_or(""),
                serialized_request_bytes = diagnostics.request.serialized_request_bytes,
                estimated_input_tokens = diagnostics.request.estimated_input_tokens,
                message_count = diagnostics.request.message_count,
                tool_definition_count = diagnostics.request.tool_definition_count,
                tool_call_count = diagnostics.request.tool_call_count,
                tool_result_count = diagnostics.request.tool_result_count,
                thinking_enabled = diagnostics.request.thinking_enabled,
                reasoning_content_present = diagnostics.request.reasoning_content_present,
                reasoning_content_bytes = diagnostics.request.reasoning_content_bytes,
                unmatched_tool_call_count = diagnostics.request.unmatched_tool_call_count,
                duplicate_tool_call_count = diagnostics.request.duplicate_tool_call_count,
                "completion failed"
            );
            (vec![failure.hitch], TokenUsage::empty(), diagnostics)
        }
    }
}

/// Build the transport used for OpenAI-compatible completion endpoints.
///
/// Some compatible gateways advertise HTTP/2 but time out on a later, larger
/// tool-result request even though the same payload completes over HTTP/1.1.
/// RCM completion calls are independent, so keeping idle upstream connections
/// provides little benefit and makes gateway behaviour less deterministic.
/// Force HTTP/1.1 and close idle connections between calls.
fn openai_http_client() -> http_client::ReqwestClient {
    http_client::ReqwestClient::builder()
        .http1_only()
        .pool_max_idle_per_host(0)
        .build()
        .expect("failed to build openai HTTP client")
}

/// Encode the context while preserving provider-valid retry and tool-call turns.
///
/// Assistant hitches are transport failures, not model output. Keeping one in
/// chat history would turn a retry into a new prompt containing the gateway
/// error instead of replaying the failed request. Tool hitches remain encodable
/// because they are real tool results the model may need to correct.
///
/// A DeepSeek response may contain visible assistant text and one or more tool
/// calls with one shared reasoning block. The machine tape stores text as its
/// own fragment and each call beside its result for execution, so this function
/// reconstructs the original assistant turn before sending the next completion
/// request.
pub fn encode_context(fragments: &[Fragment], thinking: bool) -> Vec<Message> {
    let mut messages = Vec::with_capacity(fragments.len());
    let mut index = 0;

    while index < fragments.len() {
        let fragment = &fragments[index];

        if fragment.role == Role::Assistant && matches!(fragment.content, Content::Hitch { .. }) {
            index += 1;
            continue;
        }

        let mut tool_start = index;
        let mut leading_text = Vec::new();
        while let Some(candidate) = fragments.get(tool_start) {
            let Content::Text(text) = &candidate.content else {
                break;
            };
            if candidate.role != Role::Assistant {
                break;
            }
            leading_text.push(AssistantContent::text(&text.text));
            tool_start += 1;
        }

        let Some(tool_fragment) = fragments.get(tool_start) else {
            if let Some(message) = encode(fragment, thinking) {
                messages.push(message);
            }
            index += 1;
            continue;
        };
        let Content::ToolCall(first_call) = &tool_fragment.content else {
            if let Some(message) = encode(fragment, thinking) {
                messages.push(message);
            }
            index += 1;
            continue;
        };
        if tool_fragment.role != Role::Assistant {
            if let Some(message) = encode(fragment, thinking) {
                messages.push(message);
            }
            index += 1;
            continue;
        }
        let Some(shared_reasoning) = first_call.reasoning.as_deref() else {
            if let Some(message) = encode(fragment, thinking) {
                messages.push(message);
            }
            index += 1;
            continue;
        };

        let mut assistant_content = leading_text;
        let mut tool_results = Vec::new();
        let mut tool_call_count = 0;
        let mut cursor = tool_start;

        while let Some(candidate) = fragments.get(cursor) {
            let Content::ToolCall(call) = &candidate.content else {
                break;
            };
            if candidate.role != Role::Assistant
                || call.reasoning.as_deref() != Some(shared_reasoning)
            {
                break;
            }

            assistant_content.push(AssistantContent::tool_call(
                &call.id,
                &call.name,
                call.arguments.clone(),
            ));
            tool_call_count += 1;
            cursor += 1;

            let Some(result_fragment) = fragments.get(cursor) else {
                break;
            };
            let result_call_id = match &result_fragment.content {
                Content::ToolResult(result) => Some(result.call_id.as_str()),
                Content::Hitch {
                    call_id: Some(call_id),
                    ..
                } => Some(call_id.as_str()),
                _ => None,
            };
            if result_fragment.role != Role::Tool || result_call_id != Some(call.id.as_str()) {
                break;
            }
            if let Some(message) = encode(result_fragment, thinking) {
                tool_results.push(message);
            }
            cursor += 1;
        }

        let has_leading_text = tool_start > index;
        if (has_leading_text || tool_call_count > 1) && tool_call_count == tool_results.len() {
            if thinking && !has_leading_text {
                assistant_content.push(AssistantContent::text(""));
            }
            assistant_content.push(AssistantContent::Reasoning(Reasoning::new(
                shared_reasoning,
            )));
            let content = OneOrMany::many(assistant_content)
                .expect("parallel tool-call turn always contains at least two calls");
            messages.push(Message::Assistant { id: None, content });
            messages.extend(tool_results);
            index = cursor;
            continue;
        }

        if let Some(message) = encode(fragment, thinking) {
            messages.push(message);
        }
        index += 1;
    }

    messages
}

/// Assemble the full request message list: overlay prefix, encoded tape,
/// overlay tail. With an empty overlay this is exactly `encode_context` —
/// the golden-equivalence property the overlay layer must preserve.
pub fn assemble_messages(
    fragments: &[Fragment],
    thinking: bool,
    overlay: &crate::overlay::Overlay,
) -> Vec<Message> {
    let mut messages =
        Vec::with_capacity(overlay.system_prefix.len() + overlay.tail.len() + fragments.len());
    for prefix in &overlay.system_prefix {
        messages.push(Message::system(prefix.clone()));
    }
    messages.extend(encode_context(fragments, thinking));
    for tail in &overlay.tail {
        messages.push(Message::system(tail.clone()));
    }
    messages
}

// `Fragment` as the error type is intentional (see `build_request` below);
// newer clippy (1.98+) flags its size — same waiver, same rationale.
#[allow(clippy::result_large_err)]
async fn send(
    endpoint: &impl CompletionModel,
    model: &Model,
    request: CompletionRequest,
) -> Result<(OneOrMany<AssistantContent>, TokenUsage), CompletionFailure> {
    match timeout(
        Duration::from_secs(model.timeout),
        endpoint.completion(request),
    )
    .await
    {
        Ok(Ok(response)) => Ok((
            response.choice,
            TokenUsage {
                input_tokens: response.usage.input_tokens,
                output_tokens: response.usage.output_tokens,
                total_tokens: response.usage.total_tokens,
                cached_input_tokens: response.usage.cached_input_tokens,
                cache_creation_input_tokens: response.usage.cache_creation_input_tokens,
            },
        )),
        Ok(Err(error)) => {
            let (code, provider) = match &error {
                CompletionError::HttpError(http_client::Error::InvalidStatusCode(s)) => (
                    Some(s.as_u16()),
                    provider_failure_diagnostics(s.as_u16(), ""),
                ),
                CompletionError::HttpError(http_client::Error::InvalidStatusCodeWithMessage(
                    s,
                    message,
                )) => (
                    Some(s.as_u16()),
                    provider_failure_diagnostics(s.as_u16(), message),
                ),
                _ => (None, ProviderFailureDiagnostics::default()),
            };
            let hitch_message = provider_hitch_message(&error, code);
            Err(CompletionFailure {
                hitch: Fragment::hitch(hitch_message, code, Role::Assistant, None::<&str>),
                provider,
            })
        }
        Err(_) => Err(CompletionFailure {
            hitch: Fragment::hitch(
                format!("request timed out after {}s", model.timeout),
                None,
                Role::Assistant,
                None::<&str>,
            ),
            provider: ProviderFailureDiagnostics::default(),
        }),
    }
}

fn provider_hitch_message(error: &CompletionError, status: Option<u16>) -> String {
    status.map_or_else(
        || error.to_string(),
        |status| format!("provider completion failed with HTTP {status}"),
    )
}

/// Measure a normalized generic request without retaining any request content.
pub fn request_shape_diagnostics(
    request: &CompletionRequest,
    thinking_enabled: bool,
) -> RequestShapeDiagnostics {
    use std::collections::{HashMap, HashSet};

    let mut counter = CountingWriter::default();
    let serialized_request_bytes = serde_json::to_writer(&mut counter, request)
        .map(|()| counter.bytes)
        .unwrap_or_default();
    let mut tool_call_ids = Vec::new();
    let mut tool_result_counts: HashMap<&str, u64> = HashMap::new();
    let mut reasoning_content_bytes = 0u64;
    let mut reasoning_content_present = false;
    let mut tool_result_count = 0u64;

    for message in request.chat_history.iter() {
        match message {
            Message::Assistant { content, .. } => {
                for item in content.iter() {
                    match item {
                        AssistantContent::ToolCall(tool_call) => {
                            tool_call_ids.push(tool_call.id.as_str());
                        }
                        AssistantContent::Reasoning(reasoning) => {
                            reasoning_content_present = true;
                            let bytes = serde_json::to_vec(reasoning)
                                .map(|serialized| serialized.len())
                                .unwrap_or_default();
                            reasoning_content_bytes = reasoning_content_bytes
                                .saturating_add(u64::try_from(bytes).unwrap_or(u64::MAX));
                        }
                        _ => {}
                    }
                }
            }
            Message::User { content } => {
                for item in content.iter() {
                    if let UserContent::ToolResult(tool_result) = item {
                        tool_result_count = tool_result_count.saturating_add(1);
                        let call_id = tool_result
                            .call_id
                            .as_deref()
                            .unwrap_or(tool_result.id.as_str());
                        *tool_result_counts.entry(call_id).or_default() += 1;
                    }
                }
            }
            Message::System { .. } => {}
        }
    }

    let mut seen_call_ids = HashSet::new();
    let duplicate_tool_call_count = tool_call_ids.iter().fold(0u64, |count, call_id| {
        count.saturating_add(u64::from(!seen_call_ids.insert(*call_id)))
    });
    let mut remaining_results = tool_result_counts;
    let unmatched_tool_call_count = tool_call_ids.iter().fold(0u64, |count, call_id| {
        let Some(remaining) = remaining_results.get_mut(*call_id) else {
            return count.saturating_add(1);
        };
        if *remaining == 0 {
            count.saturating_add(1)
        } else {
            *remaining -= 1;
            count
        }
    });

    RequestShapeDiagnostics {
        serialized_request_bytes,
        estimated_input_tokens: serialized_request_bytes
            .saturating_add(ESTIMATED_CHARS_PER_TOKEN - 1)
            .saturating_div(ESTIMATED_CHARS_PER_TOKEN),
        message_count: u64::try_from(request.chat_history.len()).unwrap_or(u64::MAX),
        tool_definition_count: u64::try_from(request.tools.len()).unwrap_or(u64::MAX),
        tool_call_count: u64::try_from(tool_call_ids.len()).unwrap_or(u64::MAX),
        tool_result_count,
        thinking_enabled,
        reasoning_content_present,
        reasoning_content_bytes,
        unmatched_tool_call_count,
        duplicate_tool_call_count,
    }
}

fn provider_failure_diagnostics(status: u16, body: &str) -> ProviderFailureDiagnostics {
    let parsed = serde_json::from_str::<serde_json::Value>(body).ok();
    let provider_code = parsed
        .as_ref()
        .and_then(|value| provider_identifier(value, "code"));
    let provider_type = parsed
        .as_ref()
        .and_then(|value| provider_identifier(value, "type"));
    let request_id = parsed
        .as_ref()
        .and_then(|value| provider_identifier(value, "request_id"))
        .or_else(|| {
            parsed
                .as_ref()
                .and_then(|value| provider_identifier(value, "requestId"))
        });
    let request_class = classify_provider_request(
        status,
        provider_code.as_deref(),
        provider_type.as_deref(),
        body,
    );

    ProviderFailureDiagnostics {
        provider_code,
        provider_type,
        request_id,
        request_class,
    }
}

fn provider_identifier(value: &serde_json::Value, key: &str) -> Option<String> {
    let candidate = value
        .get("error")
        .and_then(|error| error.get(key))
        .or_else(|| value.get(key))?;
    let candidate = match candidate {
        serde_json::Value::String(value) => value.clone(),
        serde_json::Value::Number(value) => value.to_string(),
        _ => return None,
    };
    let valid = !candidate.is_empty()
        && candidate.len() <= MAX_PROVIDER_IDENTIFIER_BYTES
        && candidate
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.' | b':'));
    valid.then_some(candidate)
}

fn classify_provider_request(
    status: u16,
    provider_code: Option<&str>,
    provider_type: Option<&str>,
    body: &str,
) -> Option<&'static str> {
    if status != 400 && status != 413 {
        return None;
    }
    let normalized = format!(
        "{} {} {}",
        provider_code.unwrap_or_default(),
        provider_type.unwrap_or_default(),
        body
    )
    .to_ascii_lowercase();
    let size_markers = [
        "context_length",
        "context length",
        "maximum context",
        "request_too_large",
        "request too large",
        "payload_too_large",
        "payload too large",
        "too many tokens",
        "token limit",
    ];
    if status == 413
        || size_markers
            .iter()
            .any(|marker| normalized.contains(marker))
    {
        return Some("request_size");
    }
    let invalid_history_marker = ["missing", "required", "unmatched", "duplicate", "invalid"]
        .iter()
        .any(|marker| normalized.contains(marker));
    if normalized.contains("reasoning_content")
        || normalized.contains("thinking is enabled")
        || ((normalized.contains("tool_call") || normalized.contains("tool call"))
            && invalid_history_marker)
    {
        return Some("thinking_tool_history");
    }
    Some("unknown_request")
}

/// Construct a `CompletionRequest` whose `chat_history` is the encoded
/// fragments **in their original order**.
///
/// Returns a `Fragment::hitch` when `messages` is empty — an empty context
/// has no meaningful prompt to send to the LLM, and we refuse to fabricate
/// one (see module-level contract).
///
/// `Fragment` is the unified error type — boxing would lose the
/// ergonomics of returning `Fragment::hitch(...)` directly. The size is
/// accepted intentionally.
#[allow(clippy::result_large_err)]
pub fn build_request(
    messages: &[Message],
    tools: &[RigToolDefinition],
    model: &Model,
) -> Result<CompletionRequest, Fragment> {
    let normalized_messages = merge_adjacent_system_messages(messages);
    let chat_history = OneOrMany::many(normalized_messages).map_err(|_| {
        Fragment::hitch(
            "completion called with empty context — no messages to send to LLM",
            None,
            Role::System,
            None::<&str>,
        )
    })?;

    let additional_params = match model.protocol {
        Protocol::OpenAI => model.thinking_mode().map(|enabled| {
            serde_json::json!({
                "thinking": {
                    "type": if enabled { "enabled" } else { "disabled" },
                }
            })
        }),
        Protocol::DeepSeek => {
            let mut params = serde_json::Map::new();
            if let Some(enabled) = model.thinking_mode() {
                params.insert(
                    "thinking".into(),
                    serde_json::json!({
                        "type": if enabled { "enabled" } else { "disabled" },
                    }),
                );
            }
            // Rig 0.36's native DeepSeek adapter does not project the generic
            // CompletionRequest max_tokens field, so preserve the declared
            // model limit through its flattened provider parameters.
            if let Some(limit) = &model.limit {
                params.insert("max_tokens".into(), limit.output.into());
            }
            (!params.is_empty()).then_some(serde_json::Value::Object(params))
        }
        Protocol::Anthropic | Protocol::Gemini => None,
    };

    Ok(CompletionRequest {
        model: None,
        preamble: None,
        chat_history,
        documents: Vec::new(),
        tools: tools.to_vec(),
        temperature: model.temperature,
        max_tokens: model.limit.as_ref().map(|l| l.output),
        tool_choice: None,
        additional_params,
        output_schema: None,
    })
}

fn merge_adjacent_system_messages(messages: &[Message]) -> Vec<Message> {
    let mut normalized = Vec::with_capacity(messages.len());
    for message in messages {
        if let Message::System { content } = message
            && let Some(Message::System {
                content: previous_content,
            }) = normalized.last_mut()
        {
            previous_content.push_str("\n\n");
            previous_content.push_str(content);
            continue;
        }
        normalized.push(message.clone());
    }
    normalized
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
    // A completion response is one assistant turn. Providers may emit visible
    // text before a tool call in that same turn, so its reasoning must remain
    // available until every tool call in the response has been decoded.
    let mut pending_reasoning: Vec<String> = Vec::new();
    for content in choice {
        match content {
            AssistantContent::Text(text) => {
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
                // Source: DeepSeek "Thinking Mode" tool-call contract —
                // https://api-docs.deepseek.com/guides/thinking_mode/
                // DeepSeek requires assistant `content` beside thinking-mode tool
                // calls. Rig omits the wire field for an empty text collection, so
                // retain an explicit empty item when no visible text was emitted.
                if thinking {
                    content.push(AssistantContent::text(""));
                }
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

#[cfg(test)]
mod transport_tests {
    use super::{provider_failure_diagnostics, provider_hitch_message};

    #[test]
    fn openai_http_client_builds_with_gateway_safe_settings() {
        let _client = super::openai_http_client();
    }

    #[test]
    fn provider_failure_diagnostics_classify_without_retaining_response_message() {
        let sensitive = "PRIVATE PAPER BODY MUST NOT ENTER DIAGNOSTICS";
        let body = serde_json::json!({
            "error": {
                "message": format!("{sensitive}: reasoning_content is missing"),
                "type": "invalid_request_error",
                "code": "invalid_request_error"
            },
            "request_id": "req-123"
        })
        .to_string();

        let diagnostics = provider_failure_diagnostics(400, &body);

        assert_eq!(
            diagnostics.provider_code.as_deref(),
            Some("invalid_request_error")
        );
        assert_eq!(
            diagnostics.provider_type.as_deref(),
            Some("invalid_request_error")
        );
        assert_eq!(diagnostics.request_id.as_deref(), Some("req-123"));
        assert_eq!(diagnostics.request_class, Some("thinking_tool_history"));
        assert!(!format!("{diagnostics:?}").contains(sensitive));
    }

    #[test]
    fn provider_failure_diagnostics_distinguish_size_and_unknown_requests() {
        let size = provider_failure_diagnostics(
            400,
            r#"{"error":{"code":"context_length_exceeded","message":"too many tokens"}}"#,
        );
        let unknown = provider_failure_diagnostics(
            400,
            r#"{"error":{"code":"bad code with spaces","message":"rejected"}}"#,
        );

        assert_eq!(size.request_class, Some("request_size"));
        assert_eq!(unknown.request_class, Some("unknown_request"));
        assert_eq!(unknown.provider_code, None);
    }

    #[test]
    fn http_hitch_summary_does_not_retain_provider_body() {
        let sensitive = "PRIVATE PAPER BODY MUST NOT ENTER THE HITCH";
        let error = rig::completion::CompletionError::HttpError(
            rig::http_client::Error::InvalidStatusCodeWithMessage(
                http::StatusCode::BAD_REQUEST,
                sensitive.into(),
            ),
        );
        let status = Some(http::StatusCode::BAD_REQUEST.as_u16());
        let message = provider_hitch_message(&error, status);

        assert_eq!(message, "provider completion failed with HTTP 400");
        assert!(!message.contains(sensitive));
    }
}
