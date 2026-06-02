use machine::{Context, Environment, Resources};

#[derive(Clone)]
pub struct State {
    pub purpose: String,
    /// For ContextFlux::Fold — carries the extracted last assistant text from
    /// upstream slots so the downstream accelerator can fold it into purpose_b.
    pub fold_payload: String,
    pub ctx: Context,
    pub env: Environment,
    pub res: Resources,
}

impl Default for State {
    fn default() -> Self {
        Self {
            purpose: String::new(),
            fold_payload: String::new(),
            ctx: Context::new(),
            env: Environment::new("."),
            res: Resources::new(),
        }
    }
}
