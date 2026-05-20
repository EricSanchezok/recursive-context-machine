//! Minimal LSP client for diagnostics.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};
use std::time::Duration;

use serde_json::{Value, json};
use tokio::io::BufReader;
use tokio::process::{Child, ChildStdin, Command};
use tokio::sync::{Mutex, oneshot};
use tracing::{debug, warn};

use super::diagnostics::{Diagnostic, DiagnosticStore};
use super::servers::ServerSpec;
use super::transport::{read_message, write_message};
use super::uri::path_to_uri;

const INITIALIZE_TIMEOUT: Duration = Duration::from_secs(45);

#[derive(Debug, Clone)]
struct OpenDocument {
    version: i32,
}

pub struct LspClient {
    server: ServerSpec,
    root: PathBuf,
    stdin: Mutex<ChildStdin>,
    _child: Mutex<Child>,
    next_id: AtomicI64,
    pending: Mutex<HashMap<i64, oneshot::Sender<Result<Value, String>>>>,
    diagnostics: DiagnosticStore,
    documents: Mutex<HashMap<PathBuf, OpenDocument>>,
    alive: AtomicBool,
}

impl LspClient {
    pub async fn start(server: ServerSpec, root: PathBuf) -> Result<Arc<Self>, String> {
        debug!(server = server.id, root = %root.display(), "lsp spawn");

        let mut command = Command::new(server.command);
        command.args(server.args);
        command.current_dir(&root);
        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .kill_on_drop(true);

        let mut child = command
            .spawn()
            .map_err(|error| format!("failed to spawn {}: {error}", server.command))?;

        let stdin = child.stdin.take().ok_or("LSP server stdin was not piped")?;
        let stdout = child
            .stdout
            .take()
            .ok_or("LSP server stdout was not piped")?;

        let client = Arc::new(Self {
            server,
            root,
            stdin: Mutex::new(stdin),
            _child: Mutex::new(child),
            next_id: AtomicI64::new(1),
            pending: Mutex::new(HashMap::new()),
            diagnostics: DiagnosticStore::new(),
            documents: Mutex::new(HashMap::new()),
            alive: AtomicBool::new(true),
        });

        client.spawn_reader(stdout);
        client.initialize().await?;
        Ok(client)
    }

    pub fn diagnostics(&self, path: &Path) -> Vec<Diagnostic> {
        self.diagnostics.get(path)
    }

    pub async fn touch_file(&self, path: &Path, wait: bool) -> Result<Vec<Diagnostic>, String> {
        if !self.alive.load(Ordering::Relaxed) {
            return Err(format!("{} is not running", self.server.id));
        }

        let text = tokio::fs::read_to_string(path)
            .await
            .map_err(|error| format!("failed to read {} for LSP: {error}", path.display()))?;
        let uri = path_to_uri(path)?;

        let version = {
            let mut docs = self.documents.lock().await;
            let entry = docs
                .entry(path.to_path_buf())
                .or_insert(OpenDocument { version: 0 });
            entry.version += 1;
            entry.version
        };

        if version == 1 {
            self.notify(
                "textDocument/didOpen",
                json!({
                    "textDocument": {
                        "uri": uri,
                        "languageId": self.server.language_id,
                        "version": version,
                        "text": text,
                    }
                }),
            )
            .await?;
        } else {
            self.notify(
                "textDocument/didChange",
                json!({
                    "textDocument": { "uri": uri, "version": version },
                    "contentChanges": [{ "text": text }]
                }),
            )
            .await?;
        }

        if wait {
            self.wait_for_diagnostics(path, Duration::from_secs(3))
                .await;
        }

        Ok(self.diagnostics(path))
    }

    async fn initialize(&self) -> Result<(), String> {
        let root_uri = path_to_uri(&self.root)?;
        let params = json!({
            "processId": std::process::id(),
            "rootUri": root_uri,
            "workspaceFolders": [{ "uri": root_uri, "name": "workspace" }],
            "capabilities": {
                "window": { "workDoneProgress": true },
                "workspace": { "configuration": true, "workspaceFolders": true },
                "textDocument": {
                    "synchronization": { "didOpen": true, "didChange": true },
                    "publishDiagnostics": {}
                }
            }
        });

        self.request_with_timeout("initialize", params, INITIALIZE_TIMEOUT)
            .await?;

        self.notify("initialized", json!({})).await
    }

    async fn request_with_timeout(
        &self,
        method: &str,
        params: Value,
        timeout: Duration,
    ) -> Result<Value, String> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let (tx, rx) = oneshot::channel();
        self.pending.lock().await.insert(id, tx);

        let message = json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params,
        });

        if let Err(error) = self.write(&message).await {
            self.pending.lock().await.remove(&id);
            return Err(error);
        }

        match tokio::time::timeout(timeout, rx).await {
            Ok(Ok(response)) => response,
            Ok(Err(_)) => Err(format!("LSP request {method} response channel closed")),
            Err(_) => {
                self.pending.lock().await.remove(&id);
                Err(format!("LSP request {method} timed out"))
            }
        }
    }

    async fn notify(&self, method: &str, params: Value) -> Result<(), String> {
        let message = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
        });
        self.write(&message).await
    }

    async fn write(&self, message: &Value) -> Result<(), String> {
        let mut stdin = self.stdin.lock().await;
        write_message(&mut *stdin, message).await
    }

    async fn wait_for_diagnostics(&self, path: &Path, timeout: Duration) {
        let mut rx = self.diagnostics.subscribe();
        let target = path.to_path_buf();
        let debounce = Duration::from_millis(150);

        let wait = async move {
            loop {
                match rx.recv().await {
                    Ok(event) if event.path == target => break,
                    Ok(_) => {}
                    Err(_) => return,
                }
            }

            loop {
                match tokio::time::timeout(debounce, rx.recv()).await {
                    Ok(Ok(event)) if event.path == target => continue,
                    Ok(Ok(_)) => continue,
                    Ok(Err(_)) | Err(_) => break,
                }
            }
        };

        let _ = tokio::time::timeout(timeout, wait).await;
    }

    fn spawn_reader(self: &Arc<Self>, stdout: tokio::process::ChildStdout) {
        let client = Arc::clone(self);
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout);
            loop {
                let message = match read_message(&mut reader).await {
                    Ok(message) => message,
                    Err(error) => {
                        client.alive.store(false, Ordering::Relaxed);
                        warn!(server = client.server.id, ?error, "lsp reader stopped");
                        break;
                    }
                };
                client.handle_message(message).await;
            }
        });
    }

    async fn handle_message(&self, message: Value) {
        if let Some(id) = message.get("id").and_then(|value| value.as_i64()) {
            let response = if let Some(error) = message.get("error") {
                Err(format!("LSP error response: {error}"))
            } else {
                Ok(message.get("result").cloned().unwrap_or(Value::Null))
            };

            if let Some(tx) = self.pending.lock().await.remove(&id) {
                let _ = tx.send(response);
            }
            return;
        }

        if message.get("method").and_then(|value| value.as_str())
            == Some("textDocument/publishDiagnostics")
        {
            if let Some(params) = message.get("params") {
                if let Err(error) = self.diagnostics.update_from_notification(params) {
                    warn!(server = self.server.id, ?error, "invalid lsp diagnostics");
                }
            }
        }
    }
}
