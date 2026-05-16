use machine::{Context, Environment, Policy, Resources};

/// The runtime state of an agent — both input and output.
///
/// After [`crate::accelerator::fire()`], `policy` is consumed (set to `None`).
/// A spent state cannot be re-fired but its `ctx`, `env`, and `res` carry the results.
pub struct State {
    pub purpose: String,
    pub ctx: Context,
    pub env: Environment,
    pub policy: Option<Box<dyn Policy>>,
    pub res: Resources,
}

impl State {
    pub fn new(
        purpose: impl Into<String>,
        ctx: Context,
        env: Environment,
        policy: Box<dyn Policy>,
        res: Resources,
    ) -> Self {
        Self {
            purpose: purpose.into(),
            ctx,
            env,
            policy: Some(policy),
            res,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            purpose: String::new(),
            ctx: Context::new(),
            env: crate::local(),
            policy: Some(Box::new(crate::policy::Captain::new())),
            res: crate::kit(),
        }
    }
}
