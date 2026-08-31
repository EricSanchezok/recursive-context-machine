//! RCM Accelerator — composable execution via accelerator graphs.

pub mod accelerator;
pub mod assistant;
pub mod catalog;
pub mod condition;
pub mod environment;
pub mod flux;
pub mod graph;
pub mod lsp;
pub mod mcp;
pub mod policy;
pub mod prompts;
pub mod provider;
pub mod registry;
pub mod tools;
pub mod trajectory;
pub mod wire;

pub use accelerator::Accelerator;
pub use catalog::{Catalog, ResourceSelection, RuntimeResources};
pub use condition::{
    Condition, ConditionBranch, ContextPredicate, EnvironmentPredicate, Predicate,
    PurposePredicate, ResourcesPredicate,
};
pub use flux::{BridgeKind, ContextFlux, EnvFlux, Flux, FluxMode, PurposeFlux, ResFlux};
pub use graph::{Component, ComponentKind, Graph};
pub use machine::RunState;
pub use policy::Captain;
pub use provider::{MODEL_PRESETS, ModelPreset, PROVIDERS, Provider, ResolveError, resolve_model};
pub use tools::SpawnTool;
pub use wire::{Channel, ComponentId, ComponentRef, Endpoint, Port, PortOwner, Wire};
