mod run;

use crate::args::{Cli, Command};

pub fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Command::Run(args) => run::run(args),
    }
}
