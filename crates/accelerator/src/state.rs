use machine::{Context, Environment, Policy, Resources};

/// The runtime state of an agent — both input and output.
/// Fully cloneable: policy is cloned before being consumed by Machine.
#[derive(Clone)]
pub struct State {
    pub purpose: String,
    pub ctx: Context,
    pub env: Environment,
    pub policy: Box<dyn Policy>,
    pub res: Resources,
}

impl Default for State {
    fn default() -> Self {
        Self {
            purpose: String::new(),
            ctx: Context::new(),
            env: crate::local(),
            policy: Box::new(crate::policy::Captain::new()),
            res: crate::kit(),
        }
    }
}
