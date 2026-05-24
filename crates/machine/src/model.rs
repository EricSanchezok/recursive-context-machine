use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

const DEFAULT_TIMEOUT_SECS: u64 = 180;

/// LLM configuration.
///
/// `name` is required. `protocol` defaults to `OpenAI`.
/// Most providers use the OpenAI wire format — set `endpoint` for custom providers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub name: String,
    /// API wire protocol.
    #[serde(default)]
    pub protocol: Protocol,
    /// Override the default endpoint URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Limit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<Cost>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modalities: Option<Modalities>,
    /// Request timeout in seconds.
    pub timeout: u64,
    /// Extra HTTP headers sent with every request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    #[serde(flatten, default, skip_serializing_if = "HashMap::is_empty")]
    pub extra: HashMap<String, Value>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            name: String::new(),
            protocol: Protocol::OpenAI,
            endpoint: None,
            credentials: None,
            temperature: None,
            limit: None,
            cost: None,
            modalities: None,
            timeout: DEFAULT_TIMEOUT_SECS,
            headers: None,
            extra: HashMap::new(),
        }
    }
}

/// Wire protocol.
///
/// Three protocols are supported. Most providers are OpenAI-compatible;
/// use the `OpenAI` variant with a custom `endpoint`.
///
/// | Protocol | Examples |
/// |----------|---------|
/// | `OpenAI` | OpenAI, DeepSeek, Groq, Mistral, xAI, Ollama, OpenRouter ... |
/// | `Anthropic` | Anthropic Claude |
/// | `Gemini` | Google Gemini |
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    /// OpenAI Chat Completions API.
    #[default]
    OpenAI,
    /// Anthropic Messages API.
    Anthropic,
    /// Google Gemini API.
    Gemini,
}

/// Token limits for a model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Limit {
    /// Maximum context window size in tokens.
    pub context: u64,
    /// Maximum input tokens (if different from context).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<u64>,
    /// Maximum output tokens.
    pub output: u64,
}

/// Cost per million tokens.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cost {
    /// Cost per million input tokens.
    pub input: f64,
    /// Cost per million output tokens.
    pub output: f64,
    /// Cost per million cached read tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_read: Option<f64>,
    /// Cost per million cached write tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_write: Option<f64>,
}

/// Supported input/output modalities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Modalities {
    pub input: Vec<Modality>,
    pub output: Vec<Modality>,
}

/// A content modality.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Modality {
    Text,
    Audio,
    Image,
    Video,
    Pdf,
}
