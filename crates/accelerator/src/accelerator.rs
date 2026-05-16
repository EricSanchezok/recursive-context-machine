use std::future::Future;
use std::pin::Pin;

use machine::{Context, Environment, Policy, Resources};

use crate::agent::Agent;
use crate::compound::Compound;
use crate::flux::Flux;

/// A composable execution unit — either an atomic [`Agent`] or a
/// [`Compound`] node that schedules sub-units.
pub enum Accelerator {
    Agent(Agent),
    Compound(Box<Compound>),
}

impl Accelerator {
    pub fn agent(
        purpose: impl Into<String>,
        ctx: Context,
        resources: Resources,
        env: Environment,
        policy: Box<dyn Policy>,
    ) -> Self {
        Self::Agent(Agent {
            purpose: purpose.into(),
            ctx: Box::new(ctx),
            resources: Box::new(resources),
            env: Box::new(env),
            policy,
        })
    }

    pub fn then(self, target: Accelerator, flux: Flux) -> Self {
        Self::Compound(Box::new(Compound::Then {
            first: Box::new(self),
            next: Box::new(target),
            flux,
        }))
    }

    pub fn and(self, target: Accelerator, flux: Flux) -> Self {
        Self::Compound(Box::new(Compound::And {
            left: Box::new(self),
            right: Box::new(target),
            flux,
        }))
    }

    pub fn run(self) -> Pin<Box<dyn Future<Output = (Context, Resources, Environment)> + Send>> {
        Box::pin(async move {
            match self {
                Accelerator::Agent(agent) => agent.run().await,
                Accelerator::Compound(compound) => compound.run().await,
            }
        })
    }
}
