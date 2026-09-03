use std::collections::HashMap;
use std::sync::mpsc;

use tracing::Subscriber;
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;

#[derive(Debug, Clone)]
pub struct HookEvent {
    pub source: Option<ComponentMeta>,
    pub kind: HookKind,
}

#[derive(Debug, Clone)]
pub enum HookKind {
    Graph(GraphEvent),
    Component(ComponentEvent),
    Machine(MachineEvent),
    Completion(CompletionEvent),
    Tool(ToolEvent),
    Fragment(FragmentEvent),
    Resource(ResourceEvent),
}

#[derive(Debug, Clone)]
pub enum GraphEvent {
    Start {
        graph: String,
    },
    Done {
        graph: String,
    },
    FrontierStart {
        graph: String,
        frontier: u64,
        count: usize,
    },
    FrontierDone {
        graph: String,
        frontier: u64,
        count: usize,
    },
}

#[derive(Debug, Clone)]
pub enum ComponentEvent {
    Start(ComponentMeta),
    Done(ComponentMeta),
    Skipped(ComponentMeta),
}

#[derive(Debug, Clone, Default)]
pub struct ComponentMeta {
    pub graph: String,
    pub name: String,
    pub index: usize,
    pub kind: String,
    pub frontier: Option<u64>,
}

#[derive(Debug, Clone)]
pub enum MachineEvent {
    Start,
    Halt { step: u64 },
    Done,
}

#[derive(Debug, Clone)]
pub enum CompletionEvent {
    Start,
    End {
        fragments: usize,
        input_tokens: u64,
        output_tokens: u64,
        total_tokens: u64,
        cached_input_tokens: u64,
        cache_creation_input_tokens: u64,
        outcome: Option<String>,
        http_status: Option<u16>,
        failure_kind: Option<String>,
        retryable: Option<bool>,
        duration_ms: Option<u64>,
    },
}

#[derive(Debug, Clone)]
pub enum ToolEvent {
    Call {
        call_id: String,
        tool: String,
        arguments: String,
    },
    Result {
        call_id: String,
        tool: String,
        result_len: usize,
        duration: String,
    },
    Error {
        call_id: String,
        tool: String,
        error: String,
        retryable: bool,
    },
}

#[derive(Debug, Clone)]
pub enum FragmentEvent {
    Appended(FragmentMeta),
    Taken(FragmentMeta),
    Inserted {
        meta: FragmentMeta,
        after: u64,
    },
    Replaced(FragmentMeta),
    Removed {
        id: u64,
    },
    Swapped {
        first: u64,
        second: u64,
    },
    /// v2 document model: one cell relocated after another (Edit Move op).
    Moved {
        id: u64,
        after: u64,
    },
    /// v2 inbox consumption (Edit Inbox content source). Carries the
    /// call_id when the item was addressed, else FIFO.
    Consumed {
        call_id: String,
    },
}

#[derive(Debug, Clone)]
pub struct FragmentMeta {
    pub id: u64,
    pub step: u64,
    pub role: String,
    pub kind: String,
    pub tag: String,
    pub preview: String,
}

#[derive(Debug, Clone)]
pub enum ResourceEvent {
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
    fn on_new_span(
        &self,
        attrs: &tracing::span::Attributes<'_>,
        id: &tracing::Id,
        ctx: Context<'_, S>,
    ) {
        let Some(span) = ctx.span(id) else {
            return;
        };
        let mut fields = HookFields::default();
        attrs.record(&mut fields);
        span.extensions_mut().insert(fields);
    }

    fn on_record(&self, id: &tracing::Id, values: &tracing::span::Record<'_>, ctx: Context<'_, S>) {
        let Some(span) = ctx.span(id) else {
            return;
        };
        let mut fields = span
            .extensions_mut()
            .remove::<HookFields>()
            .unwrap_or_default();
        values.record(&mut fields);
        span.extensions_mut().insert(fields);
    }

    fn on_event(&self, event: &tracing::Event<'_>, ctx: Context<'_, S>) {
        if event.metadata().target() != "hook" {
            return;
        }

        let mut fields = HookFields::default();
        if let Some(scope) = ctx.event_scope(event) {
            // Walk innermost span -> root so the *nearest* component identity
            // wins. With `merge_missing` (keep-first) a nested component — e.g.
            // a scout inside the `discovery` composite, or any sub-graph node —
            // then attributes its completion/tool/fragment events to itself,
            // not to the enclosing composite. Walking `from_root()` instead lets
            // the outer composite's fields win and lumps every child's work onto
            // the parent tape, so children render with 0 cells and parallel
            // sub-nodes look like one sequential node.
            for span in scope {
                if let Some(span_fields) = span.extensions().get::<HookFields>() {
                    fields.merge_missing(span_fields);
                }
            }
        }
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
    fn merge_missing(&mut self, other: &HookFields) {
        for (name, value) in &other.values {
            self.values
                .entry(name.clone())
                .or_insert_with(|| value.clone());
        }
    }

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

    fn source_meta(&self) -> Option<ComponentMeta> {
        let name = self.string("component")?;
        Some(ComponentMeta {
            graph: self.string("graph").unwrap_or_default(),
            name,
            index: self.usize("component_index").unwrap_or(0),
            kind: self.string("component_kind").unwrap_or_default(),
            frontier: self.u64("frontier"),
        })
    }

    fn component_meta(&self) -> ComponentMeta {
        ComponentMeta {
            graph: self.string("graph").unwrap_or_default(),
            name: self.string("component").unwrap_or_default(),
            index: self.usize("component_index").unwrap_or(0),
            kind: self.string("component_kind").unwrap_or_default(),
            frontier: self.u64("frontier"),
        }
    }

    fn fragment_meta(&self) -> FragmentMeta {
        FragmentMeta {
            id: self.u64("id").unwrap_or(0),
            step: self.u64("step").unwrap_or(0),
            role: self.string("role").unwrap_or_default(),
            kind: self.string("kind").unwrap_or_default(),
            tag: self.string("tag").unwrap_or_default(),
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
        let kind = match fields.string("event")?.as_str() {
            "graph_start" => HookKind::Graph(GraphEvent::Start {
                graph: fields.string("graph").unwrap_or_default(),
            }),
            "graph_done" => HookKind::Graph(GraphEvent::Done {
                graph: fields.string("graph").unwrap_or_default(),
            }),
            "frontier_start" => HookKind::Graph(GraphEvent::FrontierStart {
                graph: fields.string("graph").unwrap_or_default(),
                frontier: fields.u64("frontier").unwrap_or(0),
                count: fields.usize("count").unwrap_or(0),
            }),
            "frontier_done" => HookKind::Graph(GraphEvent::FrontierDone {
                graph: fields.string("graph").unwrap_or_default(),
                frontier: fields.u64("frontier").unwrap_or(0),
                count: fields.usize("count").unwrap_or(0),
            }),
            "component_start" => {
                HookKind::Component(ComponentEvent::Start(fields.component_meta()))
            }
            "component_done" => HookKind::Component(ComponentEvent::Done(fields.component_meta())),
            "component_skipped" => {
                HookKind::Component(ComponentEvent::Skipped(fields.component_meta()))
            }
            "machine_start" => HookKind::Machine(MachineEvent::Start),
            "halt" => HookKind::Machine(MachineEvent::Halt {
                step: fields.u64("step").unwrap_or(0),
            }),
            "done" => HookKind::Machine(MachineEvent::Done),
            "completion_start" => HookKind::Completion(CompletionEvent::Start),
            "completion_end" => HookKind::Completion(CompletionEvent::End {
                fragments: fields.usize("fragments").unwrap_or(0),
                input_tokens: fields.u64("input_tokens").unwrap_or(0),
                output_tokens: fields.u64("output_tokens").unwrap_or(0),
                total_tokens: fields.u64("total_tokens").unwrap_or(0),
                cached_input_tokens: fields.u64("cached_input_tokens").unwrap_or(0),
                cache_creation_input_tokens: fields.u64("cache_creation_input_tokens").unwrap_or(0),
                outcome: fields.string("outcome").filter(|value| !value.is_empty()),
                http_status: fields
                    .u64("http_status")
                    .filter(|value| *value > 0)
                    .and_then(|value| u16::try_from(value).ok()),
                failure_kind: fields
                    .string("failure_kind")
                    .filter(|value| !value.is_empty()),
                retryable: (fields.string("outcome").as_deref() == Some("failure"))
                    .then(|| fields.bool("retryable"))
                    .flatten(),
                duration_ms: fields.u64("duration_ms"),
            }),
            "tool_call" => HookKind::Tool(ToolEvent::Call {
                call_id: fields.string("call_id").unwrap_or_default(),
                tool: fields.string("tool").unwrap_or_default(),
                arguments: fields.string("arguments").unwrap_or_default(),
            }),
            "tool_result" => HookKind::Tool(ToolEvent::Result {
                call_id: fields.string("call_id").unwrap_or_default(),
                tool: fields.string("tool").unwrap_or_default(),
                result_len: fields
                    .string("result")
                    .map(|result| result.len())
                    .unwrap_or(0),
                duration: fields.string("duration").unwrap_or_default(),
            }),
            "tool_error" => HookKind::Tool(ToolEvent::Error {
                call_id: fields.string("call_id").unwrap_or_default(),
                tool: fields.string("tool").unwrap_or_default(),
                error: fields.string("error").unwrap_or_default(),
                retryable: fields.bool("retryable").unwrap_or(true),
            }),
            "appended" => HookKind::Fragment(FragmentEvent::Appended(fields.fragment_meta())),
            "taken" => HookKind::Fragment(FragmentEvent::Taken(fields.fragment_meta())),
            "inserted" => HookKind::Fragment(FragmentEvent::Inserted {
                meta: fields.fragment_meta(),
                after: fields.u64("after").unwrap_or(0),
            }),
            "replaced" => HookKind::Fragment(FragmentEvent::Replaced(fields.fragment_meta())),
            "removed" => HookKind::Fragment(FragmentEvent::Removed {
                id: fields.u64("id").unwrap_or(0),
            }),
            "swapped" => HookKind::Fragment(FragmentEvent::Swapped {
                first: fields.u64("id1").unwrap_or(0),
                second: fields.u64("id2").unwrap_or(0),
            }),
            "moved" => HookKind::Fragment(FragmentEvent::Moved {
                id: fields.u64("id").unwrap_or(0),
                after: fields.u64("after").unwrap_or(0),
            }),
            "consumed" => HookKind::Fragment(FragmentEvent::Consumed {
                call_id: fields.string("call_id").unwrap_or_default(),
            }),
            "model" => HookKind::Resource(ResourceEvent::Model {
                name: fields.string("name").unwrap_or_default(),
            }),
            "activate" => HookKind::Resource(ResourceEvent::Activate {
                name: fields.string("name").unwrap_or_default(),
            }),
            "deactivate" => HookKind::Resource(ResourceEvent::Deactivate {
                name: fields.string("name").unwrap_or_default(),
            }),
            _ => return None,
        };
        Some(Self {
            source: fields.source_meta(),
            kind,
        })
    }
}
