use machine::{Context, Environment, Policy, Resources};

/// The runtime state of an agent — the four dimensions that flow through the graph.
///
/// Created via [`State::new`] for full control or [`State::default`] for quick use.
pub struct State {
    pub ctx: Context,
    pub env: Environment,
    pub policy: Box<dyn Policy>,
    pub res: Resources,
}

impl State {
    pub fn new(ctx: Context, env: Environment, policy: Box<dyn Policy>, res: Resources) -> Self {
        Self {
            ctx,
            env,
            policy,
            res,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            ctx: Context::new(),
            env: crate::local(),
            policy: Box::new(crate::policy::Captain::new()),
            res: crate::kit(),
        }
    }
}
