//! RICA Accelerator — lightweight entry for the Context Machine.
//!
//! The [`accelerate`] function wires together a user intent, context,
//! resources, environment, and policy, then runs the [`Machine`].

mod model;
pub mod policy;
pub mod tools;
pub mod unit;

use std::collections::HashMap;
use std::path::PathBuf;

use machine::{Context, Environment, Machine, Policy, Resources};
use tracing::debug;

pub use model::{gpt4_1, nex_n1};
pub use policy::Captain;
pub use unit::Accelerator;

/// Run the context machine with a user intent and optional overrides.
///
/// When `ctx` is `None` or empty, a fresh context is created.
/// `resources` defaults to [`kit()`] (tools + prompts + GPT-4.1 model).
/// `env` defaults to [`local()`] (cwd and root set to `.`).
/// `policy` defaults to [`Captain`].
pub async fn accelerate(
    intent: impl Into<String> + std::fmt::Display,
    ctx: Option<Context>,
    resources: Option<Resources>,
    env: Option<Environment>,
    policy: Option<Box<dyn Policy>>,
) -> Context {
    let mut ctx = ctx.unwrap_or_default();
    debug!(
        intent = %intent,
        ctx_empty = ctx.is_empty(),
        "accelerate"
    );

    let mut env = env.unwrap_or_else(local);
    let mut resources = resources.unwrap_or_else(kit);

    let policy = policy.unwrap_or_else(|| Box::new(Captain::new()));
    let machine = Machine::new(policy);

    machine.run(&mut ctx, &mut env, &mut resources).await;
    debug!(fragments = ctx.len(), "accelerate done");
    ctx
}

/// Create an environment with `cwd` and `root` set to the current directory.
///
/// Tools see `.` as both their working directory and filesystem boundary.
///
/// ```no_run
/// use accelerator::local;
/// let env = local();
/// ```
pub fn local() -> Environment {
    Environment {
        cwd: PathBuf::from("."),
        vars: HashMap::new(),
        root: Some(PathBuf::from(".")),
    }
}

/// Build the default resource kit — built-in tools, prompts, and model.
///
/// ```no_run
/// use accelerator::kit;
/// let resources = kit();
/// ```
pub fn kit() -> Resources {
    use crate::tools::builtin_tools;

    let mut resources = Resources::new();

    for tool in builtin_tools() {
        let name = tool.name().to_string();
        resources = resources.with_tool(tool);
        resources.enable(name);
    }

    resources.prompts.insert(
        "default".to_string(),
        include_str!("prompts/default.txt").to_string(),
    );

    resources = resources.with_model(gpt4_1());

    resources
}
