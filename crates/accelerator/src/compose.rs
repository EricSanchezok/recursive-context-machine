use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::channel::Channel;
use crate::register::Register;
use crate::rica::{Intent, Output, Rica};
use crate::trace::Trace;

// ============================================================================
// Pipeline — sequential composition
// ============================================================================

/// A pipeline runs stages in sequence. Each stage's output feeds the next
/// stage's input through a Channel.
pub struct Pipeline {
    stages: Vec<Arc<dyn Rica>>,
    channels: Vec<Channel>,
}

impl Pipeline {
    pub fn new(stages: Vec<Arc<dyn Rica>>, channels: Vec<Channel>) -> Self {
        assert_eq!(
            stages.len().saturating_sub(1),
            channels.len(),
            "Pipeline needs N-1 channels for N stages"
        );
        Self { stages, channels }
    }
}

impl Rica for Pipeline {
    fn accelerate(
        &self,
        intent: Intent,
        register: Register,
    ) -> Pin<Box<dyn Future<Output = (Output, Register, Trace)> + Send + '_>> {
        Box::pin(async move {
            let mut current_intent = intent;
            let mut current_register = register;
            let mut combined_trace = Trace::new();

            for (i, stage) in self.stages.iter().enumerate() {
                let (output, reg, trace) = stage.accelerate(current_intent, current_register).await;

                combined_trace.cycles.extend(trace.cycles);
                current_register = reg;

                if i < self.stages.len() - 1 {
                    let channel = &self.channels[i];
                    current_register = (channel.isolate)(&current_register);
                    current_intent = Intent::new(output.text);
                } else {
                    return (output, current_register, combined_trace);
                }
            }

            // Unreachable (handled in loop), but satisfy the compiler
            (
                Output {
                    text: String::new(),
                    cycles: 0,
                    tokens: Default::default(),
                },
                current_register,
                combined_trace,
            )
        })
    }
}

// ============================================================================
// Parallel — concurrent composition
// ============================================================================

/// Parallel runs multiple Rica instances concurrently on the same intent
/// and merges their outputs.
pub struct Parallel {
    branches: Vec<Arc<dyn Rica>>,
}

impl Parallel {
    pub fn new(branches: Vec<Arc<dyn Rica>>) -> Self {
        Self { branches }
    }
}

impl Rica for Parallel {
    fn accelerate(
        &self,
        intent: Intent,
        register: Register,
    ) -> Pin<Box<dyn Future<Output = (Output, Register, Trace)> + Send + '_>> {
        Box::pin(async move {
            let futures: Vec<_> = self
                .branches
                .iter()
                .map(|branch| {
                    let branch_intent = Intent::new(format!("[branch] {}", intent.prompt));
                    let branch_reg = register.child();
                    branch.accelerate(branch_intent, branch_reg)
                })
                .collect();

            let results = futures::future::join_all(futures).await;

            let mut combined_trace = Trace::new();
            let mut all_texts = Vec::new();
            let mut total_cycles = 0u32;
            let mut total_tokens = crate::trace::TokenUsage::default();

            for (output, _reg, trace) in results {
                combined_trace.cycles.extend(trace.cycles);
                all_texts.push(output.text);
                total_cycles += output.cycles;
                total_tokens.prompt_tokens += output.tokens.prompt_tokens;
                total_tokens.completion_tokens += output.tokens.completion_tokens;
                total_tokens.total_tokens += output.tokens.total_tokens;
            }

            let output = Output {
                text: all_texts.join("\n---\n"),
                cycles: total_cycles,
                tokens: total_tokens,
            };

            (output, register, combined_trace)
        })
    }
}

// ============================================================================
// Ensemble — voting composition
// ============================================================================

/// Ensemble runs multiple Rica instances on the same intent and aggregates
/// their outputs using a custom aggregator function.
pub struct Ensemble {
    voters: Vec<Arc<dyn Rica>>,
    aggregator: Arc<dyn Fn(Vec<Output>) -> Output + Send + Sync>,
}

impl Ensemble {
    pub fn new(
        voters: Vec<Arc<dyn Rica>>,
        aggregator: Arc<dyn Fn(Vec<Output>) -> Output + Send + Sync>,
    ) -> Self {
        Self { voters, aggregator }
    }
}

impl Rica for Ensemble {
    fn accelerate(
        &self,
        intent: Intent,
        register: Register,
    ) -> Pin<Box<dyn Future<Output = (Output, Register, Trace)> + Send + '_>> {
        Box::pin(async move {
            let futures: Vec<_> = self
                .voters
                .iter()
                .map(|voter| {
                    let voter_intent = Intent::new(format!("[voter] {}", intent.prompt));
                    let voter_reg = register.child();
                    voter.accelerate(voter_intent, voter_reg)
                })
                .collect();

            let results = futures::future::join_all(futures).await;

            let mut combined_trace = Trace::new();
            let mut outputs = Vec::new();

            for (output, _reg, trace) in results {
                combined_trace.cycles.extend(trace.cycles);
                outputs.push(output);
            }

            let output = (self.aggregator)(outputs);
            (output, register, combined_trace)
        })
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::PipelineEngine;
    use crate::rica::DefaultRica;

    struct MockLlm {
        text: String,
    }

    impl crate::rica::LlmBackend for MockLlm {
        fn complete(
            &self,
            _fragments: &[&crate::fragment::Fragment],
            _tools: &[crate::tool::Tool],
        ) -> Pin<Box<dyn Future<Output = Result<crate::rica::LlmResponse, String>> + Send + '_>>
        {
            let text = self.text.clone();
            Box::pin(async move {
                Ok(crate::rica::LlmResponse {
                    text: Some(text),
                    tool_calls: vec![],
                    tokens: Default::default(),
                })
            })
        }

        fn summarize(
            &self,
            _fragments: &[&crate::fragment::Fragment],
        ) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send + '_>> {
            Box::pin(async move { Ok("summary".into()) })
        }
    }

    fn make_rica(text: &str) -> Arc<dyn Rica> {
        let llm = Arc::new(MockLlm {
            text: text.to_string(),
        });
        Arc::new(DefaultRica::new(Box::new(PipelineEngine::default()), llm))
    }

    #[tokio::test]
    async fn test_pipeline() {
        let a = make_rica("step A");
        let b = make_rica("step B");

        let pipeline = Pipeline::new(vec![a, b], vec![Channel::last_as_intent()]);

        let (output, _, _) = pipeline
            .accelerate(Intent::new("start"), Register::new("/tmp".into()))
            .await;

        assert_eq!(output.text, "step B");
    }

    #[tokio::test]
    async fn test_parallel() {
        let a = make_rica("result A");
        let b = make_rica("result B");

        let parallel = Parallel::new(vec![a, b]);

        let (output, _, _) = parallel
            .accelerate(Intent::new("start"), Register::new("/tmp".into()))
            .await;

        assert!(output.text.contains("result A"));
        assert!(output.text.contains("result B"));
    }
}
