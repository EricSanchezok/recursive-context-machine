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

/// --- Shared --------------------------------------------------------------

#[derive(Clone, Copy, ValueEnum)]
pub enum Format {
    Text,
    Json,
}
