//! RICA Accelerator — lightweight entry for the Context Machine.
//!
//! The [`accelerate`] function wires together a user intent, context,
//! resources, environment, and policy, then runs the [`Machine`].

mod model;
pub mod policy;
pub mod tools;

use std::collections::HashMap;
use std::path::PathBuf;

use machine::{Context, Environment, Fragment, Machine, Policy, Resources};
use tracing::debug;

pub use model::nex_n1;
pub use policy::Captain;

/// Run the context machine with a user intent and optional overrides.
pub async fn accelerate(
    intent: impl Into<String>,
    ctx: Option<Context>,
    resources: Option<Resources>,
    env: Option<Environment>,
    policy: Option<Box<dyn Policy>>,
) -> Context {
    let intent = intent.into();

    let mut ctx = ctx.unwrap_or_default();
    debug!(
        intent = %intent,
        ctx_empty = ctx.is_empty(),
        "accelerate"
    );

    if ctx.is_empty() {
        ctx.append(Fragment::user(intent));
    }

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
    use crate::model::nex_n1;
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

    resources = resources.with_model(nex_n1());

    resources
}
