use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Model configuration — a structured description of an LLM.
///
/// `name` is required. `protocol` defaults to `OpenAI`. `endpoint` and
/// `credentials` are used to construct the client.
///
/// Most providers (DeepSeek, Groq, Mistral, xAI, Ollama, OpenRouter,
/// ...) all speak the OpenAI Chat Completions protocol — just set a
/// different `endpoint`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub name: String,
    /// The API wire protocol. Defaults to `OpenAI` since most providers
    /// are OpenAI-compatible.
    #[serde(default)]
    pub protocol: Protocol,
    /// Base URL for the provider's API. Required for custom endpoints;
    /// omitted for well-known providers where the protocol implies the
    /// default endpoint.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,
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
            timeout: Some(180),
            extra: HashMap::new(),
        }
    }
}

impl Model {
    /// Request timeout in seconds.
    ///
    /// Falls back to 180s (3 minutes) when not explicitly set.
    pub fn timeout_secs(&self) -> u64 {
        self.timeout.unwrap_or(180)
    }
}

/// The API wire protocol.
///
/// Only 3 protocols exist — all other distinctions are just different
/// `endpoint` values on the same OpenAI-compatible wire format.
///
/// | Protocol | Rig module | Examples |
/// |----------|-----------|----------|
/// | `OpenAI` | `rig::providers::openai` | OpenAI, DeepSeek, Groq, Mistral, xAI, Ollama, OpenRouter, Together, Perplexity, Hyperbolic, MiniMax, Moonshot, Galadriel, Llamafile, XiaomiMimo, ZAI, Mira, ... |
/// | `Anthropic` | `rig::providers::anthropic` | Anthropic Claude |
/// | `Gemini` | `rig::providers::gemini` | Google Gemini |
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
