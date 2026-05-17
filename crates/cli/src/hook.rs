use std::sync::mpsc;

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
        kind: String,
        preview: String,
    },
    FragmentTaken {
        id: u64,
        role: String,
        kind: String,
        preview: String,
    },
    FragmentInserted {
        id: u64,
        role: String,
        kind: String,
        preview: String,
    },
    FragmentReplaced {
        id: u64,
        role: String,
        kind: String,
        preview: String,
    },
    FragmentRemoved {
        id: u64,
    },
    FragmentsSwapped {
        first: u64,
        second: u64,
    },
    Model {
        name: String,
    },
    Activate {
        name: String,
    },
    Deactivate {
        name: String,
    },
    Done,
}

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

#[derive(Default)]
struct Extract {
    event: Option<String>,
    id: Option<u64>,
    id1: Option<u64>,
    id2: Option<u64>,
    role: Option<String>,
    kind: Option<String>,
    preview: Option<String>,
    tool: Option<String>,
    arguments: Option<String>,
    result: Option<String>,
    error: Option<String>,
    duration: Option<String>,
    name: Option<String>,
    fragments: Option<usize>,
    round: Option<u32>,
    retryable: Option<bool>,
}

impl tracing::field::Visit for Extract {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        match field.name() {
            "event" => self.event = Some(value.to_string()),
            "role" if !value.is_empty() => self.role = Some(value.to_string()),
            "kind" if !value.is_empty() => self.kind = Some(value.to_string()),
            "preview" => self.preview = Some(value.to_string()),
            "tool" => self.tool = Some(value.to_string()),
            "arguments" => self.arguments = Some(value.to_string()),
            "result" => self.result = Some(value.to_string()),
            "error" => self.error = Some(value.to_string()),
            "duration" => self.duration = Some(value.to_string()),
            "name" => self.name = Some(value.to_string()),
            _ => {}
        }
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        match field.name() {
            "id" => self.id = Some(value),
            "id1" => self.id1 = Some(value),
            "id2" => self.id2 = Some(value),
            "fragments" => self.fragments = Some(value as usize),
            "round" => self.round = Some(value as u32),
            _ => {}
        }
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        if value < 0 {
            return;
        }
        self.record_u64(field, value as u64);
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

        let mut fields = Extract::default();
        event.record(&mut fields);

        let hook = match fields.event.as_deref() {
            Some("machine_start") => HookEvent::MachineStart,
            Some("halt") => HookEvent::Halt {
                round: fields.round.unwrap_or(0),
            },
            Some("completion_start") => HookEvent::CompletionStart,
            Some("completion_end") => HookEvent::CompletionEnd {
                fragments: fields.fragments.unwrap_or(0),
            },
            Some("tool_call") => HookEvent::ToolCall {
                tool: fields.tool.unwrap_or_default(),
                arguments: fields.arguments.unwrap_or_default(),
            },
            Some("tool_result") => HookEvent::ToolResult {
                tool: fields.tool.unwrap_or_default(),
                result_len: fields
                    .result
                    .as_ref()
                    .map(|result| result.len())
                    .unwrap_or(0),
                duration: fields.duration.unwrap_or_default(),
            },
            Some("tool_error") => HookEvent::ToolError {
                tool: fields.tool.unwrap_or_default(),
                error: fields.error.unwrap_or_default(),
                retryable: fields.retryable.unwrap_or(true),
            },
            Some("appended") => HookEvent::FragmentAppended {
                id: fields.id.unwrap_or(0),
                role: fields.role.unwrap_or_default(),
                kind: fields.kind.unwrap_or_default(),
                preview: fields.preview.unwrap_or_default(),
            },
            Some("taken") => HookEvent::FragmentTaken {
                id: fields.id.unwrap_or(0),
                role: fields.role.unwrap_or_default(),
                kind: fields.kind.unwrap_or_default(),
                preview: fields.preview.unwrap_or_default(),
            },
            Some("inserted") => HookEvent::FragmentInserted {
                id: fields.id.unwrap_or(0),
                role: fields.role.unwrap_or_default(),
                kind: fields.kind.unwrap_or_default(),
                preview: fields.preview.unwrap_or_default(),
            },
            Some("replaced") => HookEvent::FragmentReplaced {
                id: fields.id.unwrap_or(0),
                role: fields.role.unwrap_or_default(),
                kind: fields.kind.unwrap_or_default(),
                preview: fields.preview.unwrap_or_default(),
            },
            Some("removed") => HookEvent::FragmentRemoved {
                id: fields.id.unwrap_or(0),
            },
            Some("swapped") => HookEvent::FragmentsSwapped {
                first: fields.id1.unwrap_or(0),
                second: fields.id2.unwrap_or(0),
            },
            Some("model") => HookEvent::Model {
                name: fields.name.unwrap_or_default(),
            },
            Some("activate") => HookEvent::Activate {
                name: fields.name.unwrap_or_default(),
            },
            Some("deactivate") => HookEvent::Deactivate {
                name: fields.name.unwrap_or_default(),
            },
            Some("done") => HookEvent::Done,
            _ => return,
        };

        let _ = self.tx.send(hook);
    }
}
