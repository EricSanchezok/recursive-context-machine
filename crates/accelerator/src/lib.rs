//! RCM Accelerator — composable agent execution via dataflow graphs.

pub mod accelerator;
pub mod assembly;
pub mod flux;
pub mod graph;
pub mod logging;
pub mod mcp;
pub mod model;
pub mod policy;
pub mod state;
pub mod tools;

pub use accelerator::Accelerator;
pub use accelerator::{AcceleratorRef, Channel, Port};
pub use assembly::Assembly;
pub use flux::{ContextFlux, EnvFlux, FluxMode, FluxRef, PurposeFlux, ResFlux};
pub use graph::{BuildError, Graph};
pub use state::State;
