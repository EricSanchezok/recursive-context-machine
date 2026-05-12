use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::register::Register;
use crate::trace::{TokenUsage, Trace};

/// What the user wants to do.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    pub prompt: String,
    pub context: HashMap<String, Value>,
}

impl Intent {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            context: HashMap::new(),
        }
    }

    pub fn with_context(mut self, key: impl Into<String>, value: Value) -> Self {
        self.context.insert(key.into(), value);
        self
    }
}

/// The result of a machine execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    pub text: String,
    pub cycles: u32,
    pub tokens: TokenUsage,
}

/// Rica — the interface for composable machines.
///
/// Single machines (Head) are driven by the free function [`accelerate`](crate::accelerate),
/// not by this trait. This trait exists for combinators (Pipeline, Parallel, Ensemble)
/// so they can treat any machine uniformly.
pub trait Rica: Send + Sync {
    fn run(
        &self,
        intent: Intent,
        register: Register,
    ) -> Pin<Box<dyn Future<Output = (Output, Register, Trace)> + Send + '_>>;
}
