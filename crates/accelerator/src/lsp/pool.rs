//! LSP client pool.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock};

use machine::Environment;
use tokio::sync::{Mutex, Notify};
use tracing::{debug, warn};

use super::client::LspClient;
use super::diagnostics::Diagnostic;
use super::servers::{ServerSpec, find_root, server_for_file};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ClientKey {
    server_id: &'static str,
    root: PathBuf,
}

struct LspPool {
    clients: Mutex<HashMap<ClientKey, Arc<LspClient>>>,
    broken: Mutex<HashSet<ClientKey>>,
    spawning: Mutex<HashMap<ClientKey, Arc<Notify>>>,
}

static POOL: LazyLock<LspPool> = LazyLock::new(|| LspPool {
    clients: Mutex::new(HashMap::new()),
    broken: Mutex::new(HashSet::new()),
    spawning: Mutex::new(HashMap::new()),
});

pub async fn touch_file(env: &Environment, path: &Path, wait: bool) -> Vec<Diagnostic> {
    let Some(server) = server_for_file(path) else {
        return Vec::new();
    };

    let Some(root) = find_root(path, server, env) else {
        debug!(path = %path.display(), server = server.id, "lsp root not found");
        return Vec::new();
    };

    let key = ClientKey {
        server_id: server.id,
        root,
    };

    let client_key = key.clone();
    let Some(client) = get_or_start_client(key, server).await else {
        return Vec::new();
    };

    match client.touch_file(path, wait).await {
        Ok(diagnostics) => diagnostics,
        Err(error) => {
            POOL.clients.lock().await.remove(&client_key);
            warn!(server = server.id, path = %path.display(), ?error, "lsp touch failed");
            Vec::new()
        }
    }
}

async fn get_or_start_client(key: ClientKey, server: ServerSpec) -> Option<Arc<LspClient>> {
    loop {
        if POOL.broken.lock().await.contains(&key) {
            return None;
        }

        if let Some(client) = POOL.clients.lock().await.get(&key).cloned() {
            return Some(client);
        }

        let waiter = {
            let mut spawning = POOL.spawning.lock().await;

            if POOL.broken.lock().await.contains(&key) {
                return None;
            }
            if let Some(client) = POOL.clients.lock().await.get(&key).cloned() {
                return Some(client);
            }

            if let Some(waiter) = spawning.get(&key) {
                Some(Arc::clone(waiter))
            } else {
                spawning.insert(key.clone(), Arc::new(Notify::new()));
                None
            }
        };

        if let Some(waiter) = waiter {
            waiter.notified().await;
            continue;
        }

        return start_client(key, server).await;
    }
}

async fn start_client(key: ClientKey, server: ServerSpec) -> Option<Arc<LspClient>> {
    let result = LspClient::start(server, key.root.clone()).await;

    let waiter = POOL.spawning.lock().await.remove(&key);

    match result {
        Ok(client) => {
            POOL.clients.lock().await.insert(key, Arc::clone(&client));
            if let Some(waiter) = waiter {
                waiter.notify_waiters();
            }
            Some(client)
        }
        Err(error) => {
            warn!(server = server.id, root = %key.root.display(), ?error, "lsp unavailable");
            POOL.broken.lock().await.insert(key);
            if let Some(waiter) = waiter {
                waiter.notify_waiters();
            }
            None
        }
    }
}
