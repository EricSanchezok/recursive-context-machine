use std::future::Future;
use std::pin::Pin;

use machine::{Context, Environment, Machine, Policy, Resources};

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
    },
    And {
        left: Box<Accelerator>,
        right: Box<Accelerator>,
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
        }
    }

    pub fn and(self, other: Accelerator) -> Self {
        Self::And {
            left: Box::new(self),
            right: Box::new(other),
        }
    }

    pub fn run(self) -> Pin<Box<dyn Future<Output = (Context, Resources, Environment)> + Send>> {
        Box::pin(execute(self))
    }

    fn with_ctx(self, prefix: Context) -> Self {
        match self {
            Self::Agent {
                purpose,
                ctx,
                resources,
                env,
                policy,
            } => {
                let mut merged = Context::new();
                for frag in prefix.fragments().iter() {
                    merged.append(frag.clone());
                }
                for frag in ctx.fragments().iter() {
                    merged.append(frag.clone());
                }
                Self::Agent {
                    purpose,
                    ctx: Box::new(merged),
                    resources,
                    env,
                    policy,
                }
            }
            Self::Then { first, next } => Self::Then {
                first: Box::new(first.with_ctx(prefix)),
                next,
            },
            Self::And { left, right } => Self::And {
                left: Box::new(left.with_ctx(prefix)),
                right,
            },
        }
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

            // Stamp the purpose onto the context before machine starts,
            // so every policy can read it during its decide cycle.
            if !purpose.is_empty() && ctx.purpose.is_empty() {
                ctx.purpose.clone_from(&purpose);
            }

            let machine = Machine::new(policy);
            machine.run(&mut ctx, &mut env, &mut resources).await;
            (ctx, resources, env)
        }
        Accelerator::Then { first, next } => {
            let (ctx, _, _) = first.run().await;
            next.with_ctx(ctx).run().await
        }
        Accelerator::And { left, right } => {
            let (l_ctx, l_res, _) = left.run().await;
            let (r_ctx, _, _) = right.run().await;
            (merge_contexts(l_ctx, r_ctx), l_res, Environment::new("."))
        }
    }
}

fn merge_contexts(left: Context, right: Context) -> Context {
    let mut merged = Context::new();
    for frag in left.fragments().iter() {
        merged.append(frag.clone());
    }
    for frag in right.fragments().iter() {
        merged.append(frag.clone());
    }
    merged
}
