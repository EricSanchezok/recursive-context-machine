//! LSP client pool.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock};
use std::time::{Duration, Instant};

use lsp_types::request::Request;
use machine::Environment;
use serde::de::DeserializeOwned;
use tokio::sync::{Mutex, Notify};
use tracing::{debug, warn};

use super::client::LspClient;
use super::diagnostics::{Diagnostic, DiagnosticSnapshot};
use super::servers::{ServerSpec, find_root, server_for_file};

const BROKEN_TTL: Duration = Duration::from_secs(60);
pub const QUERY_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ClientKey {
    server_id: &'static str,
    root: PathBuf,
}

enum ClientState {
    Starting { notify: Arc<Notify> },
    Running { client: Arc<LspClient> },
    Broken { reason: String, since: Instant },
}

struct LspPool {
    states: Mutex<HashMap<ClientKey, ClientState>>,
}

static POOL: LazyLock<LspPool> = LazyLock::new(|| LspPool {
    states: Mutex::new(HashMap::new()),
});

pub async fn snapshot(env: &Environment, path: &Path) -> DiagnosticSnapshot {
    let Some((key, _server)) = key_for_file(env, path) else {
        return DiagnosticSnapshot::empty();
    };

    let states = POOL.states.lock().await;
    match states.get(&key) {
        Some(ClientState::Running { client }) => client.snapshot(path),
        _ => DiagnosticSnapshot::empty(),
    }
}

pub async fn touch_file_from_disk(env: &Environment, path: &Path, wait: bool) -> Vec<Diagnostic> {
    let Some((key, server)) = key_for_file(env, path) else {
        return Vec::new();
    };

    let Some(client) = get_or_start_client(key.clone(), server).await else {
        return Vec::new();
    };

    match client.touch_file_from_disk(path, wait).await {
        Ok(diagnostics) => diagnostics,
        Err(error) => {
            remove_client(&key).await;
            warn!(server = server.id, path = %path.display(), ?error, "lsp touch failed");
            Vec::new()
        }
    }
}

pub async fn touch_file_with_text(
    env: &Environment,
    path: &Path,
    text: &str,
    wait: bool,
) -> Vec<Diagnostic> {
    let Some((key, server)) = key_for_file(env, path) else {
        return Vec::new();
    };

    let Some(client) = get_or_start_client(key.clone(), server).await else {
        return Vec::new();
    };

    match client.touch_file_with_text(path, text, wait).await {
        Ok(diagnostics) => diagnostics,
        Err(error) => {
            remove_client(&key).await;
            warn!(server = server.id, path = %path.display(), ?error, "lsp touch failed");
            Vec::new()
        }
    }
}

pub async fn query<R: Request>(
    env: &Environment,
    path: &Path,
    params: R::Params,
) -> Result<R::Result, String>
where
    R::Params: serde::Serialize,
    R::Result: DeserializeOwned,
{
    let (key, server) =
        key_for_file(env, path).ok_or_else(|| format!("no LSP server for {}", path.display()))?;

    let client = get_or_start_client(key.clone(), server)
        .await
        .ok_or_else(|| format!("{} is unavailable", server.id))?;

    client.request_typed::<R>(params, QUERY_TIMEOUT).await
}

/// Shutdown all running LSP servers gracefully.
#[allow(dead_code)]
pub async fn shutdown_all() {
    let mut states = POOL.states.lock().await;
    let running: Vec<_> = states
        .iter()
        .filter_map(|(key, state)| match state {
            ClientState::Running { client } => Some((key.clone(), Arc::clone(client))),
            _ => None,
        })
        .collect();
    for (key, client) in running {
        client.shutdown().await;
        states.remove(&key);
    }
}

fn key_for_file(env: &Environment, path: &Path) -> Option<(ClientKey, ServerSpec)> {
    let server = server_for_file(path)?;
    let root = find_root(path, server, env)?;
    Some((
        ClientKey {
            server_id: server.id,
            root,
        },
        server,
    ))
}

async fn get_or_start_client(key: ClientKey, server: ServerSpec) -> Option<Arc<LspClient>> {
    loop {
        let wait = {
            let mut states = POOL.states.lock().await;
            match states.get(&key) {
                Some(ClientState::Running { client }) => return Some(Arc::clone(client)),
                Some(ClientState::Starting { notify }) => Some(Arc::clone(notify)),
                Some(ClientState::Broken { reason, since }) if since.elapsed() < BROKEN_TTL => {
                    debug!(server = key.server_id, root = %key.root.display(), reason, "lsp server is marked broken");
                    return None;
                }
                Some(ClientState::Broken { .. }) | None => {
                    let notify = Arc::new(Notify::new());
                    states.insert(
                        key.clone(),
                        ClientState::Starting {
                            notify: Arc::clone(&notify),
                        },
                    );
                    None
                }
            }
        };

        if let Some(wait) = wait {
            wait.notified().await;
            continue;
        }

        return start_client(key, server).await;
    }
}

async fn start_client(key: ClientKey, server: ServerSpec) -> Option<Arc<LspClient>> {
    let result = LspClient::start(server, key.root.clone()).await;

    let mut states = POOL.states.lock().await;
    let notify = match states.remove(&key) {
        Some(ClientState::Starting { notify }) => Some(notify),
        _ => None,
    };

    let client = match result {
        Ok(client) => {
            states.insert(
                key,
                ClientState::Running {
                    client: Arc::clone(&client),
                },
            );
            Some(client)
        }
        Err(error) => {
            warn!(server = server.id, root = %key.root.display(), ?error, "lsp unavailable");
            states.insert(
                key,
                ClientState::Broken {
                    reason: error,
                    since: Instant::now(),
                },
            );
            None
        }
    };

    if let Some(notify) = notify {
        notify.notify_waiters();
    }

    client
}

async fn remove_client(key: &ClientKey) {
    let mut states = POOL.states.lock().await;
    if matches!(states.get(key), Some(ClientState::Running { .. })) {
        states.remove(key);
    }
}
