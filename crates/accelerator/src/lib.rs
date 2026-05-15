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
mod resources;
pub mod tools;

use machine::{Context, Environment, Machine, Policy, Resources};

pub use env::default_env;
pub use model::nex_n1;
pub use policy::Captain;
pub use resources::kit;

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
