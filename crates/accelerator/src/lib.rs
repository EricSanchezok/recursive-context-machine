//! RCM Accelerator — composable execution via accelerator graphs.

pub mod accelerator;
pub mod catalog;
pub mod condition;
pub mod environment;
pub mod flux;
pub mod graph;
pub mod lsp;
pub mod mcp;
pub mod policy;
pub mod prompts;
pub mod state;
pub mod tools;
pub mod wire;

pub use accelerator::{Accelerator, GatherSpec, ScatterSpec};
pub use catalog::{Catalog, ResourceSelection, RuntimeResources};
pub use condition::{
    Condition, ConditionBranch, ContextPredicate, EnvironmentPredicate, Predicate,
    PurposePredicate, ResourcesPredicate,
};
pub use flux::{ContextFlux, EnvFlux, Flux, FluxMode, PurposeFlux, ResFlux};
pub use graph::{Component, ComponentKind, Graph};
pub use policy::Captain;
pub use state::State;
pub use wire::{Channel, ComponentId, ComponentRef, Endpoint, Port, PortOwner, Wire};
