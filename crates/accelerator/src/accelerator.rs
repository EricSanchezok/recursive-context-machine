use std::future::Future;
use std::pin::Pin;

use machine::{Context, Environment, Machine, Policy, Resources};

/// A single agent — runs the Context Machine.
pub struct Accelerator {
    pub(crate) purpose: String,
    pub(crate) ctx: Context,
    pub(crate) env: Environment,
    pub(crate) policy: Box<dyn Policy>,
    pub(crate) res: Resources,
}

impl Accelerator {
    pub fn new(
        purpose: impl Into<String>,
        ctx: Context,
        env: Environment,
        policy: Box<dyn Policy>,
        res: Resources,
    ) -> Self {
        Self {
            purpose: purpose.into(),
            ctx,
            env,
            policy,
            res,
        }
    }

    pub fn run(self) -> Pin<Box<dyn Future<Output = Output> + Send>> {
        Box::pin(async move { fire(self.purpose, self.ctx, self.env, self.policy, self.res).await })
    }
}

/// The result of running an agent.
pub struct Output {
    pub purpose: String,
    pub context: Context,
    pub environment: Environment,
    pub resources: Resources,
}

pub(crate) async fn fire(
    purpose: String,
    mut ctx: Context,
    mut env: Environment,
    policy: Box<dyn Policy>,
    mut res: Resources,
) -> Output {
    ctx.purpose = purpose;
    let machine = Machine::new(policy);
    machine.run(&mut ctx, &mut env, &mut res).await;
    let purpose = std::mem::take(&mut ctx.purpose);
    Output {
        purpose,
        context: ctx,
        environment: env,
        resources: res,
    }
}

// ── Graph wiring ──

#[derive(Clone, Copy, Debug)]
pub struct AcceleratorRef {
    pub(crate) id: usize,
}

impl AcceleratorRef {
    pub fn purpose_out(&self) -> Port {
        Port::Node(NodeId::Accelerator(self.id), Channel::Purpose)
    }
    pub fn ctx_out(&self) -> Port {
        Port::Node(NodeId::Accelerator(self.id), Channel::Context)
    }
    pub fn env_out(&self) -> Port {
        Port::Node(NodeId::Accelerator(self.id), Channel::Environment)
    }
    pub fn policy_out(&self) -> Port {
        Port::Node(NodeId::Accelerator(self.id), Channel::Policy)
    }
    pub fn res_out(&self) -> Port {
        Port::Node(NodeId::Accelerator(self.id), Channel::Resources)
    }
    pub fn done(&self) -> Port {
        Port::Node(NodeId::Accelerator(self.id), Channel::Pulse)
    }

    pub fn purpose_in(&self) -> Port {
        Port::Node(NodeId::Accelerator(self.id), Channel::Purpose)
    }
    pub fn ctx_in(&self) -> Port {
        Port::Node(NodeId::Accelerator(self.id), Channel::Context)
    }
    pub fn env_in(&self) -> Port {
        Port::Node(NodeId::Accelerator(self.id), Channel::Environment)
    }
    pub fn policy_in(&self) -> Port {
        Port::Node(NodeId::Accelerator(self.id), Channel::Policy)
    }
    pub fn res_in(&self) -> Port {
        Port::Node(NodeId::Accelerator(self.id), Channel::Resources)
    }
    pub fn run(&self) -> Port {
        Port::Node(NodeId::Accelerator(self.id), Channel::Pulse)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NodeId {
    Accelerator(usize),
    Flux(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Channel {
    Purpose,
    Context,
    Environment,
    Policy,
    Resources,
    Pulse,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Port {
    Node(NodeId, Channel),
    FluxOut(usize, Channel),
    FluxSlot(usize, usize, Channel),
}

impl Port {
    pub fn is_output(&self) -> bool {
        matches!(
            self,
            Port::Node(NodeId::Accelerator(_), _) | Port::FluxOut(_, _)
        )
    }

    pub fn is_input(&self) -> bool {
        matches!(
            self,
            Port::Node(NodeId::Accelerator(_), _) | Port::FluxSlot(_, _, _)
        )
    }

    pub fn channel(&self) -> Channel {
        match self {
            Port::Node(_, ch) => *ch,
            Port::FluxOut(_, ch) => *ch,
            Port::FluxSlot(_, _, ch) => *ch,
        }
    }

    pub(crate) fn node_index(&self, num_accelerators: usize) -> usize {
        let flux_offset = |id: usize| num_accelerators + id;
        match self {
            Port::Node(NodeId::Accelerator(id), _) => *id,
            Port::Node(NodeId::Flux(id), _) => flux_offset(*id),
            Port::FluxOut(id, _) => flux_offset(*id),
            Port::FluxSlot(id, _, _) => flux_offset(*id),
        }
    }
}
