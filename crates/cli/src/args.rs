use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "accelerate", version, about = "RCM accelerator CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Run the accelerator on a prompt.
    Run(RunArgs),
}

#[derive(Args)]
pub struct RunArgs {
    /// Prompt passed to the accelerator.
    #[arg(required = true, num_args = 1..)]
    pub prompt: Vec<String>,

    /// Delay between tape-machine animation steps, in milliseconds.
    #[arg(long, default_value_t = 50)]
    pub speed: u64,

    /// Output format after execution.
    #[arg(long, value_enum, default_value_t = Format::Text)]
    pub format: Format,

    /// Show the full context instead of only the final assistant message.
    #[arg(long)]
    pub context: bool,

    /// MCP server to launch.
    ///
    /// Stdio: `label=command arg1 arg2`
    /// HTTP:  `label=https://url|HeaderName:Value|HeaderName:Value`
    ///
    /// Repeatable — each `--mcp-server` adds one server.
    #[arg(long = "mcp-server")]
    pub mcp_servers: Vec<String>,

    /// Model selector in `<provider>/<model>` form (e.g. `sii/gpt-4.1`).
    ///
    /// Provider-only (e.g. `--model sii`) uses the provider's default model.
    /// Omit to auto-detect: the first provider whose env var is set wins.
    #[arg(long, value_name = "PROVIDER/MODEL")]
    pub model: Option<String>,
}

impl RunArgs {
    pub fn prompt_text(&self) -> String {
        self.prompt.join(" ")
    }
}

#[derive(Clone, Copy, ValueEnum)]
pub enum Format {
    Text,
    Json,
}
