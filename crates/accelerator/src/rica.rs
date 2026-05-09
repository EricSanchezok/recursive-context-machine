use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Instant;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::engine::{self, Action, Engine};
use crate::fragment::{Content, Fragment, Role, ToolCallDef};
use crate::register::Register;
use crate::tape::Tape;
use crate::tool::Tool;
use crate::trace::{Cycle, TokenUsage, Trace};

// ============================================================================
// Intent & Output
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    pub prompt: String,
    pub context: HashMap<String, Value>,
}

impl Intent {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            context: HashMap::new(),
        }
    }

    pub fn with_context(mut self, key: impl Into<String>, value: Value) -> Self {
        self.context.insert(key.into(), value);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    pub text: String,
    pub cycles: u32,
    pub tokens: TokenUsage,
}

// ============================================================================
// LlmBackend
// ============================================================================

#[derive(Clone)]
pub struct LlmResponse {
    pub text: Option<String>,
    pub tool_calls: Vec<ToolCallDef>,
    pub tokens: TokenUsage,
}

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

            let tool_calls: Vec<ToolCallDef> = resp
                .choice
                .iter()
                .filter_map(|c| match c {
                    rig::completion::AssistantContent::ToolCall(tc) => Some(ToolCallDef {
                        id: tc.id.clone(),
                        name: tc.function.name.clone(),
                        arguments: tc.function.arguments.to_string(),
                    }),
                    _ => None,
                })
                .collect();

            Ok(LlmResponse {
                text,
                tool_calls,
                tokens: TokenUsage {
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

pub trait HaltCondition: Send + Sync {
    fn check(&self, tape: &Tape, register: &Register, trace: &Trace) -> bool;
}

pub struct MaxCycles(pub u32);

impl HaltCondition for MaxCycles {
    fn check(&self, _tape: &Tape, _register: &Register, trace: &Trace) -> bool {
        trace.cycle_count() >= self.0
    }
}

// ============================================================================
// Rica trait
// ============================================================================

pub trait Rica: Send + Sync {
    fn accelerate(
        &self,
        intent: Intent,
        register: Register,
    ) -> Pin<Box<dyn Future<Output = (Output, Register, Trace)> + Send + '_>>;
}

// ============================================================================
// DefaultRica
// ============================================================================

pub struct DefaultRica {
    engine: Box<dyn Engine>,
    llm: Arc<dyn LlmBackend>,
    halt_conditions: Vec<Box<dyn HaltCondition>>,
}

impl DefaultRica {
    pub fn new(engine: Box<dyn Engine>, llm: Arc<dyn LlmBackend>) -> Self {
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

impl Rica for DefaultRica {
    fn accelerate(
        &self,
        intent: Intent,
        register: Register,
    ) -> Pin<Box<dyn Future<Output = (Output, Register, Trace)> + Send + '_>> {
        Box::pin(async move {
            let mut tape = Tape::new();
            let register = register;
            let mut trace = Trace::new();

            tape.write(Fragment::system("You are a helpful assistant."));
            tape.right();
            tape.write(Fragment::user_text(&intent.prompt));
            tape.right();

            loop {
                let action = self.engine.decide(&tape, &register);

                match action {
                    Action::CallLlm => {
                        let frags = tape.fragments();
                        let response = match self.llm.complete(&frags, &register.tools).await {
                            Ok(r) => r,
                            Err(e) => {
                                engine::append_fragment(
                                    &mut tape,
                                    Fragment::assistant_text(format!("Error: {e}")),
                                );
                                trace.record(Cycle::Halt {
                                    reason: format!("LLM error: {e}"),
                                });
                                break;
                            }
                        };

                        trace.record(Cycle::LlmCall {
                            tokens: response.tokens.clone(),
                        });

                        if !response.tool_calls.is_empty() {
                            engine::append_fragment(
                                &mut tape,
                                Fragment::assistant_tool_calls(response.tool_calls.clone()),
                            );

                            for tc in &response.tool_calls {
                                let tool = register.tools.iter().find(|t| t.name == tc.name);

                                let (output, duration_ms) = if let Some(tool) = tool {
                                    let args: Value =
                                        serde_json::from_str(&tc.arguments).unwrap_or(Value::Null);
                                    let start = Instant::now();
                                    let result = (tool.run)(args, register.clone()).await;
                                    let duration = start.elapsed().as_millis() as u64;
                                    (result, duration)
                                } else {
                                    (Err(format!("tool '{}' not found", tc.name)), 0u64)
                                };

                                let result_text = match &output {
                                    Ok(v) => v.to_string(),
                                    Err(e) => e.clone(),
                                };

                                trace.record(Cycle::ToolCall {
                                    tool: tc.name.clone(),
                                    input: serde_json::from_str(&tc.arguments)
                                        .unwrap_or(Value::Null),
                                    output: output.clone(),
                                    duration_ms,
                                });

                                engine::append_fragment(
                                    &mut tape,
                                    Fragment::tool_result(&tc.id, &result_text),
                                );
                            }
                        } else if let Some(text) = response.text {
                            engine::append_fragment(&mut tape, Fragment::assistant_text(text));
                        }
                    }
                    Action::Prune { from, to, reason } => {
                        engine::prune_range(&mut tape, from, to);
                        trace.record(Cycle::Prune { from, to, reason });
                    }
                    Action::Compact { from, to } => {
                        let frags = tape.fragments();
                        let range: Vec<&Fragment> = frags
                            .iter()
                            .skip(from)
                            .take(to.saturating_sub(from))
                            .copied()
                            .collect();

                        let summary_text = self
                            .llm
                            .summarize(&range)
                            .await
                            .unwrap_or_else(|e| format!("[compaction failed: {e}]"));

                        engine::compact_range(&mut tape, from, to, Fragment::system(summary_text));
                        trace.record(Cycle::Compact { from, to });
                    }
                    Action::Halt { reason } => {
                        trace.record(Cycle::Halt { reason });
                        break;
                    }
                }

                if self
                    .halt_conditions
                    .iter()
                    .any(|h| h.check(&tape, &register, &trace))
                {
                    break;
                }
            }

            let frags = tape.fragments();
            let last_text = frags
                .iter()
                .rev()
                .find(|f| f.role == Role::Assistant)
                .and_then(|f| f.as_text())
                .unwrap_or("")
                .to_string();

            let output = Output {
                text: last_text,
                cycles: trace
                    .cycles
                    .iter()
                    .filter(|c| !matches!(c, Cycle::Halt { .. }))
                    .count() as u32,
                tokens: trace.total_tokens(),
            };

            (output, register, trace)
        })
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{Engine, PipelineEngine};
    use crate::fragment::Fragment;
    use std::pin::Pin;

    struct MockLlm {
        responses: Vec<LlmResponse>,
        call_count: std::sync::Mutex<usize>,
    }

    impl MockLlm {
        fn new(responses: Vec<LlmResponse>) -> Self {
            Self {
                responses,
                call_count: std::sync::Mutex::new(0),
            }
        }
    }

    impl LlmBackend for MockLlm {
        fn complete(
            &self,
            _fragments: &[&Fragment],
            _tools: &[Tool],
        ) -> Pin<Box<dyn Future<Output = Result<LlmResponse, String>> + Send + '_>> {
            let mut count = self.call_count.lock().unwrap();
            let idx = *count;
            *count += 1;
            let resp = self.responses.get(idx).cloned().unwrap_or(LlmResponse {
                text: Some("done".into()),
                tool_calls: vec![],
                tokens: TokenUsage::default(),
            });
            Box::pin(async move { Ok(resp) })
        }

        fn summarize(
            &self,
            _fragments: &[&Fragment],
        ) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send + '_>> {
            Box::pin(async move { Ok("summary".into()) })
        }
    }

    #[tokio::test]
    async fn test_simple_prompt() {
        let llm = Arc::new(MockLlm::new(vec![LlmResponse {
            text: Some("Hello!".into()),
            tool_calls: vec![],
            tokens: TokenUsage {
                prompt_tokens: 10,
                completion_tokens: 2,
                total_tokens: 12,
            },
        }]));

        let engine: Box<dyn Engine> = Box::new(PipelineEngine::default());
        let rica = DefaultRica::new(engine, llm);

        let (output, _register, trace) = rica
            .accelerate(Intent::new("say hi"), Register::new("/tmp".into()))
            .await;

        assert_eq!(output.text, "Hello!");
        assert_eq!(output.cycles, 1);
        assert_eq!(output.tokens.total_tokens, 12);
        assert_eq!(trace.cycles.len(), 2);
    }

    #[tokio::test]
    async fn test_with_tool_call() {
        let llm = Arc::new(MockLlm::new(vec![
            LlmResponse {
                text: None,
                tool_calls: vec![ToolCallDef {
                    id: "call_1".into(),
                    name: "echo".into(),
                    arguments: r#"{"msg":"hello"}"#.into(),
                }],
                tokens: TokenUsage {
                    prompt_tokens: 10,
                    completion_tokens: 5,
                    total_tokens: 15,
                },
            },
            LlmResponse {
                text: Some("Got it!".into()),
                tool_calls: vec![],
                tokens: TokenUsage {
                    prompt_tokens: 20,
                    completion_tokens: 3,
                    total_tokens: 23,
                },
            },
        ]));

        let engine: Box<dyn Engine> = Box::new(PipelineEngine::default());
        let rica = DefaultRica::new(engine, llm);

        let echo_tool = Tool::from_fn(
            "echo",
            "echo a message",
            serde_json::json!({"type": "object", "properties": {"msg": {"type": "string"}}}),
            |args, _reg| async move { Ok(args) },
        );

        let mut register = Register::new("/tmp".into());
        register.tools.push(echo_tool);

        let (output, _register, trace) = rica.accelerate(Intent::new("test"), register).await;

        assert_eq!(output.text, "Got it!");
        assert_eq!(output.cycles, 3);
        assert_eq!(output.tokens.total_tokens, 38);
        assert_eq!(trace.cycles.len(), 4);
    }
}
