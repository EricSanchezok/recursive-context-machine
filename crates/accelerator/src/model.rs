use machine::{Cost, Limit, Modalities, Modality, Model, Protocol};

/// Nex N1 — DeepSeek-based model hosted on the Nex platform.
///
/// Credentials are read from the `NEX_API_KEY` environment variable.
/// If unset, the model is still registered but calls will fail with a
/// [`Hitch`](machine::Content::Hitch) until the key is provided.
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
