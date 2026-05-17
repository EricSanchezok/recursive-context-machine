use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "accelerate", version, about = "RICA accelerator CLI")]
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
