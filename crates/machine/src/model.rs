use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Model configuration — a structured description of an LLM.
///
/// Only `name` and `provider` are required. All other fields are optional
/// and interpreted by the Reactor implementation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Model {
    pub name: String,
    pub provider: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u64>,
    #[serde(flatten, default, skip_serializing_if = "HashMap::is_empty")]
    pub extra: HashMap<String, Value>,
}
