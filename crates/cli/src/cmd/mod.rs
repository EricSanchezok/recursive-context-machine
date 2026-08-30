pub mod dispatch;
mod image_canary;
mod inventory;
mod parse;
pub mod report;
pub mod run;

use crate::args::{Cli, Command};

pub fn dispatch(cli: Cli) -> anyhow::Result<()> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    match cli.command {
        Command::Run(args) => runtime.block_on(run::run(args)),
        Command::Parse(args) => runtime.block_on(parse::run(args)),
        Command::Inventory(args) => inventory::run(args),
        Command::Dispatch(args) => dispatch::run(args),
        Command::ImageCanary(args) => runtime.block_on(image_canary::run(args)),
    }
}
