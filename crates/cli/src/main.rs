use clap::Parser;

fn main() -> anyhow::Result<()> {
    cli::cmd::run(cli::Cli::parse())
}
