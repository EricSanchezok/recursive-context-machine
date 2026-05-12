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
    fn run(
        &self,
        intent: Intent,
        register: Register,
    ) -> Pin<Box<dyn Future<Output = (Output, Register, Trace)> + Send + '_>> {
        Box::pin(async move {
            let mut current_intent = intent;
            let mut current_register = register;
            let mut combined_trace = Trace::new();

            for (i, stage) in self.stages.iter().enumerate() {
                let (output, reg, trace) = stage.run(current_intent, current_register).await;

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

pub struct Parallel {
    branches: Vec<Arc<dyn Rica>>,
}

impl Parallel {
    pub fn new(branches: Vec<Arc<dyn Rica>>) -> Self {
        Self { branches }
    }
}

impl Rica for Parallel {
    fn run(
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
                    branch.run(branch_intent, branch_reg)
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
    fn run(
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
                    voter.run(voter_intent, voter_reg)
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
