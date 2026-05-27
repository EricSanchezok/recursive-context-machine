//! Minimal LSP client for diagnostics and queries.

use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};
use std::time::Duration;

use lsp_types::notification::{
    DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument, Exit, Initialized,
    Notification,
};
use lsp_types::request::{Request, Shutdown};
use lsp_types::{
    ClientCapabilities, DidChangeTextDocumentParams, DidCloseTextDocumentParams,
    DidOpenTextDocumentParams, InitializeParams, InitializedParams, PublishDiagnosticsParams,
    TextDocumentContentChangeEvent, TextDocumentIdentifier, TextDocumentItem,
    TextDocumentSyncClientCapabilities, VersionedTextDocumentIdentifier, WorkspaceFolder,
};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use tokio::io::BufReader;
use tokio::process::{Child, ChildStdin, Command};
use tokio::sync::{Mutex, oneshot};
use tracing::{debug, warn};

use super::diagnostics::{Diagnostic, DiagnosticSnapshot, DiagnosticStore};
use super::servers::ServerSpec;
use super::transport::{read_message, write_message};
use super::uri::path_to_uri;

const INITIALIZE_TIMEOUT: Duration = Duration::from_secs(45);
const DIAGNOSTICS_TIMEOUT: Duration = Duration::from_secs(3);
const DIAGNOSTICS_DEBOUNCE: Duration = Duration::from_millis(150);
const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(5);

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

fn to_lsp_uri(s: &str) -> Result<lsp_types::Uri, String> {
    lsp_types::Uri::from_str(s).map_err(|e| format!("invalid URI '{}': {e}", s))
}

impl LspClient {
    pub async fn start(server: ServerSpec, root: PathBuf) -> Result<Arc<Self>, String> {
        Self::start_with_command(server, root, server.command, server.args).await
    }

    pub async fn start_with_command(
        server: ServerSpec,
        root: PathBuf,
        command_path: impl AsRef<OsStr>,
        command_args: &[impl AsRef<OsStr>],
    ) -> Result<Arc<Self>, String> {
        debug!(server = server.id, root = %root.display(), "lsp spawn");
        let mut command = Command::new(command_path);
        command.args(command_args);
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

    pub fn snapshot(&self, path: &Path) -> DiagnosticSnapshot {
        self.diagnostics.snapshot(path)
    }

    // ── typed request/query ───────────────────────────────────────────

    pub async fn request_typed<R: Request>(
        &self,
        params: R::Params,
        timeout: Duration,
    ) -> Result<R::Result, String>
    where
        R::Params: serde::Serialize,
        R::Result: DeserializeOwned,
    {
        let pv = serde_json::to_value(params)
            .map_err(|error| format!("failed to encode {}: {error}", R::METHOD))?;
        let rv = self.request_with_timeout(R::METHOD, pv, timeout).await?;
        serde_json::from_value(rv)
            .map_err(|error| format!("failed to decode {}: {error}", R::METHOD))
    }

    pub async fn notify_typed<N: Notification>(&self, params: N::Params) -> Result<(), String>
    where
        N::Params: serde::Serialize,
    {
        let pv = serde_json::to_value(params)
            .map_err(|error| format!("failed to encode {}: {error}", N::METHOD))?;
        self.notify(N::METHOD, pv).await
    }

    // ── lifecycle ─────────────────────────────────────────────────────

    #[allow(dead_code)]
    pub async fn close_file(&self, path: &Path) -> Result<(), String> {
        let uri = to_lsp_uri(&path_to_uri(path)?)?;
        self.notify_typed::<DidCloseTextDocument>(DidCloseTextDocumentParams {
            text_document: TextDocumentIdentifier { uri },
        })
        .await?;
        self.documents.lock().await.remove(path);
        self.diagnostics.clear(path);
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn shutdown(&self) {
        self.alive.store(false, Ordering::Relaxed);
        let _ = tokio::time::timeout(
            SHUTDOWN_TIMEOUT,
            self.request_typed::<Shutdown>((), SHUTDOWN_TIMEOUT),
        )
        .await;
        let _ = self.notify_typed::<Exit>(()).await;
        let mut child = self._child.lock().await;
        let _ = child.kill().await;
        let _ = child.wait().await;
    }

    // ── document sync ─────────────────────────────────────────────────

    pub async fn touch_file_from_disk(
        &self,
        path: &Path,
        wait: bool,
    ) -> Result<Vec<Diagnostic>, String> {
        let text = tokio::fs::read_to_string(path)
            .await
            .map_err(|error| format!("failed to read {} for LSP: {error}", path.display()))?;
        self.touch_file_with_text(path, &text, wait).await
    }

    pub async fn touch_file_with_text(
        &self,
        path: &Path,
        text: &str,
        wait: bool,
    ) -> Result<Vec<Diagnostic>, String> {
        if !self.alive.load(Ordering::Relaxed) {
            return Err(format!("{} is not running", self.server.id));
        }
        let uri = to_lsp_uri(&path_to_uri(path)?)?;
        let (version, was_open) = self.next_document_version(path).await;
        self.diagnostics.clear(path);
        if was_open {
            self.notify_typed::<DidChangeTextDocument>(DidChangeTextDocumentParams {
                text_document: VersionedTextDocumentIdentifier { uri, version },
                content_changes: vec![TextDocumentContentChangeEvent {
                    range: None,
                    range_length: None,
                    text: text.to_string(),
                }],
            })
            .await?;
        } else {
            self.notify_typed::<DidOpenTextDocument>(DidOpenTextDocumentParams {
                text_document: TextDocumentItem {
                    uri,
                    language_id: self.server.language_id.to_string(),
                    version,
                    text: text.to_string(),
                },
            })
            .await?;
        }
        if wait {
            self.wait_for_diagnostics(path, version, DIAGNOSTICS_TIMEOUT)
                .await;
        }
        Ok(self.diagnostics.get(path))
    }

    async fn next_document_version(&self, path: &Path) -> (i32, bool) {
        let mut docs = self.documents.lock().await;
        let entry = docs
            .entry(path.to_path_buf())
            .or_insert(OpenDocument { version: 0 });
        let was_open = entry.version > 0;
        entry.version += 1;
        (entry.version, was_open)
    }

    async fn current_document_version(&self, path: &Path) -> Option<i32> {
        self.documents.lock().await.get(path).map(|doc| doc.version)
    }

    // ── initialize ────────────────────────────────────────────────────

    async fn initialize(&self) -> Result<(), String> {
        let root_uri = to_lsp_uri(&path_to_uri(&self.root)?)?;
        let params = InitializeParams {
            #[allow(deprecated)]
            process_id: Some(std::process::id()),
            #[allow(deprecated)]
            root_uri: None, // deprecated — using workspace_folders
            workspace_folders: Some(vec![WorkspaceFolder {
                uri: root_uri,
                name: "workspace".to_string(),
            }]),
            capabilities: ClientCapabilities {
                window: Some(lsp_types::WindowClientCapabilities {
                    work_done_progress: Some(true),
                    ..Default::default()
                }),
                workspace: Some(lsp_types::WorkspaceClientCapabilities {
                    configuration: Some(true),
                    workspace_folders: Some(true),
                    ..Default::default()
                }),
                text_document: Some(lsp_types::TextDocumentClientCapabilities {
                    synchronization: Some(TextDocumentSyncClientCapabilities {
                        dynamic_registration: Some(false),
                        will_save: Some(false),
                        will_save_wait_until: Some(false),
                        did_save: Some(false),
                    }),
                    publish_diagnostics: Some(
                        lsp_types::PublishDiagnosticsClientCapabilities::default(),
                    ),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        };
        self.request_with_timeout(
            "initialize",
            serde_json::to_value(params).unwrap(),
            INITIALIZE_TIMEOUT,
        )
        .await?;
        self.notify_typed::<Initialized>(InitializedParams {}).await
    }

    // ── request / notify ──────────────────────────────────────────────

    async fn request_with_timeout(
        &self,
        method: &str,
        params: Value,
        timeout: Duration,
    ) -> Result<Value, String> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let (tx, rx) = oneshot::channel();
        self.pending.lock().await.insert(id, tx);
        let message = json!({"jsonrpc":"2.0","id":id,"method":method,"params":params});
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
        let message = json!({"jsonrpc":"2.0","method":method,"params":params});
        self.write(&message).await
    }

    async fn write(&self, message: &Value) -> Result<(), String> {
        let mut stdin = self.stdin.lock().await;
        write_message(&mut *stdin, message).await
    }

    // ── diagnostics waiting ───────────────────────────────────────────

    async fn wait_for_diagnostics(&self, path: &Path, version: i32, timeout: Duration) {
        let mut rx = self.diagnostics.subscribe();
        let target = path.to_path_buf();
        let wait = async move {
            loop {
                match rx.recv().await {
                    Ok(event)
                        if event.path == target
                            && (event.version.is_none() || event.version == Some(version)) =>
                    {
                        break;
                    }
                    Ok(_) => {}
                    Err(_) => return,
                }
            }
            loop {
                match tokio::time::timeout(DIAGNOSTICS_DEBOUNCE, rx.recv()).await {
                    Ok(Ok(event))
                        if event.path == target
                            && (event.version.is_none() || event.version == Some(version)) =>
                    {
                        continue;
                    }
                    Ok(Ok(_)) => continue,
                    Ok(Err(_)) | Err(_) => break,
                }
            }
        };
        let _ = tokio::time::timeout(timeout, wait).await;
    }

    // ── reader / handler ──────────────────────────────────────────────

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
                self.handle_diagnostics(params.clone()).await;
            }
        }
    }

    async fn handle_diagnostics(&self, params: Value) {
        let params = match serde_json::from_value::<PublishDiagnosticsParams>(params) {
            Ok(params) => params,
            Err(error) => {
                warn!(server = self.server.id, ?error, "invalid lsp diagnostics");
                return;
            }
        };
        let path = match super::uri::uri_to_path(&params.uri.to_string()) {
            Ok(path) => path,
            Err(error) => {
                warn!(server = self.server.id, ?error, "invalid diagnostics URI");
                return;
            }
        };
        let current_version = self.current_document_version(&path).await;
        if !self
            .diagnostics
            .update(path, params.version, params.diagnostics, current_version)
        {
            debug!(server = self.server.id, "dropped stale diagnostics");
        }
    }
}
