use clap::Parser;

fn main() -> anyhow::Result<()> {
    cli::cmd::dispatch(cli::Cli::parse())
}
