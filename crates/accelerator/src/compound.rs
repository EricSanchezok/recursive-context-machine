use std::future::Future;
use std::pin::Pin;

use machine::{Context, Environment, Resources};

use crate::accelerator::Accelerator;
use crate::flux::Flux;

/// A compound node — schedules sub-units without running the machine.
pub enum Compound {
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

impl Compound {
    pub fn run(self) -> Pin<Box<dyn Future<Output = (Context, Resources, Environment)> + Send>> {
        Box::pin(execute(self))
    }
}

async fn execute(compound: Compound) -> (Context, Resources, Environment) {
    match compound {
        Compound::Then { first, next, flux } => {
            let (f_ctx, f_res, f_env) = first.run().await;
            let next = inject(next, f_ctx, f_res, f_env, &flux);
            next.run().await
        }
        Compound::And { left, right, flux } => {
            let (l_ctx, l_res, l_env) = left.run().await;
            let (r_ctx, r_res, r_env) = right.run().await;
            flux.fuse(l_ctx, l_res, l_env, r_ctx, r_res, r_env)
        }
    }
}

/// Walk to the first leaf and apply upstream state.
#[allow(clippy::boxed_local)]
fn inject(
    target: Box<Accelerator>,
    src_ctx: Context,
    src_res: Resources,
    src_env: Environment,
    flux: &Flux,
) -> Box<Accelerator> {
    match *target {
        Accelerator::Agent(agent) => {
            let (ctx, resources, env) = flux.shift(
                *agent.ctx,
                &src_ctx,
                *agent.resources,
                &src_res,
                *agent.env,
                &src_env,
            );
            Box::new(Accelerator::Agent(crate::agent::Agent {
                purpose: agent.purpose,
                ctx: Box::new(ctx),
                resources: Box::new(resources),
                env: Box::new(env),
                policy: agent.policy,
            }))
        }
        Accelerator::Compound(compound) => match *compound {
            Compound::Then {
                first,
                next,
                flux: inner,
            } => Box::new(Accelerator::Compound(Box::new(Compound::Then {
                first: inject(first, src_ctx, src_res, src_env, flux),
                next,
                flux: inner,
            }))),
            Compound::And {
                left,
                right,
                flux: inner,
            } => Box::new(Accelerator::Compound(Box::new(Compound::And {
                left: inject(left, src_ctx, src_res, src_env, flux),
                right,
                flux: inner,
            }))),
        },
    }
}
