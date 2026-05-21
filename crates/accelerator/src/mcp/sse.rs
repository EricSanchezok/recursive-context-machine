use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use futures_util::StreamExt;
use serde_json::{Value, json};
use tokio::sync::oneshot;
use tracing::warn;
use url::Url;

use super::transport::{CALL_TIMEOUT, parse_sse_events};

pub(crate) struct SseTransport {
    client: reqwest::Client,
    endpoint: String,
    headers: Vec<(String, String)>,
    pending: Arc<Mutex<HashMap<u64, oneshot::Sender<Value>>>>,
    next_id: AtomicU64,
    _reader_handle: tokio::task::JoinHandle<()>,
}

impl SseTransport {
    pub(crate) async fn connect(
        url: String,
        headers: Vec<(String, String)>,
    ) -> Result<Self, String> {
        let client = reqwest::Client::builder()
            .timeout(CALL_TIMEOUT)
            .build()
            .map_err(|error| format!("failed to build SSE MCP client: {error}"))?;
        let pending: Arc<Mutex<HashMap<u64, oneshot::Sender<Value>>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let mut request = client.get(&url).header("Accept", "text/event-stream");
        for (key, value) in &headers {
            request = request.header(key.as_str(), value.as_str());
        }
        let response = request
            .send()
            .await
            .map_err(|error| format!("SSE connection failed: {error}"))?;
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(format!("SSE HTTP {status}: {text}"));
        }

        let mut stream = response.bytes_stream();
        let mut buffer = String::new();
        let endpoint = loop {
            let Some(chunk) = stream.next().await else {
                return Err("SSE stream closed before endpoint event".to_string());
            };
            let chunk = chunk.map_err(|error| format!("SSE read failed: {error}"))?;
            buffer.push_str(&String::from_utf8_lossy(&chunk));
            if let Some(endpoint) = take_endpoint(&mut buffer, &url)? {
                break endpoint;
            }
        };

        let reader_pending = Arc::clone(&pending);
        let reader_handle = tokio::spawn(async move {
            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(bytes) => {
                        buffer.push_str(&String::from_utf8_lossy(&bytes));
                        dispatch_events(&mut buffer, &reader_pending);
                    }
                    Err(error) => {
                        warn!(target: "mcp", ?error, "SSE stream read failed");
                        break;
                    }
                }
            }
        });

        Ok(Self {
            client,
            endpoint,
            headers,
            pending,
            next_id: AtomicU64::new(1),
            _reader_handle: reader_handle,
        })
    }

    pub(crate) async fn initialize(&self) -> Result<Value, String> {
        let result = self
            .call_raw(
                "initialize",
                json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {},
                    "clientInfo": { "name": "rcm", "version": "0.1.0" }
                }),
            )
            .await?;
        self.notify("notifications/initialized", json!({})).await?;
        Ok(result)
    }

    pub(crate) async fn list_tools(&self) -> Result<Vec<Value>, String> {
        let mut cursor = None;
        let mut tools = Vec::new();
        loop {
            let params = cursor
                .take()
                .map(|cursor| json!({ "cursor": cursor }))
                .unwrap_or_else(|| json!({}));
            let result = self.call_raw("tools/list", params).await?;
            let page = result["tools"]
                .as_array()
                .ok_or_else(|| "tools/list returned non-array 'tools'".to_string())?;
            tools.extend(page.iter().cloned());
            cursor = result
                .get("nextCursor")
                .and_then(|value| value.as_str())
                .map(ToOwned::to_owned);
            if cursor.is_none() {
                break;
            }
        }
        Ok(tools)
    }

    pub(crate) async fn call(&self, method: &str, params: Value) -> Result<Value, String> {
        self.call_raw(method, params).await
    }

    async fn call_raw(&self, method: &str, params: Value) -> Result<Value, String> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let (response_tx, response_rx) = oneshot::channel();
        self.pending
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .insert(id, response_tx);
        let request = json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params
        });
        self.post(&request).await?;
        let response = tokio::time::timeout(CALL_TIMEOUT, response_rx)
            .await
            .map_err(|_| format!("MCP '{method}' timed out after 120s"))?
            .map_err(|_| format!("MCP '{method}' cancelled (server closed)"))?;
        if let Some(error) = response.get("error") {
            return Err(format!(
                "MCP error ({}): {}",
                error["code"].as_i64().unwrap_or(0),
                error["message"].as_str().unwrap_or("unknown")
            ));
        }
        Ok(response["result"].clone())
    }

    async fn notify(&self, method: &str, params: Value) -> Result<(), String> {
        self.post(&json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params
        }))
        .await
    }

    async fn post(&self, body: &Value) -> Result<(), String> {
        let mut request = self.client.post(&self.endpoint).json(body);
        for (key, value) in &self.headers {
            request = request.header(key.as_str(), value.as_str());
        }
        let response = request
            .send()
            .await
            .map_err(|error| format!("SSE POST failed: {error}"))?;
        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            Err(format!("SSE POST HTTP {status}: {text}"))
        }
    }
}

fn take_endpoint(buffer: &mut String, base_url: &str) -> Result<Option<String>, String> {
    let Some(split) = buffer.find("\n\n") else {
        return Ok(None);
    };
    let chunk = buffer[..split + 2].to_string();
    buffer.replace_range(..split + 2, "");
    for event in parse_sse_events(&chunk) {
        if event.event.as_deref() == Some("endpoint") {
            let data = event
                .data
                .ok_or_else(|| "SSE endpoint event missing data".to_string())?;
            return Ok(Some(resolve_endpoint(base_url, data.trim())?));
        }
    }
    Ok(None)
}

fn dispatch_events(
    buffer: &mut String,
    pending: &Arc<Mutex<HashMap<u64, oneshot::Sender<Value>>>>,
) {
    while let Some(split) = buffer.find("\n\n") {
        let chunk = buffer[..split + 2].to_string();
        buffer.replace_range(..split + 2, "");
        for event in parse_sse_events(&chunk) {
            if event.event.as_deref() != Some("message") {
                continue;
            }
            let Some(data) = event.data else {
                continue;
            };
            match serde_json::from_str::<Value>(&data) {
                Ok(value) => {
                    if let Some(id) = value.get("id").and_then(|value| value.as_u64())
                        && let Ok(mut pending) = pending.lock()
                        && let Some(sender) = pending.remove(&id)
                    {
                        let _ = sender.send(value);
                    }
                }
                Err(error) => warn!(target: "mcp", ?error, "failed to parse SSE message"),
            }
        }
    }
}

fn resolve_endpoint(base_url: &str, endpoint: &str) -> Result<String, String> {
    if endpoint.starts_with("http://") || endpoint.starts_with("https://") {
        return Ok(endpoint.to_string());
    }
    let base = Url::parse(base_url).map_err(|error| format!("invalid SSE URL: {error}"))?;
    base.join(endpoint)
        .map(|url| url.to_string())
        .map_err(|error| format!("invalid SSE endpoint: {error}"))
}
