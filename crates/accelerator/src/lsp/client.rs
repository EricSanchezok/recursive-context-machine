//! Minimal LSP client for diagnostics.

use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};
use std::time::Duration;

use lsp_types::PublishDiagnosticsParams;
use serde_json::{Value, json};
use tokio::io::BufReader;
use tokio::process::{Child, ChildStdin, Command};
use tokio::sync::{Mutex, oneshot};
use tracing::{debug, warn};

use super::diagnostics::{Diagnostic, DiagnosticSnapshot, DiagnosticStore};
use super::servers::ServerSpec;
use super::transport::{read_message, write_message};
use super::uri::{path_to_uri, uri_to_path};

const INITIALIZE_TIMEOUT: Duration = Duration::from_secs(45);
const DIAGNOSTICS_TIMEOUT: Duration = Duration::from_secs(3);
const DIAGNOSTICS_DEBOUNCE: Duration = Duration::from_millis(150);

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
        Self::start_with_command(server, root, server.command, server.args).await
    }

    async fn start_with_command(
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

        let uri = path_to_uri(path)?;
        let (version, was_open) = self.next_document_version(path).await;

        self.diagnostics.clear(path);

        if was_open {
            self.notify(
                "textDocument/didChange",
                json!({
                    "textDocument": { "uri": uri, "version": version },
                    "contentChanges": [{ "text": text }]
                }),
            )
            .await?;
        } else {
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

        let path = match uri_to_path(&params.uri.to_string()) {
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

#[cfg(test)]
mod tests {
    use super::*;

    const FAKE_SERVER: &str = r#"
import json, sys

def read_msg():
    length = None
    while True:
        line = sys.stdin.buffer.readline()
        if not line:
            return None
        if line in (b'\r\n', b'\n'):
            break
        name, value = line.decode().split(':', 1)
        if name.lower() == 'content-length':
            length = int(value.strip())
    return json.loads(sys.stdin.buffer.read(length).decode())

def write_msg(msg):
    body = json.dumps(msg, separators=(',', ':')).encode()
    sys.stdout.buffer.write(b'Content-Length: ' + str(len(body)).encode() + b'\r\n\r\n' + body)
    sys.stdout.buffer.flush()

while True:
    msg = read_msg()
    if msg is None:
        break
    if msg.get('method') == 'initialize':
        write_msg({'jsonrpc':'2.0','id':msg['id'],'result':{'capabilities':{'textDocumentSync':1}}})
    elif msg.get('method') == 'textDocument/didOpen':
        doc = msg['params']['textDocument']
        write_msg({'jsonrpc':'2.0','method':'textDocument/publishDiagnostics','params':{'uri':doc['uri'],'version':doc['version'],'diagnostics':[{'range':{'start':{'line':0,'character':0},'end':{'line':0,'character':1}},'severity':1,'message':'fake error'}]}})
    elif msg.get('method') == 'textDocument/didChange':
        doc = msg['params']['textDocument']
        write_msg({'jsonrpc':'2.0','method':'textDocument/publishDiagnostics','params':{'uri':doc['uri'],'version':doc['version'],'diagnostics':[{'range':{'start':{'line':1,'character':0},'end':{'line':1,'character':1}},'severity':1,'message':'changed error'}]}})
"#;

    #[tokio::test]
    async fn fake_lsp_server_reports_versioned_diagnostics() {
        if std::process::Command::new("python3")
            .arg("--version")
            .output()
            .is_err()
        {
            eprintln!("skipping fake LSP test: python3 not found");
            return;
        }

        let root = std::env::temp_dir().join(format!("rcm_fake_lsp_{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        let path = root.join("lib.rs");
        std::fs::write(&path, "fn main() {}\n").unwrap();

        let server = ServerSpec {
            id: "fake-lsp",
            language_id: "rust",
            extensions: &["rs"],
            root_markers: &["Cargo.toml"],
            command: "python3",
            args: &[],
        };

        let client = LspClient::start_with_command(
            server,
            root.clone(),
            "python3",
            &["-u", "-c", FAKE_SERVER],
        )
        .await
        .unwrap();

        let first = client
            .touch_file_with_text(&path, "fn main() {}\n", true)
            .await
            .unwrap();
        assert_eq!(first[0].message, "fake error");

        let second = client
            .touch_file_with_text(&path, "fn main() {}\nlet x = 1;\n", true)
            .await
            .unwrap();
        assert_eq!(second[0].message, "changed error");

        std::fs::remove_dir_all(root).ok();
    }
}
