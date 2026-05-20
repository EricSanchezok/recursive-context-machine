use std::time::Duration;

use serde_json::Value;

use super::http::HttpTransport;
use super::stdio::StdioTransport;

pub(crate) const CALL_TIMEOUT: Duration = Duration::from_secs(120);

/// MCP transport dispatching to stdio or HTTP.
pub(crate) enum Transport {
    Stdio(Box<StdioTransport>),
    Http(HttpTransport),
}

impl Transport {
    pub(crate) async fn initialize(&self) -> Result<Value, String> {
        match self {
            Self::Stdio(t) => t.initialize().await,
            Self::Http(t) => t.initialize().await,
        }
    }

    pub(crate) async fn list_tools(&self) -> Result<Vec<Value>, String> {
        match self {
            Self::Stdio(t) => t.list_tools().await,
            Self::Http(t) => t.list_tools().await,
        }
    }

    pub(crate) async fn call(&self, method: &str, params: Value) -> Result<Value, String> {
        match self {
            Self::Stdio(t) => t.call(method, params).await,
            Self::Http(t) => t.call(method, params).await,
        }
    }
}
