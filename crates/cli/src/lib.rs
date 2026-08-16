mod args;
pub mod cmd;
pub mod hook;
mod output;
pub mod rcm;

pub mod tape_animation {
    pub use crate::output::tape::{
        TapeSnapshot, TapeSnapshotCell, TapeSnapshotTape, snapshot_events,
    };
}

pub use args::{Cli, Command, Format, ImageCanaryArgs, RunArgs};
