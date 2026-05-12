use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::context::Context;
use crate::fragment::Fragment;
use crate::register::Register;
use crate::tool::Tool;
use crate::trace::Trace;

// ============================================================================
// LlmBackend
// ============================================================================

/// The response from a single LLM call.
#[derive(Clone)]
pub struct LlmResponse {
    pub text: Option<String>,
    pub tool_calls: Vec<crate::fragment::ToolCallDef>,
    pub tokens: crate::trace::TokenUsage,
}

/// Abstraction over the underlying LLM provider.
pub trait LlmBackend: Send + Sync {
    fn complete(
        &self,
        fragments: &[&Fragment],
        tools: &[Tool],
    ) -> Pin<Box<dyn Future<Output = Result<LlmResponse, String>> + Send + '_>>;

    fn summarize(
        &self,
        fragments: &[&Fragment],
    ) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send + '_>>;
}

// ============================================================================
// RigBackend
// ============================================================================

trait DynModel: Send + Sync {
    fn completion(
        &self,
        request: rig::completion::CompletionRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        rig::completion::CompletionResponse<serde_json::Value>,
                        rig::completion::CompletionError,
                    >,
                > + Send
                + '_,
        >,
    >;
}

struct ModelAdapter<M: rig::completion::CompletionModel> {
    model: M,
}

impl<M: rig::completion::CompletionModel> ModelAdapter<M> {
    fn new(model: M) -> Self {
        Self { model }
    }
}

impl<M: rig::completion::CompletionModel> DynModel for ModelAdapter<M> {
    fn completion(
        &self,
        request: rig::completion::CompletionRequest,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        rig::completion::CompletionResponse<serde_json::Value>,
                        rig::completion::CompletionError,
                    >,
                > + Send
                + '_,
        >,
    > {
        let model = self.model.clone();
        Box::pin(async move {
            model
                .completion(request)
                .await
                .map(|resp| rig::completion::CompletionResponse {
                    choice: resp.choice,
                    usage: resp.usage,
                    raw_response: serde_json::to_value(&resp.raw_response).unwrap_or_default(),
                    message_id: resp.message_id,
                })
        })
    }
}

/// LlmBackend backed by a rig CompletionModel.
pub struct RigBackend {
    model: Box<dyn DynModel>,
}

impl RigBackend {
    pub fn new<M: rig::completion::CompletionModel + 'static>(model: M) -> Self {
        Self {
            model: Box::new(ModelAdapter::new(model)),
        }
    }
}

impl LlmBackend for RigBackend {
    fn complete(
        &self,
        fragments: &[&Fragment],
        tools: &[Tool],
    ) -> Pin<Box<dyn Future<Output = Result<LlmResponse, String>> + Send + '_>> {
        let request = build_completion_request(fragments, tools);
        Box::pin(async move {
            let resp = self
                .model
                .completion(request)
                .await
                .map_err(|e| e.to_string())?;

            let first = resp.choice.first();
            let text = match &first {
                rig::completion::AssistantContent::Text(t) => Some(t.text.clone()),
                _ => None,
            };

            let tool_calls: Vec<crate::fragment::ToolCallDef> = resp
                .choice
                .iter()
                .filter_map(|c| match c {
                    rig::completion::AssistantContent::ToolCall(tc) => {
                        Some(crate::fragment::ToolCallDef {
                            id: tc.id.clone(),
                            name: tc.function.name.clone(),
                            arguments: tc.function.arguments.to_string(),
                        })
                    }
                    _ => None,
                })
                .collect();

            Ok(LlmResponse {
                text,
                tool_calls,
                tokens: crate::trace::TokenUsage {
                    prompt_tokens: resp.usage.input_tokens,
                    completion_tokens: resp.usage.output_tokens,
                    total_tokens: resp.usage.total_tokens,
                },
            })
        })
    }

    fn summarize(
        &self,
        fragments: &[&Fragment],
    ) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send + '_>> {
        let texts: Vec<String> = fragments
            .iter()
            .filter_map(|f| f.as_text().map(|t| t.to_string()))
            .collect();
        let joined = texts.join("\n");

        let prompt = Fragment::user_text(format!(
            "Summarize the following conversation into a concise summary. \
             Keep all key decisions, facts, and action items:\n\n{joined}"
        ));

        let request = build_completion_request(&[&prompt], &[]);
        Box::pin(async move {
            let resp = self
                .model
                .completion(request)
                .await
                .map_err(|e| e.to_string())?;
            let first = resp.choice.first();
            let text = match first {
                rig::completion::AssistantContent::Text(t) => t.text,
                _ => String::new(),
            };
            Ok(text)
        })
    }
}

fn build_completion_request(
    fragments: &[&Fragment],
    tools: &[Tool],
) -> rig::completion::CompletionRequest {
    use rig::OneOrMany;
    use rig::completion::{AssistantContent, Message, ToolDefinition};
    use rig::message::{
        DocumentSourceKind, Image, ImageMediaType, MimeType, Text, ToolCall, ToolFunction,
        ToolResult, ToolResultContent, UserContent,
    };

    use crate::fragment::{Content, Role};
    use serde_json::Value;

    let mut chat_history = Vec::new();

    for frag in fragments {
        match frag.role {
            Role::System => {
                if let Some(text) = frag.as_text() {
                    chat_history.push(Message::System {
                        content: text.to_string(),
                    });
                }
            }
            Role::User => match &frag.content {
                Content::Text(text) => {
                    chat_history.push(Message::User {
                        content: OneOrMany::one(UserContent::Text(Text { text: text.clone() })),
                    });
                }
                Content::ToolResult { call_id, text } => {
                    chat_history.push(Message::User {
                        content: OneOrMany::one(UserContent::ToolResult(ToolResult {
                            id: call_id.clone(),
                            call_id: Some(call_id.clone()),
                            content: OneOrMany::one(ToolResultContent::Text(Text {
                                text: text.clone(),
                            })),
                        })),
                    });
                }
                Content::Image { data, mime } => {
                    chat_history.push(Message::User {
                        content: OneOrMany::one(UserContent::Image(Image {
                            data: DocumentSourceKind::Base64(data.clone()),
                            media_type: ImageMediaType::from_mime_type(&mime),
                            detail: None,
                            additional_params: None,
                        })),
                    });
                }
                _ => {}
            },
            Role::Assistant => match &frag.content {
                Content::Text(text) => {
                    chat_history.push(Message::Assistant {
                        id: None,
                        content: OneOrMany::one(AssistantContent::Text(Text {
                            text: text.clone(),
                        })),
                    });
                }
                Content::ToolCalls(calls) => {
                    let contents: Vec<AssistantContent> = calls
                        .iter()
                        .map(|tc| {
                            AssistantContent::ToolCall(ToolCall::new(
                                tc.id.clone(),
                                ToolFunction {
                                    name: tc.name.clone(),
                                    arguments: serde_json::from_str(&tc.arguments)
                                        .unwrap_or(Value::Null),
                                },
                            ))
                        })
                        .collect();
                    chat_history.push(Message::Assistant {
                        id: None,
                        content: OneOrMany::many(contents).unwrap_or(OneOrMany::one(
                            AssistantContent::Text(Text {
                                text: String::new(),
                            }),
                        )),
                    });
                }
                _ => {}
            },
        }
    }

    let tool_defs: Vec<ToolDefinition> = tools
        .iter()
        .map(|t| ToolDefinition {
            name: t.name.clone(),
            description: t.description.clone(),
            parameters: t.schema.clone(),
        })
        .collect();

    rig::completion::CompletionRequest {
        model: None,
        preamble: None,
        chat_history: OneOrMany::many(chat_history).unwrap_or(OneOrMany::one(Message::System {
            content: String::new(),
        })),
        documents: vec![],
        tools: tool_defs,
        temperature: None,
        max_tokens: None,
        tool_choice: None,
        additional_params: None,
        output_schema: None,
    }
}

// ============================================================================
// HaltCondition
// ============================================================================

/// A predicate that checks whether the machine should stop.
pub trait HaltCondition: Send + Sync {
    fn check(&self, ctx: &Context, register: &Register, trace: &Trace) -> bool;
}

/// Halts after N cycles.
pub struct MaxCycles(pub u32);

impl HaltCondition for MaxCycles {
    fn check(&self, _ctx: &Context, _register: &Register, trace: &Trace) -> bool {
        trace.cycle_count() >= self.0
    }
}

// ============================================================================
// Head
// ============================================================================

/// Head — the read/write head of the Turing machine.
///
/// Head is a configuration bundle, not an active executor. It holds
/// the engine (transition decision), LLM backend, and halt conditions.
/// The actual cycle loop lives in [`accelerate`](crate::accelerate).
pub struct Head {
    pub engine: Box<dyn crate::engine::Engine>,
    pub llm: Arc<dyn LlmBackend>,
    pub halt_conditions: Vec<Box<dyn HaltCondition>>,
}

impl Head {
    pub fn new(engine: Box<dyn crate::engine::Engine>, llm: Arc<dyn LlmBackend>) -> Self {
        Self {
            engine,
            llm,
            halt_conditions: vec![Box::new(MaxCycles(50))],
        }
    }

    pub fn with_halt(mut self, condition: Box<dyn HaltCondition>) -> Self {
        self.halt_conditions.push(condition);
        self
    }
}
