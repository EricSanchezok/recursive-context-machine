mod args;
mod cmd;
mod hook;
mod output;

use clap::Parser;

fn main() -> anyhow::Result<()> {
    cmd::run(args::Cli::parse())
}
