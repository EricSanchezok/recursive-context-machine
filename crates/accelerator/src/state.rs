use std::path::PathBuf;

use machine::{Context, Environment, Resources};

#[derive(Clone)]
pub struct State {
    pub purpose: String,
    pub ctx: Context,
    pub env: Environment,
    pub res: Resources,
}

impl Default for State {
    fn default() -> Self {
        Self {
            purpose: String::new(),
            ctx: Context::new(),
            env: Environment::new("."),
            res: Resources::new(),
        }
    }
}

/// An honest snapshot of the host the agent is currently running on:
/// inherits host env vars, cwd, and platform tag.
///
/// For sandboxed scenarios, use [`Environment::empty`].
pub fn local() -> Environment {
    Environment::new(std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
}
