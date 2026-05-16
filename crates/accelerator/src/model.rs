use machine::{Cost, Limit, Modalities, Modality, Model, Protocol};

/// Nex N1 — DeepSeek-based model hosted on the Nex platform.
///
/// Credentials are read from the `NEX_API_KEY` environment variable.
pub fn nex_n1() -> Model {
    Model {
        name: "nex-agi/nex-n1".into(),
        protocol: Protocol::OpenAI,
        endpoint: Some("https://nex-deepseek.openapi-qb-ai.sii.edu.cn/v1".into()),
        credentials: std::env::var("NEX_API_KEY").ok(),
        limit: Some(Limit {
            context: 128000,
            input: None,
            output: 32768,
        }),
        cost: Some(Cost {
            input: 0.0,
            output: 0.0,
            cache_read: None,
            cache_write: None,
        }),
        modalities: Some(Modalities {
            input: vec![Modality::Text],
            output: vec![Modality::Text],
        }),
        ..Default::default()
    }
}

/// GPT-4.1 — OpenAI model hosted on the SII platform.
///
/// Supports text and image input. Credentials are read from `SII_API_KEY`.
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
