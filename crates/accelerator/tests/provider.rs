//! Tests for the provider table and `resolve_model`.

use accelerator::provider::{self, ResolveError};
use machine::Protocol;
use std::sync::Mutex;

/// Tests in this file all mutate process-wide environment variables. Cargo
/// runs tests in a single binary on multiple threads by default, so without
/// serialization they would race. A module-level mutex held for the lifetime
/// of each test serializes them.
static ENV_LOCK: Mutex<()> = Mutex::new(());

/// Save / restore env vars so tests don't leak into one another. Acquires the
/// module-level [`ENV_LOCK`] for the lifetime of the guard.
struct EnvGuard {
    _lock: std::sync::MutexGuard<'static, ()>,
    saved: Vec<(String, Option<String>)>,
}

impl EnvGuard {
    fn new(keys: &[&'static str]) -> Self {
        let lock = ENV_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let saved = keys
            .iter()
            .map(|key| (key.to_string(), std::env::var(key).ok()))
            .collect();
        for key in keys {
            unsafe {
                std::env::remove_var(key);
            }
        }
        Self { _lock: lock, saved }
    }

    fn set(&self, key: &str, value: &str) {
        unsafe {
            std::env::set_var(key, value);
        }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        for (key, value) in &self.saved {
            unsafe {
                match value {
                    Some(original) => std::env::set_var(key, original),
                    None => std::env::remove_var(key),
                }
            }
        }
    }
}

const ALL_ENV_VARS: &[&str] = &[
    "SII_API_KEY",
    "DEEPSEEK_API_KEY",
    "OPENAI_API_KEY",
    "ANTHROPIC_API_KEY",
    "GEMINI_API_KEY",
];

#[test]
fn resolves_explicit_provider_slash_model() {
    let guard = EnvGuard::new(ALL_ENV_VARS);
    guard.set("SII_API_KEY", "sk-test");

    let model = provider::resolve_model(Some("sii/gpt-4.1")).expect("resolve");
    assert_eq!(model.name, "sii/gpt-4.1");
    assert_eq!(
        model.endpoint.as_deref(),
        Some("https://apicz.boyuerichdata.com/v1")
    );
    assert_eq!(model.credentials.as_deref(), Some("sk-test"));
    // Cost / Limit / Modalities preserved from MODEL_PRESETS.
    assert!(model.cost.is_some());
    assert!(model.limit.is_some());
    assert!(model.modalities.is_some());
}

#[test]
fn resolves_provider_only_to_default_model() {
    let guard = EnvGuard::new(ALL_ENV_VARS);
    guard.set("DEEPSEEK_API_KEY", "sk-deep");

    let model = provider::resolve_model(Some("deepseek")).expect("resolve");
    assert_eq!(model.name, "deepseek/deepseek-v4-flash");
    assert_eq!(model.endpoint.as_deref(), Some("https://api.deepseek.com"));
    assert_eq!(model.protocol, Protocol::DeepSeek);
}

#[test]
fn auto_detect_picks_first_credentialed_provider() {
    let guard = EnvGuard::new(ALL_ENV_VARS);
    guard.set("OPENAI_API_KEY", "sk-openai");

    let model = provider::resolve_model(None).expect("resolve");
    assert_eq!(model.name, "openai/gpt-4o");
}

#[test]
fn auto_detect_respects_priority_order() {
    let guard = EnvGuard::new(ALL_ENV_VARS);
    // SII is listed before DEEPSEEK in PROVIDERS, so it should win.
    guard.set("SII_API_KEY", "sk-sii");
    guard.set("DEEPSEEK_API_KEY", "sk-deep");

    let model = provider::resolve_model(None).expect("resolve");
    assert_eq!(model.name, "sii/gpt-4.1");
}

#[test]
fn bare_model_name_is_rejected() {
    let _guard = EnvGuard::new(ALL_ENV_VARS);
    match provider::resolve_model(Some("gpt-4.1")) {
        Err(ResolveError::UnknownProvider(name)) => assert_eq!(name, "gpt-4.1"),
        other => panic!("expected UnknownProvider, got {other:?}"),
    }
}

#[test]
fn empty_model_after_slash_is_bad_format() {
    let _guard = EnvGuard::new(ALL_ENV_VARS);
    match provider::resolve_model(Some("sii/")) {
        Err(ResolveError::BadFormat(spec)) => assert_eq!(spec, "sii/"),
        other => panic!("expected BadFormat, got {other:?}"),
    }
}

#[test]
fn unknown_provider_is_reported() {
    let _guard = EnvGuard::new(ALL_ENV_VARS);
    match provider::resolve_model(Some("foo/bar")) {
        Err(ResolveError::UnknownProvider(name)) => assert_eq!(name, "foo"),
        other => panic!("expected UnknownProvider, got {other:?}"),
    }
}

#[test]
fn missing_credential_is_reported_with_env_var() {
    let _guard = EnvGuard::new(ALL_ENV_VARS);
    match provider::resolve_model(Some("sii/gpt-4.1")) {
        Err(ResolveError::NoCredential { provider, env_var }) => {
            assert_eq!(provider, "sii");
            assert_eq!(env_var, "SII_API_KEY");
        }
        other => panic!("expected NoCredential, got {other:?}"),
    }
}

#[test]
fn no_credential_at_all_when_auto_detecting() {
    let _guard = EnvGuard::new(ALL_ENV_VARS);
    match provider::resolve_model(None) {
        Err(ResolveError::NoCredentialAtAll) => {}
        other => panic!("expected NoCredentialAtAll, got {other:?}"),
    }
}

#[test]
fn unknown_model_under_known_provider_resolves_without_preset() {
    let guard = EnvGuard::new(ALL_ENV_VARS);
    guard.set("SII_API_KEY", "sk-test");

    // Provider known but model has no preset entry — should still resolve, just
    // without cost/limit/modalities populated.
    let model = provider::resolve_model(Some("sii/some-future-model")).expect("resolve");
    assert_eq!(model.name, "sii/some-future-model");
    assert!(model.cost.is_none());
    assert!(model.limit.is_none());
    assert!(model.modalities.is_none());
}

#[test]
fn error_messages_are_actionable() {
    let bad = ResolveError::BadFormat("gpt-4.1".into());
    assert!(bad.to_string().contains("provider"));

    let unknown = ResolveError::UnknownProvider("foo".into());
    let msg = unknown.to_string();
    assert!(msg.contains("foo"));
    assert!(msg.contains("sii"));

    let no_cred = ResolveError::NoCredential {
        provider: "sii",
        env_var: "SII_API_KEY",
    };
    assert!(no_cred.to_string().contains("SII_API_KEY"));

    let none = ResolveError::NoCredentialAtAll;
    let msg = none.to_string();
    assert!(msg.contains("SII_API_KEY"));
    assert!(msg.contains("--model"));
}
