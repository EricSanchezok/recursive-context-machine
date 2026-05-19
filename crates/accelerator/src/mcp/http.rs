use std::sync::atomic::{AtomicU64, Ordering};

use serde_json::{Value, json};

use super::transport::CALL_TIMEOUT;

/// JSON-RPC 2.0 transport over HTTP POST (Streamable HTTP).
///
/// Sends JSON-RPC requests as HTTP POST to the configured endpoint.
/// No subprocess, no background reader — each call is a simple
/// request-response cycle. Supports custom headers (Authorization, etc.).
pub(crate) struct HttpTransport {
    url: String,
    headers: Vec<(String, String)>,
    client: reqwest::Client,
    next_id: AtomicU64,
}

impl HttpTransport {
    pub(crate) fn new(url: String, headers: Vec<(String, String)>) -> Self {
        Self {
            url,
            headers,
            client: reqwest::Client::builder()
                .timeout(CALL_TIMEOUT)
                .build()
                .expect("reqwest Client::builder"),
            next_id: AtomicU64::new(1),
        }
    }

    pub(crate) async fn initialize(&self) -> Result<Value, String> {
        let result = self
            .call_raw(
                "initialize",
                json!({
                    "protocolVersion": "2025-03-26",
                    "capabilities": {},
                    "clientInfo": { "name": "rcm", "version": "0.1.0" }
                }),
            )
            .await?;

        let _ = self
            .post(&json!({
                "jsonrpc": "2.0",
                "method": "notifications/initialized"
            }))
            .await;

        Ok(result)
    }

    pub(crate) async fn list_tools(&self) -> Result<Vec<Value>, String> {
        let result: Value = self.call_raw("tools/list", json!({})).await?;
        let tools = result["tools"]
            .as_array()
            .ok_or_else(|| "tools/list returned non-array 'tools'".to_string())?;
        Ok(tools.clone())
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

        let response: Value = self.post(&request).await?;

        if let Some(error) = response.get("error") {
            return Err(format!(
                "MCP error ({}): {}",
                error["code"].as_i64().unwrap_or(0),
                error["message"].as_str().unwrap_or("unknown")
            ));
        }

        Ok(response["result"].clone())
    }

    async fn post(&self, body: &Value) -> Result<Value, String> {
        let mut req = self.client.post(&self.url).json(body);
        for (key, value) in &self.headers {
            req = req.header(key.as_str(), value.as_str());
        }

        let resp = req
            .send()
            .await
            .map_err(|e| format!("HTTP request failed: {e}"))?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(format!("HTTP {status}: {text}"));
        }

        resp.json::<Value>()
            .await
            .map_err(|e| format!("JSON parse error: {e}"))
    }
}
