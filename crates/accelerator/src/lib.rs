//! RICA Accelerator — composable agent execution via dataflow graphs.

pub mod accelerator;
pub mod assembly;
pub mod flux;
pub mod graph;
mod model;
pub mod policy;
pub mod tools;

pub use accelerator::Accelerator;
pub use accelerator::{AcceleratorRef, InPin, NodeId, OutPin};
pub use assembly::Assembly;
pub use flux::FluxRef;
pub use graph::{BuildError, Graph};
pub use model::{gpt4_1, nex_n1};
pub use policy::Captain;

use std::collections::HashMap;
use std::path::PathBuf;

use machine::Environment;

/// Create an environment with `cwd` and `root` set to the current directory.
pub fn local() -> Environment {
    Environment {
        cwd: PathBuf::from("."),
        vars: HashMap::new(),
        root: Some(PathBuf::from(".")),
    }
}

/// Build the default resource kit — built-in tools, prompts, and model.
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
