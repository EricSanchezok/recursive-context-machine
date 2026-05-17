use std::sync::mpsc;

/// Events emitted by the machine hook layer for the visualizer.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub(crate) enum HookEvent {
    MachineStart,
    Halt {
        round: u32,
    },
    CompletionStart,
    CompletionEnd {
        fragments: usize,
    },
    ToolCall {
        tool: String,
        arguments: String,
    },
    ToolResult {
        tool: String,
        result_len: usize,
        duration: String,
    },
    ToolError {
        tool: String,
        error: String,
        retryable: bool,
    },
    FragmentAppended {
        id: u64,
        role: String,
        preview: String,
    },
    FragmentTaken {
        id: u64,
        role: String,
        preview: String,
    },
    FragmentInserted {
        id: u64,
        role: String,
        preview: String,
    },
    FragmentReplaced {
        id: u64,
        role: String,
        preview: String,
    },
    Done,
}

/// Create a tracing-subscriber layer that intercepts `target: "hook"` events
/// and forwards them to the given sender.
pub(crate) fn hook_layer(
    tx: mpsc::Sender<HookEvent>,
) -> impl tracing_subscriber::layer::Layer<tracing_subscriber::registry::Registry> {
    HookLayer { tx }
}

use tracing::Subscriber;
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;

struct HookLayer {
    tx: mpsc::Sender<HookEvent>,
}

struct Extract {
    event: Option<String>,
    id: Option<u64>,
    role: Option<String>,
    preview: Option<String>,
    tool: Option<String>,
    arguments: Option<String>,
    result: Option<String>,
    error: Option<String>,
    duration: Option<String>,
    fragments: Option<usize>,
    round: Option<u32>,
    retryable: Option<bool>,
}

impl tracing::field::Visit for Extract {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        match field.name() {
            "event" => self.event = Some(value.to_string()),
            "role" if !value.is_empty() => self.role = Some(value.to_string()),
            "preview" => self.preview = Some(value.to_string()),
            "tool" => self.tool = Some(value.to_string()),
            "arguments" => self.arguments = Some(value.to_string()),
            "result" => self.result = Some(value.to_string()),
            "error" => self.error = Some(value.to_string()),
            "duration" => self.duration = Some(value.to_string()),
            _ => {}
        }
    }
    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        match field.name() {
            "id" => self.id = Some(value),
            "fragments" => self.fragments = Some(value as usize),
            "round" => self.round = Some(value as u32),
            _ => {}
        }
    }
    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        if field.name() == "id" {
            self.id = Some(value as u64);
        }
    }
    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        if field.name() == "retryable" {
            self.retryable = Some(value);
        }
    }
    fn record_debug(&mut self, _field: &tracing::field::Field, _value: &dyn std::fmt::Debug) {}
}

impl<S: Subscriber + for<'a> LookupSpan<'a>> tracing_subscriber::layer::Layer<S> for HookLayer {
    fn on_event(&self, event: &tracing::Event<'_>, _ctx: Context<'_, S>) {
        if event.metadata().target() != "hook" {
            return;
        }

        let mut e = Extract {
            event: None,
            id: None,
            role: None,
            preview: None,
            tool: None,
            arguments: None,
            result: None,
            error: None,
            duration: None,
            fragments: None,
            round: None,
            retryable: None,
        };
        event.record(&mut e);

        let name = match e.event.as_deref() {
            Some(n) => n,
            None => return,
        };

        let hook = match name {
            "machine_start" => HookEvent::MachineStart,
            "halt" => HookEvent::Halt {
                round: e.round.unwrap_or(0),
            },
            "completion_start" => HookEvent::CompletionStart,
            "completion_end" => HookEvent::CompletionEnd {
                fragments: e.fragments.unwrap_or(0),
            },
            "tool_call" => HookEvent::ToolCall {
                tool: e.tool.unwrap_or_default(),
                arguments: e.arguments.unwrap_or_default(),
            },
            "tool_result" => HookEvent::ToolResult {
                tool: e.tool.unwrap_or_default(),
                result_len: e.result.as_ref().map(|r| r.len()).unwrap_or(0),
                duration: e.duration.unwrap_or_default(),
            },
            "tool_error" => HookEvent::ToolError {
                tool: e.tool.unwrap_or_default(),
                error: e.error.unwrap_or_default(),
                retryable: e.retryable.unwrap_or(true),
            },
            "appended" => HookEvent::FragmentAppended {
                id: e.id.unwrap_or(0),
                role: e.role.unwrap_or_default(),
                preview: e.preview.unwrap_or_default(),
            },
            "taken" => HookEvent::FragmentTaken {
                id: e.id.unwrap_or(0),
                role: e.role.unwrap_or_default(),
                preview: e.preview.unwrap_or_default(),
            },
            "inserted" => HookEvent::FragmentInserted {
                id: e.id.unwrap_or(0),
                role: e.role.unwrap_or_default(),
                preview: e.preview.unwrap_or_default(),
            },
            "replaced" => HookEvent::FragmentReplaced {
                id: e.id.unwrap_or(0),
                role: e.role.unwrap_or_default(),
                preview: e.preview.unwrap_or_default(),
            },
            "done" => HookEvent::Done,
            _ => return,
        };

        let _ = self.tx.send(hook);
    }
}
