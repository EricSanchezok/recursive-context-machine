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

fn local() -> Environment {
    let mut env = Environment::new(std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    env.root = Some(env.cwd.clone());
    env
}

fn kit() -> Resources {
    use crate::tools::builtin_tools;

    let mut resources = Resources::new();

    for tool in builtin_tools() {
        let name = tool.name().to_string();
        resources = resources.with_tool(tool);
        resources.enable(name);
    }

    resources.prompts.insert(
        "captain".to_string(),
        include_str!("prompts/captain.txt").to_string(),
    );

    resources = resources.with_model(crate::model::deepseek_v4_flash());
    resources = resources.with_model(crate::model::gpt4_1());

    resources
}
