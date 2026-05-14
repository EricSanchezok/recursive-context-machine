//! RICA Accelerator — lightweight entry for the Context Machine.
//!
//! The accelerator wires together:
//!   - a user **intent**
//!   - a **Policy** (one-shot or custom)
//!   - an **Environment** (cwd, vars)
//!   - **Resources** (models, tools)
//!
//! and runs the [`Machine`] against them.
//!
//! # Quick start
//!
//! ```no_run
//! # async fn example() {
//! use accelerator::Accelerator;
//!
//! let result = Accelerator::new("What is 3 + 5?")
//!     .run()
//!     .await;
//! # }
//! ```
//!
//! # Custom policy / env / tools
//!
//! ```no_run
//! # async fn example() {
//! use accelerator::Accelerator;
//! use machine::Environment;
//!
//! let result = Accelerator::new("What is 3 + 5?")
//!     .with_env(Environment::new("/tmp"))
//!     .run()
//!     .await;
//! # }
//! ```

mod policy;
mod tools;

use machine::{Context, Environment, Fragment, Machine, Model, Policy, Resources, Tool};
use policy::DefaultPolicy;
use tools::builtin_tools;

/// The accelerator entry point.
///
/// `intent` is the only required parameter — everything else has
/// sensible defaults or is additive.
pub struct Accelerator {
    intent: String,
    policy: Option<Box<dyn Policy>>,
    env: Option<Environment>,
    tools: Vec<Box<dyn Tool>>,
}

impl Accelerator {
    /// Create an accelerator with the given user intent.
    ///
    /// The intent becomes a `User` fragment at the head of the context.
    pub fn new(intent: impl Into<String>) -> Self {
        Self {
            intent: intent.into(),
            policy: None,
            env: None,
            tools: Vec::new(),
        }
    }

    /// Override the default policy.
    pub fn with_policy(mut self, policy: Box<dyn Policy>) -> Self {
        self.policy = Some(policy);
        self
    }

    /// Override the default environment.
    pub fn with_env(mut self, env: Environment) -> Self {
        self.env = Some(env);
        self
    }

    /// Register an additional tool (extends the built-in set).
    pub fn with_tool(mut self, tool: Box<dyn Tool>) -> Self {
        self.tools.push(tool);
        self
    }

    /// Build the context, environment, resources, and run the machine.
    ///
    /// Returns the final context after the machine loop terminates.
    pub async fn run(self) -> Context {
        let mut ctx = Context::new();
        ctx.append(Fragment::user(self.intent));

        let mut env = self.env.unwrap_or_else(|| Environment::new("."));

        let mut resources = Resources::new();

        // Register built-in tools + user tools.
        for t in builtin_tools().into_iter().chain(self.tools) {
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

        let policy = self
            .policy
            .unwrap_or_else(|| Box::new(DefaultPolicy::new()));
        let machine = Machine::new(policy);

        machine.run(&mut ctx, &mut env, &mut resources).await;
        ctx
    }
}
