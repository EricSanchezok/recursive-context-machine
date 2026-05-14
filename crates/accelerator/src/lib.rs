//! RICA Accelerator — lightweight entry for the Context Machine.
//!
//! The `accelerate` function wires together:
//!   - a user **intent**
//!   - a **Policy** (defaults to [`DefaultPolicy`])
//!   - an **Environment** (cwd, vars)
//!   - **Resources** (models, tools)
//!
//! and runs the [`Machine`] against them, returning the final [`Context`].
//!
//! # Quick start
//!
//! ```no_run
//! # async fn example() {
//! use accelerator::accelerate;
//!
//! let result = accelerate("What is 3 + 5?", None, None, vec![]).await;
//! # }
//! ```
//!
//! # Custom policy / env / tools
//!
//! ```no_run
//! # async fn example() {
//! use accelerator::accelerate;
//! use machine::Environment;
//!
//! let result = accelerate(
//!     "What is 3 + 5?",
//!     None,
//!     Some(Environment::new("/tmp")),
//!     vec![],
//! ).await;
//! # }
//! ```

mod policy;
mod tools;

use machine::{Context, Environment, Machine, Model, Policy, Resources, Tool};
use policy::DefaultPolicy;
use tools::builtin_tools;

/// Run the context machine with a user intent and optional overrides.
///
/// # Parameters
///
/// - `intent` — the user's natural-language request (required).
/// - `policy` — overrides the default [`DefaultPolicy`] when `Some`.
/// - `env` — overrides the default environment (`cwd = "."`) when `Some`.
/// - `tools` — additional tools beyond the built-in set.
///
/// # Returns
///
/// The final [`Context`] after the machine loop terminates.
pub async fn accelerate(
    intent: impl Into<String>,
    policy: Option<Box<dyn Policy>>,
    env: Option<Environment>,
    tools: Vec<Box<dyn Tool>>,
) -> Context {
    let intent = intent.into();

    let mut ctx = Context::new();

    let mut env = env.unwrap_or_else(|| Environment::new("."));

    let mut resources = Resources::new();

    // Register built-in tools + user tools.
    for t in builtin_tools().into_iter().chain(tools) {
        let name = t.name().to_string();
        resources = resources.with_tool(t);
        // Activate all registered tools by default.
        resources.catch_tool(name);
    }

    // Register a default model. The caller's Station / UI layer
    // should ultimately provide this; for now we use a hard-coded
    // OpenAI-compatible local endpoint.
    resources = resources.with_model(Model {
        name: "default".into(),
        protocol: machine::Protocol::OpenAI,
        endpoint: None, // falls back to OPENAI_BASE_URL or https://api.openai.com/v1
        credentials: None, // falls back to OPENAI_API_KEY
        ..Default::default()
    });
    resources.set_active_model("default");

    // Load default system prompt so the policy can read it in phase 1.
    resources.prompts.insert(
        "default".to_string(),
        include_str!("prompts/default.txt").to_string(),
    );

    let policy = policy.unwrap_or_else(|| Box::new(DefaultPolicy::new(intent)));
    let machine = Machine::new(policy);

    machine.run(&mut ctx, &mut env, &mut resources).await;
    ctx
}
