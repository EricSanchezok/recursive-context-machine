//! RCM Accelerator — composable agent execution via dataflow graphs.

pub mod accelerator;
pub mod assembly;
pub mod catalog;
pub mod condition;
pub mod environment;
pub mod flux;
pub mod graph;
pub mod logging;
pub mod lsp;
pub mod mcp;
pub mod policy;
pub mod prompts;
pub mod resources;
pub mod state;
pub mod tools;

pub use accelerator::Accelerator;
pub use accelerator::{AcceleratorRef, Channel, Port};
pub use assembly::Assembly;
pub use catalog::Catalog;
pub use condition::{
    ConditionBranch, ConditionRef, ContextPredicate, EnvironmentPredicate, Predicate,
    PurposePredicate, ResourcesPredicate,
};
pub use flux::{ContextFlux, EnvFlux, FluxMode, FluxRef, PolicyFlux, PurposeFlux, ResFlux};
pub use graph::{BuildError, Graph};
pub use policy::Captain;
pub use state::State;
