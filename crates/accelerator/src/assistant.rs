//! Metered completion assistant for generative tools (C3).
//!
//! The production [`CompletionAssistant`]: one shared context snapshot plus
//! the active model, refreshed by the fire loop every step before `decide`.
//! Tools never call models directly — they go through this gateway, which
//! enforces the per-tool-call completion cap and reports token usage back
//! through the tool payload so the machine can mirror it into the WAL.
//!
//! [`StubAssistant`] is the LLM-unavailable test double: canned responses,
//! no network, identical contract.

use std::sync::{Arc, RwLock};

use machine::completion;
use machine::overlay::Overlay;
use machine::{
    AssistantFuture, AssistantRequest, CompletionAssistant, Context, Fragment,
    MAX_COMPLETIONS_PER_TOOL_CALL, MachineState, Model, Resources, Role, TokenUsage,
};

/// Shared, read-only snapshot of the live document. The fire loop calls
/// [`ContextSnapshot::publish`] before each decide so generative tools see
/// the document as of this step even mid-batch.
#[derive(Clone, Default)]
pub struct ContextSnapshot {
    inner: Arc<RwLock<Option<Context>>>,
}

impl ContextSnapshot {
    pub fn new() -> Self {
        Self::default()
    }

    /// Replace the snapshot with the current document.
    pub fn publish(&self, context: &Context) {
        if let Ok(mut guard) = self.inner.write() {
            *guard = Some(context.clone());
        }
    }

    fn current(&self) -> Option<Context> {
        self.inner.read().ok().and_then(|guard| guard.clone())
    }
}

/// The production assistant: snapshot + active model + per-tool-call meter.
/// The fire loop (or gRPC server) calls [`AssistantGateway::publish`] once
/// per step; tools reach the gateway through `Environment::assistant`.
pub struct AssistantGateway {
    snapshot: ContextSnapshot,
    active_model: RwLock<Option<Model>>,
    charged: RwLock<u32>,
}

impl Default for AssistantGateway {
    fn default() -> Self {
        Self::new()
    }
}

impl AssistantGateway {
    pub fn new() -> Self {
        Self {
            snapshot: ContextSnapshot::new(),
            active_model: RwLock::new(None),
            charged: RwLock::new(0),
        }
    }

    /// Publish the step-start document and the active model. One call per
    /// step keeps concurrent tool readers on a consistent view.
    pub fn publish(&self, state: &MachineState) {
        self.snapshot.publish(&state.run.context);
        if let Ok(mut guard) = self.active_model.write() {
            *guard = state.run.resources.active_model().cloned();
        }
    }

    fn meter(&self) -> u32 {
        self.charged.read().map(|guard| *guard).unwrap_or(0)
    }
}

impl CompletionAssistant for AssistantGateway {
    fn begin_tool_call(&self) {
        if let Ok(mut guard) = self.charged.write() {
            *guard = 0;
        }
    }

    fn complete(&self, request: AssistantRequest) -> AssistantFuture<'_> {
        Box::pin(async move {
            if self.meter() >= MAX_COMPLETIONS_PER_TOOL_CALL {
                return Err(format!(
                    "completion cap reached for this tool call ({}); a tool may not escalate into a chat loop",
                    MAX_COMPLETIONS_PER_TOOL_CALL
                ));
            }
            let model = self
                .active_model
                .read()
                .ok()
                .and_then(|guard| guard.clone())
                .ok_or("no active model set; activate a model before generating content")?;

            let document = self.snapshot.current().ok_or(
                "no context snapshot published; the accelerator must publish before decide",
            )?;
            let source_ids = document
                .select(&request.source)
                .map_err(|error| format!("source selector failed: {error}"))?;
            let source_text = document
                .fragments()
                .iter()
                .filter(|cell| source_ids.contains(&cell.id()))
                .map(Fragment::content_as_text)
                .collect::<Vec<_>>()
                .join("\n\n");
            if source_text.trim().is_empty() {
                return Err("source selector resolved to no content".into());
            }

            // One-off request document: instruction (system) + source
            // material (user). No tools offered — an embedded completion
            // is content generation, not another agent turn. The model is
            // both registered and selected so `complete` finds it active.
            let mut request_document = Context::new();
            request_document.append(Fragment::system(request.instruction.clone()));
            request_document.append(Fragment::user(source_text));
            let model_name = model.name.clone();
            let mut resources = Resources::new().with_model(model);
            let _ = resources.use_model(model_name);

            let (fragments, usage) =
                completion::complete(&request_document, &resources, &Overlay::default()).await;
            if let Some(message) = fragments
                .iter()
                .find_map(|fragment| match &fragment.content {
                    machine::Content::Hitch { message, .. } => Some(message.clone()),
                    _ => None,
                })
            {
                return Err(message);
            }
            if let Ok(mut guard) = self.charged.write() {
                *guard += 1;
            }
            let text = fragments
                .iter()
                .filter(|fragment| fragment.role == Role::Assistant)
                .filter_map(|fragment| fragment.as_text().map(String::from))
                .collect::<Vec<_>>()
                .join("\n");
            Ok((text, usage))
        })
    }
}

/// Test-double assistant: returns canned text FIFO, records nothing. The
/// Blueprint's LLM-unavailable path — behavior verification without keys.
#[derive(Clone, Default)]
pub struct StubAssistant {
    pub responses: Arc<RwLock<Vec<String>>>,
}

impl StubAssistant {
    /// Queue one canned response (consumed in order; exhaustion errors).
    pub fn enqueue(&self, text: impl Into<String>) {
        if let Ok(mut guard) = self.responses.write() {
            guard.push(text.into());
        }
    }
}

impl CompletionAssistant for StubAssistant {
    fn begin_tool_call(&self) {}

    fn complete(&self, _request: AssistantRequest) -> AssistantFuture<'_> {
        Box::pin(async move {
            let next = self
                .responses
                .write()
                .ok()
                .and_then(|mut guard| (!guard.is_empty()).then(|| guard.remove(0)));
            match next {
                Some(text) => Ok((
                    text,
                    TokenUsage {
                        input_tokens: 10,
                        output_tokens: 5,
                        total_tokens: 15,
                        cached_input_tokens: 0,
                        cache_creation_input_tokens: 0,
                    },
                )),
                None => Err("stub assistant exhausted".into()),
            }
        })
    }
}
