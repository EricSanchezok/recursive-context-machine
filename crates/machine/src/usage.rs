use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CompletionId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenUsage {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub total_tokens: u64,
    pub cached_input_tokens: u64,
    pub cache_creation_input_tokens: u64,
}

impl TokenUsage {
    pub fn empty() -> Self {
        Self {
            input_tokens: 0,
            output_tokens: 0,
            total_tokens: 0,
            cached_input_tokens: 0,
            cache_creation_input_tokens: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletionRecord {
    pub id: CompletionId,
    pub step: u64,
    pub model: Option<String>,
    pub tokens: TokenUsage,
    pub output_fragment_ids: Vec<u64>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Telemetry {
    pub completions: Vec<CompletionRecord>,
    pub action_counts: std::collections::HashMap<String, u64>,
}

impl Telemetry {
    pub fn next_completion_id(&self) -> CompletionId {
        CompletionId(self.completions.len() as u64 + 1)
    }

    pub fn count_action(&mut self, action: impl Into<String>) {
        *self.action_counts.entry(action.into()).or_default() += 1;
    }

    pub fn record_output_fragment(&mut self, completion_id: CompletionId, fragment_id: u64) {
        if let Some(record) = self
            .completions
            .iter_mut()
            .find(|record| record.id == completion_id)
        {
            record.output_fragment_ids.push(fragment_id);
        }
    }
}
