use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "accelerate", version, about = "RCM accelerator CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Compile and run a .rcm file.
    Run(RunArgs),
    /// Parse a .rcm file and output its AST as JSON.
    Parse(ParseArgs),
    /// Output available policies, tools, prompts, models, and MCP servers as JSON.
    Inventory(InventoryArgs),
    /// Render a template for a GitHub event into a runnable .rcm file.
    Dispatch(DispatchArgs),
}

/// --- Run ----------------------------------------------------------------

#[derive(Args)]
pub struct RunArgs {
    /// Path to a .rcm file.
    #[arg(required = true)]
    pub file: PathBuf,

    /// Delay between tape-machine animation steps, in milliseconds.
    #[arg(long, default_value_t = 50)]
    pub speed: u64,

    /// Output format.
    #[arg(long, value_enum, default_value_t = Format::Text)]
    pub format: Format,

    /// Show the full context instead of only the final assistant message.
    #[arg(long)]
    pub context: bool,

    /// Stream hook events as JSON lines to stdout instead of the tape animation.
    #[arg(long)]
    pub stream: bool,

    /// Override the purpose declared in the .rcm file.
    #[arg(long)]
    pub purpose: Option<String>,
}

/// --- Parse ---------------------------------------------------------------

#[derive(Args)]
pub struct ParseArgs {
    /// Path to a .rcm file.
    #[arg(required = true)]
    pub file: PathBuf,
}

/// --- Inventory -----------------------------------------------------------

#[derive(Args)]
pub struct InventoryArgs {
    /// Project directory to scan for prompts/ and .rcm files.
    #[arg(default_value = ".")]
    pub project: PathBuf,
}

/// --- Dispatch ------------------------------------------------------------

#[derive(Args)]
pub struct DispatchArgs {
    /// Path to dispatch.toml describing routes from (event, action) to templates.
    #[arg(long)]
    pub config: PathBuf,

    /// GitHub event name (e.g., "issues", "pull_request"). Typically $GITHUB_EVENT_NAME.
    #[arg(long)]
    pub event_name: String,

    /// Event action (e.g., "opened"). Typically extracted from the event payload's "action" field.
    #[arg(long, default_value = "")]
    pub action: String,

    /// Path to the GitHub event JSON payload. Typically $GITHUB_EVENT_PATH.
    #[arg(long)]
    pub event_path: PathBuf,

    /// Print human-readable progress to stderr.
    #[arg(long)]
    pub verbose: bool,
}

/// --- Shared --------------------------------------------------------------

#[derive(Clone, Copy, ValueEnum)]
pub enum Format {
    Text,
    Json,
}
