use std::collections::HashMap;
use std::sync::mpsc;

use tracing::Subscriber;
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;

#[derive(Debug, Clone)]
pub(crate) enum HookEvent {
    Machine(MachineEvent),
    Completion(CompletionEvent),
    Tool(ToolEvent),
    Fragment(FragmentEvent),
    Resource(ResourceEvent),
}

#[derive(Debug, Clone)]
pub(crate) enum MachineEvent {
    Start,
    Halt { step: u64 },
    Done,
}

#[derive(Debug, Clone)]
pub(crate) enum CompletionEvent {
    Start,
    End { fragments: usize },
}

#[derive(Debug, Clone)]
pub(crate) enum ToolEvent {
    Call {
        tool: String,
        arguments: String,
    },
    Result {
        tool: String,
        result_len: usize,
        duration: String,
    },
    Error {
        tool: String,
        error: String,
        retryable: bool,
    },
}

#[derive(Debug, Clone)]
pub(crate) enum FragmentEvent {
    Appended(FragmentMeta),
    Taken(FragmentMeta),
    Inserted(FragmentMeta),
    Replaced(FragmentMeta),
    Removed { id: u64 },
    Swapped { first: u64, second: u64 },
}

#[derive(Debug, Clone)]
pub(crate) struct FragmentMeta {
    pub(crate) id: u64,
    pub(crate) role: String,
    pub(crate) kind: String,
    pub(crate) preview: String,
}

#[derive(Debug, Clone)]
pub(crate) enum ResourceEvent {
    Model { name: String },
    Activate { name: String },
    Deactivate { name: String },
}

pub(crate) fn hook_layer(
    tx: mpsc::Sender<HookEvent>,
) -> impl tracing_subscriber::layer::Layer<tracing_subscriber::registry::Registry> {
    HookLayer { tx }
}

struct HookLayer {
    tx: mpsc::Sender<HookEvent>,
}

impl<S: Subscriber + for<'a> LookupSpan<'a>> tracing_subscriber::layer::Layer<S> for HookLayer {
    fn on_event(&self, event: &tracing::Event<'_>, _ctx: Context<'_, S>) {
        if event.metadata().target() != "hook" {
            return;
        }

        let mut fields = HookFields::default();
        event.record(&mut fields);
        if let Some(event) = HookEvent::from_fields(&fields) {
            let _ = self.tx.send(event);
        }
    }
}

#[derive(Default)]
struct HookFields {
    values: HashMap<String, FieldValue>,
}

#[derive(Debug, Clone)]
enum FieldValue {
    String(String),
    U64(u64),
    Bool(bool),
    Debug(String),
}

impl HookFields {
    fn string(&self, name: &str) -> Option<String> {
        match self.values.get(name) {
            Some(FieldValue::String(value)) | Some(FieldValue::Debug(value)) => Some(value.clone()),
            Some(FieldValue::U64(value)) => Some(value.to_string()),
            Some(FieldValue::Bool(value)) => Some(value.to_string()),
            None => None,
        }
    }

    fn u64(&self, name: &str) -> Option<u64> {
        match self.values.get(name) {
            Some(FieldValue::U64(value)) => Some(*value),
            Some(FieldValue::String(value)) | Some(FieldValue::Debug(value)) => value.parse().ok(),
            _ => None,
        }
    }

    fn usize(&self, name: &str) -> Option<usize> {
        self.u64(name).map(|value| value as usize)
    }

    fn bool(&self, name: &str) -> Option<bool> {
        match self.values.get(name) {
            Some(FieldValue::Bool(value)) => Some(*value),
            Some(FieldValue::String(value)) | Some(FieldValue::Debug(value)) => value.parse().ok(),
            _ => None,
        }
    }

    fn fragment_meta(&self) -> FragmentMeta {
        FragmentMeta {
            id: self.u64("id").unwrap_or(0),
            role: self.string("role").unwrap_or_default(),
            kind: self.string("kind").unwrap_or_default(),
            preview: self.string("preview").unwrap_or_default(),
        }
    }
}

impl tracing::field::Visit for HookFields {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        self.values.insert(
            field.name().to_string(),
            FieldValue::String(value.to_string()),
        );
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        self.values
            .insert(field.name().to_string(), FieldValue::U64(value));
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        if value >= 0 {
            self.record_u64(field, value as u64);
        }
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        self.values
            .insert(field.name().to_string(), FieldValue::Bool(value));
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.values.insert(
            field.name().to_string(),
            FieldValue::Debug(format!("{value:?}")),
        );
    }
}

impl HookEvent {
    fn from_fields(fields: &HookFields) -> Option<Self> {
        match fields.string("event")?.as_str() {
            "machine_start" => Some(Self::Machine(MachineEvent::Start)),
            "halt" => Some(Self::Machine(MachineEvent::Halt {
                step: fields.u64("step").unwrap_or(0),
            })),
            "done" => Some(Self::Machine(MachineEvent::Done)),
            "completion_start" => Some(Self::Completion(CompletionEvent::Start)),
            "completion_end" => Some(Self::Completion(CompletionEvent::End {
                fragments: fields.usize("fragments").unwrap_or(0),
            })),
            "tool_call" => Some(Self::Tool(ToolEvent::Call {
                tool: fields.string("tool").unwrap_or_default(),
                arguments: fields.string("arguments").unwrap_or_default(),
            })),
            "tool_result" => Some(Self::Tool(ToolEvent::Result {
                tool: fields.string("tool").unwrap_or_default(),
                result_len: fields
                    .string("result")
                    .map(|result| result.len())
                    .unwrap_or(0),
                duration: fields.string("duration").unwrap_or_default(),
            })),
            "tool_error" => Some(Self::Tool(ToolEvent::Error {
                tool: fields.string("tool").unwrap_or_default(),
                error: fields.string("error").unwrap_or_default(),
                retryable: fields.bool("retryable").unwrap_or(true),
            })),
            "appended" => Some(Self::Fragment(FragmentEvent::Appended(
                fields.fragment_meta(),
            ))),
            "taken" => Some(Self::Fragment(FragmentEvent::Taken(fields.fragment_meta()))),
            "inserted" => Some(Self::Fragment(FragmentEvent::Inserted(
                fields.fragment_meta(),
            ))),
            "replaced" => Some(Self::Fragment(FragmentEvent::Replaced(
                fields.fragment_meta(),
            ))),
            "removed" => Some(Self::Fragment(FragmentEvent::Removed {
                id: fields.u64("id").unwrap_or(0),
            })),
            "swapped" => Some(Self::Fragment(FragmentEvent::Swapped {
                first: fields.u64("id1").unwrap_or(0),
                second: fields.u64("id2").unwrap_or(0),
            })),
            "model" => Some(Self::Resource(ResourceEvent::Model {
                name: fields.string("name").unwrap_or_default(),
            })),
            "activate" => Some(Self::Resource(ResourceEvent::Activate {
                name: fields.string("name").unwrap_or_default(),
            })),
            "deactivate" => Some(Self::Resource(ResourceEvent::Deactivate {
                name: fields.string("name").unwrap_or_default(),
            })),
            _ => None,
        }
    }
}
