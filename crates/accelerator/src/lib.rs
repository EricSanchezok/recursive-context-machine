//! RICA Accelerator — composable agent execution.
//!
//! The [`Accelerator`] enum is the only entry point. Use
//! [`Accelerator::agent`] to create an agent, then chain with
//! `.then()` and `.and()` to build execution trees.

pub mod agent;
pub mod flux;
mod model;
pub mod policy;
pub mod tools;

use std::collections::HashMap;
use std::path::PathBuf;

use machine::Environment;

pub use agent::Accelerator;
pub use model::{gpt4_1, nex_n1};
pub use policy::Captain;

/// Create an environment with `cwd` and `root` set to the current directory.
///
/// Tools see `.` as both their working directory and filesystem boundary.
///
/// ```no_run
/// use accelerator::local;
/// let env = local();
/// ```
pub fn local() -> Environment {
    Environment {
        cwd: PathBuf::from("."),
        vars: HashMap::new(),
        root: Some(PathBuf::from(".")),
    }
}

/// Build the default resource kit — built-in tools, prompts, and model.
///
/// ```no_run
/// use accelerator::kit;
/// let resources = kit();
/// ```
pub fn kit() -> machine::Resources {
    use crate::tools::builtin_tools;

    let mut resources = machine::Resources::new();

    for tool in builtin_tools() {
        let name = tool.name().to_string();
        resources = resources.with_tool(tool);
        resources.enable(name);
    }

    resources.prompts.insert(
        "default".to_string(),
        include_str!("prompts/default.txt").to_string(),
    );

    resources = resources.with_model(gpt4_1());

    resources
}
