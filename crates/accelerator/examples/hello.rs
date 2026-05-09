//! RICA example: simple LLM call using rig.
//!
//! Usage:
//!   export OPENAI_API_KEY=sk-...
//!   cargo run --example hello -p accelerator

use rig::client::{CompletionClient, ProviderClient};
use rig::completion::Prompt;
use rig::providers::openai;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let client = openai::Client::from_env()?;
    let agent = client.agent("gpt-4o").build();
    let response = agent.prompt("用一句话介绍你自己").await?;
    println!("{response}");
    Ok(())
}
