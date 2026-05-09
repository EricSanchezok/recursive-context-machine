use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Token usage for a single LLM call.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

/// One cycle in the computation history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Cycle {
    LlmCall {
        tokens: TokenUsage,
    },
    ToolCall {
        tool: String,
        input: Value,
        output: Result<Value, String>,
        duration_ms: u64,
    },
    Prune {
        from: usize,
        to: usize,
        reason: String,
    },
    Compact {
        from: usize,
        to: usize,
    },
    Halt {
        reason: String,
    },
}

/// The full computation history — a sequence of cycles.
///
/// Given the initial tape and register, the Trace can reproduce
/// the entire execution. This is the audit trail.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Trace {
    pub cycles: Vec<Cycle>,
}

impl Trace {
    pub fn new() -> Self {
        Self { cycles: Vec::new() }
    }

    pub fn record(&mut self, cycle: Cycle) {
        self.cycles.push(cycle);
    }

    pub fn cycle_count(&self) -> u32 {
        self.cycles.len() as u32
    }

    pub fn total_tokens(&self) -> TokenUsage {
        let mut total = TokenUsage::default();
        for cycle in &self.cycles {
            if let Cycle::LlmCall { tokens } = cycle {
                total.prompt_tokens += tokens.prompt_tokens;
                total.completion_tokens += tokens.completion_tokens;
                total.total_tokens += tokens.total_tokens;
            }
        }
        total
    }
}
