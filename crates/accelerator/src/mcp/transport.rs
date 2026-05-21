use std::time::Duration;

use serde_json::Value;

use super::http::HttpTransport;
use super::sse::SseTransport;
use super::stdio::StdioTransport;

pub(crate) const CALL_TIMEOUT: Duration = Duration::from_secs(120);
pub(crate) const PROTOCOL_VERSION: &str = "2025-06-18";

pub(crate) enum Transport {
    Stdio(Box<StdioTransport>),
    Http(HttpTransport),
    Sse(Box<SseTransport>),
}

impl Transport {
    pub(crate) async fn initialize(&self) -> Result<Value, String> {
        match self {
            Self::Stdio(transport) => transport.initialize().await,
            Self::Http(transport) => transport.initialize().await,
            Self::Sse(transport) => transport.initialize().await,
        }
    }

    pub(crate) async fn list_tools(&self) -> Result<Vec<Value>, String> {
        match self {
            Self::Stdio(transport) => transport.list_tools().await,
            Self::Http(transport) => transport.list_tools().await,
            Self::Sse(transport) => transport.list_tools().await,
        }
    }

    pub(crate) async fn call(&self, method: &str, params: Value) -> Result<Value, String> {
        match self {
            Self::Stdio(transport) => transport.call(method, params).await,
            Self::Http(transport) => transport.call(method, params).await,
            Self::Sse(transport) => transport.call(method, params).await,
        }
    }
}

pub(crate) fn find_sse_response(stream: &str, expected_id: Option<u64>) -> Result<Value, String> {
    for event in parse_sse_events(stream) {
        let Some(data) = event.data else {
            continue;
        };
        let value = serde_json::from_str::<Value>(&data)
            .map_err(|error| format!("SSE JSON parse error: {error}"))?;
        if expected_id.is_none() || value.get("id").and_then(|id| id.as_u64()) == expected_id {
            return Ok(value);
        }
    }
    Err("SSE stream did not contain the expected JSON-RPC response".to_string())
}

pub(crate) fn parse_sse_events(stream: &str) -> Vec<SseEvent> {
    let mut events = Vec::new();
    let mut event = SseEvent::default();
    for line in stream.lines() {
        let line = line.trim_end_matches('\r');
        if line.is_empty() {
            if event.event.is_some() || event.data.is_some() {
                events.push(event);
                event = SseEvent::default();
            }
            continue;
        }
        if let Some(value) = line.strip_prefix("event:") {
            event.event = Some(value.trim().to_string());
        } else if let Some(value) = line.strip_prefix("data:") {
            let value = value.trim_start();
            event.data.get_or_insert_with(String::new).push_str(value);
        }
    }
    if event.event.is_some() || event.data.is_some() {
        events.push(event);
    }
    events
}

#[derive(Default)]
pub(crate) struct SseEvent {
    pub event: Option<String>,
    pub data: Option<String>,
}
