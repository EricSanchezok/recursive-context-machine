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
    /// Run a .rcm file.
    Run(RunArgs),
}

#[derive(Args)]
pub struct RunArgs {
    /// Path to a .rcm file.
    #[arg(required = true)]
    pub file: PathBuf,

    /// Delay between tape-machine animation steps, in milliseconds.
    #[arg(long, default_value_t = 50)]
    pub speed: u64,

    /// Output format after execution.
    #[arg(long, value_enum, default_value_t = Format::Text)]
    pub format: Format,

    /// Show the full context instead of only the final assistant message.
    #[arg(long)]
    pub context: bool,
}

impl RunArgs {
    pub fn prompt_text(&self) -> String {
        self.file.display().to_string()
    }
}

#[derive(Clone, Copy, ValueEnum)]
pub enum Format {
    Text,
    Json,
}
