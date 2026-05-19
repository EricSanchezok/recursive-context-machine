use machine::{Cost, Limit, Modalities, Modality, Model, Protocol};

/// GPT-4.1 — OpenAI model hosted on the SII platform.
///
/// Supports text and image input. Credentials (`SII_API_KEY`) are read
/// from the environment at call time; `credentials` is `None` if unset.
pub fn gpt4_1() -> Model {
    Model {
        name: "gpt-4.1".into(),
        protocol: Protocol::OpenAI,
        endpoint: Some("https://apicz.boyuerichdata.com/v1".into()),
        credentials: std::env::var("SII_API_KEY").ok(),
        limit: Some(Limit {
            context: 1047576,
            input: None,
            output: 32768,
        }),
        cost: Some(Cost {
            input: 1.6,
            output: 3.21,
            cache_read: Some(0.5),
            cache_write: None,
        }),
        modalities: Some(Modalities {
            input: vec![Modality::Text, Modality::Image],
            output: vec![Modality::Text],
        }),
        ..Default::default()
    }
}

/// DeepSeek V4 Flash — general-purpose model.
///
/// Supports text input only. Credentials (`DEEPSEEK_API_KEY`) are read
/// from the environment at call time; `credentials` is `None` if unset.
pub fn deepseek_v4_flash() -> Model {
    Model {
        name: "deepseek-v4-flash".into(),
        protocol: Protocol::OpenAI,
        endpoint: Some("https://api.deepseek.com".into()),
        credentials: std::env::var("DEEPSEEK_API_KEY").ok(),
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
        modalities: Some(Modalities {
            input: vec![Modality::Text],
            output: vec![Modality::Text],
        }),
        ..Default::default()
    }
}

/// DeepSeek V4 Pro — premium reasoning model.
///
/// Supports text input only. Credentials (`DEEPSEEK_API_KEY`) are read
/// from the environment at call time; `credentials` is `None` if unset.
pub fn deepseek_v4_pro() -> Model {
    Model {
        name: "deepseek-v4-pro".into(),
        protocol: Protocol::OpenAI,
        endpoint: Some("https://api.deepseek.com".into()),
        credentials: std::env::var("DEEPSEEK_API_KEY").ok(),
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
        modalities: Some(Modalities {
            input: vec![Modality::Text],
            output: vec![Modality::Text],
        }),
        ..Default::default()
    }
}
