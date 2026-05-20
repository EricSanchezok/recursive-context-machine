//! Provider table — declarative registry of LLM providers and model presets.
//!
//! Adding a new provider means adding a row to [`PROVIDERS`]; adding a model
//! preset means adding a row to [`MODEL_PRESETS`]. No business logic changes.
//!
//! Use [`resolve_model`] to construct a [`Model`] from a `provider/model`
//! specifier (or via auto-detection from environment variables).

use std::fmt;

use machine::{Cost, Limit, Modalities, Modality, Model, Protocol};

/// A provider — wire protocol, endpoint, credential source, and a default
/// model name to use when only the provider is specified.
pub struct Provider {
    pub key: &'static str,
    pub protocol: Protocol,
    pub endpoint: &'static str,
    pub env_var: &'static str,
    pub default_model: &'static str,
}

/// Static metadata about a model — token limits, pricing, modalities. Joined
/// with provider info at resolve time.
pub struct ModelPreset {
    pub name: &'static str,
    pub limit: Option<Limit>,
    pub cost: Option<Cost>,
    pub modalities_input: &'static [Modality],
    pub modalities_output: &'static [Modality],
}

/// Built-in providers. Order defines auto-detection priority when no spec is
/// given: the first provider whose env var is set wins.
pub const PROVIDERS: &[Provider] = &[
    Provider {
        key: "sii",
        protocol: Protocol::OpenAI,
        endpoint: "https://apicz.boyuerichdata.com/v1",
        env_var: "SII_API_KEY",
        default_model: "gpt-4.1",
    },
    Provider {
        key: "deepseek",
        protocol: Protocol::OpenAI,
        endpoint: "https://api.deepseek.com",
        env_var: "DEEPSEEK_API_KEY",
        default_model: "deepseek-v4-flash",
    },
    Provider {
        key: "openai",
        protocol: Protocol::OpenAI,
        endpoint: "https://api.openai.com/v1",
        env_var: "OPENAI_API_KEY",
        default_model: "gpt-4o",
    },
    Provider {
        key: "anthropic",
        protocol: Protocol::Anthropic,
        endpoint: "https://api.anthropic.com",
        env_var: "ANTHROPIC_API_KEY",
        default_model: "claude-sonnet-4-5",
    },
    Provider {
        key: "gemini",
        protocol: Protocol::Gemini,
        endpoint: "https://generativelanguage.googleapis.com/v1beta",
        env_var: "GEMINI_API_KEY",
        default_model: "gemini-2.5-flash",
    },
];

/// Built-in model metadata. Models without a preset still resolve — they just
/// carry no cost/limit/modality info.
pub const MODEL_PRESETS: &[ModelPreset] = &[
    ModelPreset {
        name: "gpt-4.1",
        limit: Some(Limit {
            context: 1_047_576,
            input: None,
            output: 32_768,
        }),
        cost: Some(Cost {
            input: 1.6,
            output: 3.21,
            cache_read: Some(0.5),
            cache_write: None,
        }),
        modalities_input: &[Modality::Text, Modality::Image],
        modalities_output: &[Modality::Text],
    },
    ModelPreset {
        name: "deepseek-v4-flash",
        limit: Some(Limit {
            context: 1_048_576,
            input: None,
            output: 393_216,
        }),
        cost: Some(Cost {
            input: 1.0,
            output: 2.0,
            cache_read: Some(0.02),
            cache_write: None,
        }),
        modalities_input: &[Modality::Text],
        modalities_output: &[Modality::Text],
    },
    ModelPreset {
        name: "deepseek-v4-pro",
        limit: Some(Limit {
            context: 1_048_576,
            input: None,
            output: 393_216,
        }),
        cost: Some(Cost {
            input: 3.0,
            output: 6.0,
            cache_read: Some(0.025),
            cache_write: None,
        }),
        modalities_input: &[Modality::Text],
        modalities_output: &[Modality::Text],
    },
];

/// Reasons [`resolve_model`] can fail. Each variant carries enough context for
/// a CLI to print an actionable message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolveError {
    BadFormat(String),
    UnknownProvider(String),
    NoCredential {
        provider: &'static str,
        env_var: &'static str,
    },
    NoCredentialAtAll,
}

impl fmt::Display for ResolveError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadFormat(spec) => write!(
                formatter,
                "model spec '{spec}' is missing the provider prefix; try '<provider>/<model>' (e.g. 'sii/gpt-4.1')"
            ),
            Self::UnknownProvider(name) => {
                let known: Vec<&str> = PROVIDERS.iter().map(|p| p.key).collect();
                write!(
                    formatter,
                    "unknown provider '{name}'; known providers: {}",
                    known.join(", ")
                )
            }
            Self::NoCredential { provider, env_var } => write!(
                formatter,
                "provider '{provider}' has no credential; set {env_var} or pick another provider"
            ),
            Self::NoCredentialAtAll => {
                let envs: Vec<&str> = PROVIDERS.iter().map(|p| p.env_var).collect();
                write!(
                    formatter,
                    "no provider credential found; set one of: {} — or pass --model <provider>/<model>",
                    envs.join(", ")
                )
            }
        }
    }
}

impl std::error::Error for ResolveError {}

/// Resolve a model specifier to a fully-populated [`Model`].
///
/// `spec` accepts:
/// - `Some("provider/model")` — explicit provider and model
/// - `Some("provider")` — provider only; uses the provider's `default_model`
/// - `None` — auto-detect: pick the first provider in [`PROVIDERS`] whose env
///   var is set
///
/// A bare model name (e.g. `Some("gpt-4.1")`) is rejected as ambiguous —
/// the same model name may be served by multiple providers.
pub fn resolve_model(spec: Option<&str>) -> Result<Model, ResolveError> {
    let (provider, model_name) = match spec {
        Some(raw) => parse_spec(raw)?,
        None => auto_detect()?,
    };
    build_model(provider, model_name)
}

fn parse_spec(spec: &str) -> Result<(&'static Provider, String), ResolveError> {
    let trimmed = spec.trim();
    if trimmed.is_empty() {
        return Err(ResolveError::BadFormat(spec.to_string()));
    }
    match trimmed.split_once('/') {
        Some((provider_key, model_name)) => {
            if model_name.is_empty() {
                return Err(ResolveError::BadFormat(spec.to_string()));
            }
            let provider = lookup_provider(provider_key)?;
            Ok((provider, model_name.to_string()))
        }
        None => {
            // Provider-only form: use the provider's default_model.
            // To distinguish from a bare model name, require the token to
            // actually be a known provider key.
            let provider = lookup_provider(trimmed)?;
            Ok((provider, provider.default_model.to_string()))
        }
    }
}

fn lookup_provider(key: &str) -> Result<&'static Provider, ResolveError> {
    PROVIDERS
        .iter()
        .find(|provider| provider.key == key)
        .ok_or_else(|| ResolveError::UnknownProvider(key.to_string()))
}

fn auto_detect() -> Result<(&'static Provider, String), ResolveError> {
    for provider in PROVIDERS {
        if std::env::var(provider.env_var).is_ok() {
            return Ok((provider, provider.default_model.to_string()));
        }
    }
    Err(ResolveError::NoCredentialAtAll)
}

fn build_model(provider: &'static Provider, model_name: String) -> Result<Model, ResolveError> {
    let credentials = std::env::var(provider.env_var).map_err(|_| ResolveError::NoCredential {
        provider: provider.key,
        env_var: provider.env_var,
    })?;

    let preset = MODEL_PRESETS
        .iter()
        .find(|preset| preset.name == model_name);

    let qualified = format!("{}/{}", provider.key, model_name);

    let mut model = Model {
        name: qualified,
        protocol: provider.protocol,
        endpoint: Some(provider.endpoint.to_string()),
        credentials: Some(credentials),
        ..Default::default()
    };

    if let Some(preset) = preset {
        model.limit = preset.limit.clone();
        model.cost = preset.cost.clone();
        model.modalities = Some(Modalities {
            input: preset.modalities_input.to_vec(),
            output: preset.modalities_output.to_vec(),
        });
    }

    // The qualified name is what we send to the wire — but providers expect the
    // bare model name. We store the bare name in `extra` for completion.rs to
    // pick up, while keeping the qualified name as the registry key.
    model
        .extra
        .insert("wire_name".to_string(), model_name.into());

    Ok(model)
}

/// The bare model name to send on the wire. Falls back to `model.name` if the
/// `wire_name` extra is missing (for hand-built `Model`s that did not go
/// through [`resolve_model`]).
pub fn wire_name(model: &Model) -> &str {
    model
        .extra
        .get("wire_name")
        .and_then(|value| value.as_str())
        .unwrap_or(&model.name)
}
