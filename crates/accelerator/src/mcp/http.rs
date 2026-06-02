use std::sync::atomic::{AtomicU64, Ordering};

use serde_json::{Value, json};
use tokio::sync::Mutex;

use super::transport::{CALL_TIMEOUT, PROTOCOL_VERSION, find_sse_response};

pub(crate) struct HttpTransport {
    url: String,
    headers: Vec<(String, String)>,
    client: reqwest::Client,
    next_id: AtomicU64,
    session_id: Mutex<Option<String>>,
}

impl HttpTransport {
    pub(crate) fn new(url: String, headers: Vec<(String, String)>) -> Result<Self, String> {
        let client = reqwest::Client::builder()
            .timeout(CALL_TIMEOUT)
            .pool_max_idle_per_host(0)
            .build()
            .map_err(|error| format!("failed to build HTTP MCP client: {error}"))?;
        Ok(Self {
            url,
            headers,
            client,
            next_id: AtomicU64::new(1),
            session_id: Mutex::new(None),
        })
    }

    pub(crate) async fn initialize(&self) -> Result<Value, String> {
        let result = self
            .call_raw(
                "initialize",
                json!({
                    "protocolVersion": PROTOCOL_VERSION,
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
        let request = json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params
        });
        self.post(&request, Some(id))
            .await?
            .ok_or_else(|| format!("MCP request '{method}' returned no response body"))
    }

    async fn notify(&self, method: &str, params: Value) -> Result<(), String> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params
        });
        self.post(&request, None).await.map(|_| ())
    }

    async fn post(&self, body: &Value, expected_id: Option<u64>) -> Result<Option<Value>, String> {
        let mut request = self
            .client
            .post(&self.url)
            .header("Accept", "application/json, text/event-stream")
            .header("MCP-Protocol-Version", PROTOCOL_VERSION)
            .json(body);
        for (key, value) in &self.headers {
            request = request.header(key.as_str(), value.as_str());
        }
        if let Some(session_id) = self.session_id.lock().await.as_deref() {
            request = request.header("Mcp-Session-Id", session_id);
        }

        let response = request
            .send()
            .await
            .map_err(|error| format!("HTTP request failed: {error}"))?;
        if let Some(session_id) = response.headers().get("Mcp-Session-Id") {
            let session_id = session_id
                .to_str()
                .map_err(|error| format!("invalid MCP session id header: {error}"))?
                .to_string();
            *self.session_id.lock().await = Some(session_id);
        }
        let status = response.status();
        if status == reqwest::StatusCode::ACCEPTED {
            return Ok(None);
        }
        if !status.is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(format!("HTTP {status}: {text}"));
        }

        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("")
            .to_string();
        let text = response
            .text()
            .await
            .map_err(|error| format!("HTTP body read failed: {error}"))?;
        if text.trim().is_empty() {
            return Ok(None);
        }
        let response = if content_type.starts_with("text/event-stream") {
            find_sse_response(&text, expected_id)?
        } else {
            serde_json::from_str::<Value>(&text)
                .map_err(|error| format!("JSON parse error: {error}"))?
        };
        if let Some(error) = response.get("error") {
            return Err(format!(
                "MCP error ({}): {}",
                error["code"].as_i64().unwrap_or(0),
                error["message"].as_str().unwrap_or("unknown")
            ));
        }
        Ok(Some(response["result"].clone()))
    }
}
