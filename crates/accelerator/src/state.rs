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
            env: local(),
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
