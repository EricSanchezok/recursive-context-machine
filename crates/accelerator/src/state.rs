use std::path::PathBuf;

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
            env: local(),
            policy: Box::new(crate::policy::Captain::new()),
            res: kit(),
        }
    }
}

pub fn local() -> Environment {
    let mut env = Environment::named(
        "local",
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
    );
    env.root = Some(env.cwd.clone());
    env
}

pub fn kit() -> Resources {
    let mut resources = Resources::named("kit");

    for tool in crate::tools::builtin_tools() {
        resources = resources.with_tool(tool);
    }

    resources.prompts.insert(
        "captain".to_string(),
        include_str!("prompts/captain.txt").to_string(),
    );

    resources
}
