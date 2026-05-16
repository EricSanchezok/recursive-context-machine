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

    pub fn run(
        self,
    ) -> Pin<Box<dyn Future<Output = (String, Context, Environment, Resources)> + Send>> {
        Box::pin(async move { fire(self.purpose, self.ctx, self.env, self.policy, self.res).await })
    }
}

pub(crate) async fn fire(
    purpose: String,
    mut ctx: Context,
    mut env: Environment,
    policy: Box<dyn Policy>,
    mut res: Resources,
) -> (String, Context, Environment, Resources) {
    ctx.purpose = purpose;
    let machine = Machine::new(policy);
    machine.run(&mut ctx, &mut env, &mut res).await;
    let out_purpose = ctx.purpose.clone();
    (out_purpose, ctx, env, res)
}

#[derive(Clone, Copy, Debug)]
pub struct AcceleratorRef {
    pub(crate) id: usize,
}

impl AcceleratorRef {
    pub fn purpose_out(&self) -> OutPin {
        OutPin::Purpose(NodeId::Accelerator(self.id))
    }
    pub fn ctx_out(&self) -> OutPin {
        OutPin::Context(NodeId::Accelerator(self.id))
    }
    pub fn env_out(&self) -> OutPin {
        OutPin::Environment(NodeId::Accelerator(self.id))
    }
    pub fn policy_out(&self) -> OutPin {
        OutPin::Policy(NodeId::Accelerator(self.id))
    }
    pub fn res_out(&self) -> OutPin {
        OutPin::Resources(NodeId::Accelerator(self.id))
    }
    pub fn done(&self) -> OutPin {
        OutPin::Pulse(NodeId::Accelerator(self.id))
    }

    pub fn purpose_in(&self) -> InPin {
        InPin::Purpose(NodeId::Accelerator(self.id))
    }
    pub fn ctx_in(&self) -> InPin {
        InPin::Context(NodeId::Accelerator(self.id))
    }
    pub fn env_in(&self) -> InPin {
        InPin::Environment(NodeId::Accelerator(self.id))
    }
    pub fn policy_in(&self) -> InPin {
        InPin::Policy(NodeId::Accelerator(self.id))
    }
    pub fn res_in(&self) -> InPin {
        InPin::Resources(NodeId::Accelerator(self.id))
    }
    pub fn run(&self) -> InPin {
        InPin::Pulse(NodeId::Accelerator(self.id))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeId {
    Accelerator(usize),
    Flux(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutPin {
    Purpose(NodeId),
    Context(NodeId),
    Environment(NodeId),
    Policy(NodeId),
    Resources(NodeId),
    Pulse(NodeId),
    FluxOut(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InPin {
    Purpose(NodeId),
    Context(NodeId),
    Environment(NodeId),
    Policy(NodeId),
    Resources(NodeId),
    Pulse(NodeId),
    FluxSlot(usize, usize),
}
