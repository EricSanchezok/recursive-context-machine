use crate::register::Register;
use crate::tape::Tape;

/// The next action the machine should take.
#[derive(Debug, Clone)]
pub enum Action {
    CallLlm,
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

/// Engine — the transition decision function.
///
/// Observes the current Tape + Register and decides the next Action.
/// This is the primary extension point: swap the Engine to change
/// how the machine decides what to do next.
pub trait Engine: Send + Sync {
    fn decide(&self, tape: &Tape, register: &Register) -> Action;
}

// ── PipelineEngine ──

/// A pipeline of decision stages. Each stage is evaluated in order;
/// the first to return `Some(action)` wins.
pub struct PipelineEngine {
    stages: Vec<Box<dyn EngineStage>>,
}

impl PipelineEngine {
    pub fn new(stages: Vec<Box<dyn EngineStage>>) -> Self {
        Self { stages }
    }

    /// Default pipeline: budget → prune → halt → llm.
    pub fn default() -> Self {
        Self::new(vec![
            Box::new(BudgetStage {
                max_tokens: 128_000,
            }),
            Box::new(PruneStage { max_fragments: 50 }),
            Box::new(HaltStage),
            Box::new(LlmStage),
        ])
    }
}

impl Engine for PipelineEngine {
    fn decide(&self, tape: &Tape, register: &Register) -> Action {
        for stage in &self.stages {
            if let Some(action) = stage.evaluate(tape, register) {
                return action;
            }
        }
        Action::CallLlm // fallback
    }
}

// ── EngineStage trait ──

pub trait EngineStage: Send + Sync {
    fn evaluate(&self, tape: &Tape, register: &Register) -> Option<Action>;
}

// ── Built-in stages ──

/// Triggers Compact when estimated token count exceeds budget.
pub struct BudgetStage {
    pub max_tokens: usize,
}

impl EngineStage for BudgetStage {
    fn evaluate(&self, tape: &Tape, _register: &Register) -> Option<Action> {
        let frags = tape.fragments();
        let estimated: usize = frags
            .iter()
            .map(|f| f.as_text().map(|t| t.len()).unwrap_or(0))
            .sum();
        // Rough heuristic: ~1 token per 4 chars
        if estimated / 4 > self.max_tokens {
            let len = frags.len();
            if len > 4 {
                // Compact the middle portion, keeping head and tail
                let from = len / 4;
                let to = len * 3 / 4;
                return Some(Action::Compact { from, to });
            }
        }
        None
    }
}

/// Triggers Prune when the tape has too many fragments.
pub struct PruneStage {
    pub max_fragments: usize,
}

impl EngineStage for PruneStage {
    fn evaluate(&self, tape: &Tape, _register: &Register) -> Option<Action> {
        let frags = tape.fragments();
        if frags.len() > self.max_fragments {
            // Prune the oldest fragments, keeping the most recent ones
            let excess = frags.len() - self.max_fragments;
            return Some(Action::Prune {
                from: 0,
                to: excess,
                reason: format!(
                    "tape length {} exceeds max {}",
                    frags.len(),
                    self.max_fragments
                ),
            });
        }
        None
    }
}

/// Halts when the last assistant fragment is plain text (no pending tool calls).
pub struct HaltStage;

impl EngineStage for HaltStage {
    fn evaluate(&self, tape: &Tape, _register: &Register) -> Option<Action> {
        let frags = tape.fragments();
        let last = frags.last()?;
        if last.role == crate::fragment::Role::Assistant {
            match &last.content {
                crate::fragment::Content::Text(_) | crate::fragment::Content::Reasoning(_) => {
                    return Some(Action::Halt {
                        reason: "assistant produced final text response".into(),
                    });
                }
                _ => {}
            }
        }
        None
    }
}

/// Default stage: call the LLM.
pub struct LlmStage;

impl EngineStage for LlmStage {
    fn evaluate(&self, _tape: &Tape, _register: &Register) -> Option<Action> {
        Some(Action::CallLlm)
    }
}

// ── High-level tape operations (composed from atomic ops) ──

/// Prune a range of cells: erase each cell in [from, to).
pub fn prune_range(tape: &mut Tape, from: usize, to: usize) {
    for pos in from..to {
        tape.goto(pos);
        tape.erase();
    }
}

/// Compact a range: erase [from, to), then write summary at `from`.
pub fn compact_range(tape: &mut Tape, from: usize, to: usize, summary: crate::fragment::Fragment) {
    prune_range(tape, from, to);
    tape.goto(from);
    tape.write(summary);
}

/// Append a fragment at the end of the tape.
pub fn append_fragment(tape: &mut Tape, fragment: crate::fragment::Fragment) {
    let end = tape.len();
    tape.goto(end);
    tape.write(fragment);
    tape.right();
}
