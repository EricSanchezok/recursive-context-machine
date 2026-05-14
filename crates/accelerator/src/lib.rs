pub mod policy;
pub mod reactor;
pub mod station;
pub mod tool;

use std::collections::HashMap;
use std::sync::Mutex;

use machine::{Context, Environment, Fragment, Machine, Resources};

use crate::policy::DefaultPolicy;
use crate::reactor::LLMReactor;

/// In-memory context store.
static CONTEXTS: std::sync::LazyLock<Mutex<HashMap<String, Context>>> =
    std::sync::LazyLock::new(|| Mutex::new(HashMap::new()));

/// Run an accelerator with the given intent.
///
/// - `intent`: The user's request.
/// - `station_url`: URL of the Station configuration service.
/// - `ctx_id`: Optional context ID for continuing a previous session.
/// - `policy`: Optional custom Policy.
pub async fn accelerate(
    intent: String,
    station_url: &str,
    ctx_id: Option<&str>,
    policy: Option<Box<dyn machine::Policy>>,
) -> Result<Context, String> {
    // Fetch configuration from Station
    let client = station::Client::new(station_url);
    let config = client.config().await?;

    // Build Resources
    let mut resources = Resources::new().with_model(config.model);
    for tool_def in &config.tools {
        let tool = crate::tool::ShellTool::new(tool_def, station_url);
        resources = resources.with_tool(Box::new(tool));
    }

    // Build Environment
    let mut env = Environment::new(std::env::current_dir().unwrap_or_else(|_| "/".into()));

    // Build or retrieve Context
    let mut ctx = match ctx_id {
        Some(id) => CONTEXTS.lock().unwrap().remove(id).unwrap_or_default(),
        None => Context::new(),
    };

    // Add system prompt and user intent
    ctx.append(Fragment::system(&config.system_prompt));
    ctx.append(Fragment::user(&intent));

    // Build Machine
    let policy: Box<dyn machine::Policy> = policy.unwrap_or_else(|| Box::new(DefaultPolicy::new()));
    let reactor = Box::new(LLMReactor::new());
    let machine = Machine::new(policy, reactor);

    // Run
    machine.run(&mut ctx, &mut env, &mut resources).await;

    // Store context if ctx_id was provided
    if let Some(id) = ctx_id {
        CONTEXTS.lock().unwrap().insert(id.to_string(), ctx.clone());
    }

    Ok(ctx)
}
