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
    /// Generate one fixed, non-sensitive image for provider diagnostics.
    ImageCanary(ImageCanaryArgs),
}

/// --- Image canary --------------------------------------------------------

#[derive(Args)]
pub struct ImageCanaryArgs {
    /// Output path for the verified canary image.
    #[arg(long, required = true)]
    pub output: PathBuf,
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

    /// Read the purpose override from standard input.
    ///
    /// This keeps sensitive or large purpose text out of the process argument
    /// list. It cannot be combined with `--purpose`.
    #[arg(long, conflicts_with = "purpose")]
    pub purpose_stdin: bool,

    /// Use a specific run directory instead of auto-generating a timestamped one.
    /// Exported as RCM_RUN_DIR for subprocess/shell tools.
    #[arg(long)]
    pub run_dir: Option<PathBuf>,
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

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::{Cli, Command};

    #[test]
    fn purpose_stdin_is_accepted_for_run() {
        let cli = Cli::try_parse_from(["accelerate", "run", "workflow.rcm", "--purpose-stdin"])
            .expect("purpose stdin should parse");

        let Command::Run(args) = cli.command else {
            panic!("expected run command");
        };
        assert!(args.purpose_stdin);
    }

    #[test]
    fn purpose_stdin_conflicts_with_inline_purpose() {
        let result = Cli::try_parse_from([
            "accelerate",
            "run",
            "workflow.rcm",
            "--purpose",
            "secret",
            "--purpose-stdin",
        ]);

        assert!(result.is_err());
    }

    #[test]
    fn image_canary_requires_an_output_path() {
        let cli = Cli::try_parse_from(["accelerate", "image-canary", "--output", "canary.png"])
            .expect("image canary should parse");

        let Command::ImageCanary(args) = cli.command else {
            panic!("expected image canary command");
        };
        assert_eq!(args.output, std::path::Path::new("canary.png"));
    }
}
