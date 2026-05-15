//! RICA Accelerator — lightweight entry for the Context Machine.
//!
//! The [`accelerate`] function wires together a user intent, context,
//! resources, environment, and policy, then runs the [`Machine`].
//!
//! # Quick start
//!
//! ```no_run
//! # async fn example() {
//! use accelerator::accelerate;
//!
//! let result = accelerate("What is 3 + 5?", None, None, None, None).await;
//! # }
//! ```
//!
//! # Custom resources
//!
//! ```no_run
//! # async fn example() {
//! use accelerator::{accelerate, kit};
//!
//! let result = accelerate(
//!     "What is 3 + 5?",
//!     None,
//!     Some(kit()),
//!     None,
//!     None,
//! ).await;
//! # }
//! ```

mod env;
mod model;
pub mod policy;
pub mod tools;

use machine::{Context, Environment, Machine, Policy, Resources};

pub use env::default_env;
pub use model::nex_n1;
pub use policy::Captain;

/// Run the context machine with a user intent and optional overrides.
///
/// # Parameters
///
/// - `intent` — the user's natural-language request (required).
///   Ignored when `ctx` is non-empty (continuing an existing conversation).
/// - `ctx` — an existing context. When `None` or empty, a fresh context
///   is created and `intent` is appended as a user fragment.
/// - `resources` — overrides the default [`kit`]. When `None`, the
///   built-in kit (tools, prompts, and model) is used.
/// - `env` — overrides the default environment. When `None`, `cwd = "."`.
/// - `policy` — overrides the default [`Captain`].
///
/// # Returns
///
/// The final [`Context`] after the machine loop terminates.
pub async fn accelerate(
    intent: impl Into<String>,
    ctx: Option<Context>,
    resources: Option<Resources>,
    env: Option<Environment>,
    policy: Option<Box<dyn Policy>>,
) -> Context {
    let intent = intent.into();

    let mut ctx = ctx.unwrap_or_default();
    if ctx.is_empty() {
        ctx.append(machine::Fragment::user(intent));
    }

    let mut env = env.unwrap_or_else(default_env);
    let mut resources = resources.unwrap_or_else(kit);

    let policy = policy.unwrap_or_else(|| Box::new(Captain::new()));
    let machine = Machine::new(policy);

    machine.run(&mut ctx, &mut env, &mut resources).await;
    ctx
}

/// Build the default resource kit — built-in tools, prompts, and model.
///
/// The kit includes:
///   - all built-in tools (activated by default)
///   - the default system prompt
///   - the Nex N1 model (active by default)
///
/// # Example
///
/// ```no_run
/// use accelerator::kit;
///
/// let resources = kit();
/// ```
pub fn kit() -> Resources {
    use crate::model::nex_n1;
    use crate::tools::builtin_tools;

    let mut resources = Resources::new();

    // Register and activate all built-in tools.
    for t in builtin_tools() {
        let name = t.name().to_string();
        resources = resources.with_tool(t);
        resources.catch_tool(name);
    }

    // Load default system prompt.
    resources.prompts.insert(
        "default".to_string(),
        include_str!("prompts/default.txt").to_string(),
    );

    // Register and activate the default model.
    resources = resources.with_model(nex_n1());
    resources.set_active_model("nex-agi/nex-n1");

    resources
}
