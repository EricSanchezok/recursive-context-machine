use std::future::Future;
use std::pin::Pin;

use machine::{Context, Environment, Machine, Policy, Resources};

use crate::flux::{CtxFlux, EnvFlux, Flux, ResFlux};

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
            let next = apply_flux(*next, &f_ctx, &f_res, &f_env, &flux);
            next.run().await
        }

        Accelerator::And { left, right, flux } => {
            let (l_ctx, l_res, l_env) = left.run().await;
            let (r_ctx, r_res, r_env) = right.run().await;
            merge_results(l_ctx, r_ctx, l_res, r_res, l_env, r_env, &flux)
        }
    }
}

fn apply_flux(
    target: Accelerator,
    source_ctx: &Context,
    source_res: &Resources,
    source_env: &Environment,
    flux: &Flux,
) -> Accelerator {
    match target {
        Accelerator::Agent {
            purpose,
            ctx,
            resources,
            env,
            policy,
        } => {
            let ctx = apply_ctx_flux(*ctx, source_ctx, &flux.ctx);
            let resources = apply_res_flux(*resources, source_res, &flux.resources);
            let env = apply_env_flux(*env, source_env, &flux.env);
            Accelerator::Agent {
                purpose,
                ctx: Box::new(ctx),
                resources: Box::new(resources),
                env: Box::new(env),
                policy,
            }
        }
        Accelerator::Then {
            first,
            next,
            flux: inner,
        } => Accelerator::Then {
            first: Box::new(apply_flux(*first, source_ctx, source_res, source_env, flux)),
            next,
            flux: inner,
        },
        Accelerator::And {
            left,
            right,
            flux: inner,
        } => Accelerator::And {
            left: Box::new(apply_flux(*left, source_ctx, source_res, source_env, flux)),
            right,
            flux: inner,
        },
    }
}

fn apply_ctx_flux(mut target: Context, source: &Context, flux: &CtxFlux) -> Context {
    match flux {
        CtxFlux::Isolate => target,
        CtxFlux::Prepend => {
            let mut merged = Context::new();
            for frag in source.fragments().iter() {
                merged.append(frag.clone());
            }
            for frag in target.fragments().iter() {
                merged.append(frag.clone());
            }
            merged
        }
        CtxFlux::Append => {
            for frag in source.fragments().iter() {
                target.append(frag.clone());
            }
            target
        }
        CtxFlux::Replace => source.clone(),
    }
}

fn apply_res_flux(mut target: Resources, source: &Resources, flux: &ResFlux) -> Resources {
    match flux {
        ResFlux::Isolate => target,
        ResFlux::Inherit => {
            for (name, model) in &source.models {
                target.models.insert(name.clone(), model.clone());
            }
            target.active_model.clone_from(&source.active_model);
            for name in &source.active_tools {
                target.active_tools.insert(name.clone());
            }
            for (name, prompt) in &source.prompts {
                target.prompts.insert(name.clone(), prompt.clone());
            }
            target
        }
        ResFlux::Merge => {
            for (name, model) in &source.models {
                target
                    .models
                    .entry(name.clone())
                    .or_insert_with(|| model.clone());
            }
            if target.active_model.is_empty() {
                target.active_model.clone_from(&source.active_model);
            }
            for name in &source.active_tools {
                target.active_tools.insert(name.clone());
            }
            for (name, prompt) in &source.prompts {
                target
                    .prompts
                    .entry(name.clone())
                    .or_insert_with(|| prompt.clone());
            }
            target
        }
    }
}

fn apply_env_flux(mut target: Environment, source: &Environment, flux: &EnvFlux) -> Environment {
    match flux {
        EnvFlux::Isolate => target,
        EnvFlux::Inherit => {
            target.cwd.clone_from(&source.cwd);
            target.root.clone_from(&source.root);
            for (k, v) in &source.vars {
                target.vars.insert(k.clone(), v.clone());
            }
            target
        }
    }
}

fn merge_results(
    l_ctx: Context,
    r_ctx: Context,
    l_res: Resources,
    r_res: Resources,
    l_env: Environment,
    r_env: Environment,
    flux: &Flux,
) -> (Context, Resources, Environment) {
    let ctx = apply_ctx_flux(l_ctx, &r_ctx, &flux.ctx);
    let resources = apply_res_flux(l_res, &r_res, &flux.resources);
    let env = apply_env_flux(l_env, &r_env, &flux.env);
    (ctx, resources, env)
}
