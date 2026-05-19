use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use serde_json::{Value, json};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::process::Child;
use tokio::sync::oneshot;
use tracing::{debug, warn};

use super::transport::CALL_TIMEOUT;

/// JSON-RPC 2.0 transport over a subprocess's stdin/stdout.
///
/// Spawns a child process, reads JSON-RPC responses from stdout in a
/// background task, and dispatches them to awaiting callers via oneshot
/// channels. Stderr is forwarded to tracing at debug level.
pub(crate) struct StdioTransport {
    writer: tokio::sync::Mutex<BufWriter<tokio::process::ChildStdin>>,
    pending: Arc<Mutex<HashMap<u64, oneshot::Sender<Value>>>>,
    next_id: AtomicU64,
    _reader_handle: tokio::task::JoinHandle<()>,
    _child: Child,
}

impl StdioTransport {
    pub(crate) async fn spawn(command: &str, args: &[String]) -> Result<Self, String> {
        let mut child = tokio::process::Command::new(command)
            .args(args)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .map_err(|e| format!("failed to spawn MCP server '{}': {}", command, e))?;

        let stdin = child.stdin.take().ok_or_else(|| "no stdin".to_string())?;
        let stdout = child.stdout.take().ok_or_else(|| "no stdout".to_string())?;
        let stderr = child.stderr.take().ok_or_else(|| "no stderr".to_string())?;

        let pending: Arc<Mutex<HashMap<u64, oneshot::Sender<Value>>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let pending_reader = Arc::clone(&pending);

        // Drain stderr to prevent the child from blocking.
        tokio::spawn(async move {
            let mut reader = BufReader::new(stderr);
            let mut line = String::new();
            loop {
                match reader.read_line(&mut line).await {
                    Ok(0) | Err(_) => break,
                    Ok(_) => {
                        let trimmed = line.trim();
                        if !trimmed.is_empty() {
                            debug!(target: "mcp:stderr", "{trimmed}");
                        }
                        line.clear();
                    }
                }
            }
        });

        // Background task: read JSON-RPC responses from stdout.
        let reader_handle = tokio::spawn(async move {
            let mut reader = BufReader::new(stdout);
            let mut line = String::new();
            loop {
                match reader.read_line(&mut line).await {
                    Ok(0) | Err(_) => break,
                    Ok(_) => {
                        let trimmed = line.trim();
                        if trimmed.is_empty() {
                            line.clear();
                            continue;
                        }
                        match serde_json::from_str::<Value>(trimmed) {
                            Ok(response) => {
                                if let Some(id) = response.get("id").and_then(|v| v.as_u64()) {
                                    if let Ok(mut map) = pending_reader.lock() {
                                        if let Some(tx) = map.remove(&id) {
                                            let _ = tx.send(response);
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                warn!(target: "mcp", "parse error: {e}, line: {trimmed}");
                            }
                        }
                        line.clear();
                    }
                }
            }
        });

        let writer = tokio::sync::Mutex::new(BufWriter::new(stdin));

        Ok(Self {
            writer,
            pending,
            next_id: AtomicU64::new(1),
            _reader_handle: reader_handle,
            _child: child,
        })
    }

    pub(crate) async fn initialize(&self) -> Result<Value, String> {
        let result = self
            .call_raw(
                "initialize",
                json!({
                    "protocolVersion": "2025-03-26",
                    "capabilities": {},
                    "clientInfo": { "name": "rica", "version": "0.1.0" }
                }),
            )
            .await?;

        {
            let mut writer = self.writer.lock().await;
            let notification = json!({
                "jsonrpc": "2.0",
                "method": "notifications/initialized"
            });
            let encoded =
                serde_json::to_string(&notification).map_err(|e| format!("serialize: {e}"))?;
            writer
                .write_all(encoded.as_bytes())
                .await
                .map_err(|e| format!("write: {e}"))?;
            writer
                .write_all(b"\n")
                .await
                .map_err(|e| format!("write: {e}"))?;
            writer.flush().await.map_err(|e| format!("flush: {e}"))?;
        }

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
        let (resp_tx, resp_rx) = oneshot::channel();

        {
            let mut map = self.pending.lock().unwrap_or_else(|e| e.into_inner());
            map.insert(id, resp_tx);
        }

        let request = json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params
        });

        {
            let mut writer = self.writer.lock().await;
            let encoded = serde_json::to_string(&request).map_err(|e| format!("serialize: {e}"))?;
            writer
                .write_all(encoded.as_bytes())
                .await
                .map_err(|e| format!("write: {e}"))?;
            writer
                .write_all(b"\n")
                .await
                .map_err(|e| format!("write: {e}"))?;
            writer.flush().await.map_err(|e| format!("flush: {e}"))?;
        }

        let response: Value = tokio::time::timeout(CALL_TIMEOUT, resp_rx)
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
}
