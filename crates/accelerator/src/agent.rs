use std::future::Future;
use std::pin::Pin;

use machine::{Context, Environment, Machine, Policy, Resources};

use crate::flux::Flux;

pub enum Accelerator {
    Agent {
        purpose: String,
        ctx: Box<Context>,
        resources: Box<Resources>,
        env: Box<Environment>,
        policy: Box<dyn Policy>,
    },
    Then {
        first: Box<Accelerator>,
        next: Box<Accelerator>,
        flux: Flux,
    },
    And {
        left: Box<Accelerator>,
        right: Box<Accelerator>,
        flux: Flux,
    },
}

impl Accelerator {
    pub fn agent(
        purpose: impl Into<String>,
        ctx: Context,
        resources: Resources,
        env: Environment,
        policy: Box<dyn Policy>,
    ) -> Self {
        Self::Agent {
            purpose: purpose.into(),
            ctx: Box::new(ctx),
            resources: Box::new(resources),
            env: Box::new(env),
            policy,
        }
    }

    pub fn then(self, next: Accelerator) -> Self {
        Self::Then {
            first: Box::new(self),
            next: Box::new(next),
            flux: Flux::ISOLATE,
        }
    }

    pub fn then_with(self, next: Accelerator, flux: Flux) -> Self {
        Self::Then {
            first: Box::new(self),
            next: Box::new(next),
            flux,
        }
    }

    pub fn and(self, other: Accelerator) -> Self {
        Self::And {
            left: Box::new(self),
            right: Box::new(other),
            flux: Flux::ISOLATE,
        }
    }

    pub fn and_with(self, other: Accelerator, flux: Flux) -> Self {
        Self::And {
            left: Box::new(self),
            right: Box::new(other),
            flux,
        }
    }

    pub fn run(self) -> Pin<Box<dyn Future<Output = (Context, Resources, Environment)> + Send>> {
        Box::pin(execute(self))
    }
}

async fn execute(acc: Accelerator) -> (Context, Resources, Environment) {
    match acc {
        Accelerator::Agent {
            purpose,
            ctx,
            resources,
            env,
            policy,
            ..
        } => {
            let mut ctx = *ctx;
            let mut env = *env;
            let mut resources = *resources;

            if !purpose.is_empty() && ctx.purpose.is_empty() {
                ctx.purpose.clone_from(&purpose);
            }

            let machine = Machine::new(policy);
            machine.run(&mut ctx, &mut env, &mut resources).await;
            (ctx, resources, env)
        }

        Accelerator::Then { first, next, flux } => {
            let (f_ctx, f_res, f_env) = first.run().await;
            let next = shift(next, f_ctx, f_res, f_env, &flux);
            next.run().await
        }

        Accelerator::And { left, right, flux } => {
            let (l_ctx, l_res, l_env) = left.run().await;
            let (r_ctx, r_res, r_env) = right.run().await;
            flux.fuse(l_ctx, l_res, l_env, r_ctx, r_res, r_env)
        }
    }
}

/// Walk to the first agent and merge upstream state via flux.
#[allow(clippy::boxed_local)]
fn shift(
    target: Box<Accelerator>,
    src_ctx: Context,
    src_res: Resources,
    src_env: Environment,
    flux: &Flux,
) -> Box<Accelerator> {
    match *target {
        Accelerator::Agent {
            purpose,
            ctx,
            resources,
            env,
            policy,
        } => {
            let (ctx, resources, env) =
                flux.shift(*ctx, &src_ctx, *resources, &src_res, *env, &src_env);
            Box::new(Accelerator::Agent {
                purpose,
                ctx: Box::new(ctx),
                resources: Box::new(resources),
                env: Box::new(env),
                policy,
            })
        }
        Accelerator::Then {
            first,
            next,
            flux: inner,
        } => Box::new(Accelerator::Then {
            first: shift(first, src_ctx, src_res, src_env, flux),
            next,
            flux: inner,
        }),
        Accelerator::And {
            left,
            right,
            flux: inner,
        } => Box::new(Accelerator::And {
            left: shift(left, src_ctx, src_res, src_env, flux),
            right,
            flux: inner,
        }),
    }
}
