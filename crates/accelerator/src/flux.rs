use machine::{Context, Environment, Resources};

/// State transfer policy between agents in a composition.
///
/// Each dimension (context, resources, environment) can be independently
/// configured. Use [`Flux::ISOLATE`] for full isolation, [`Flux::PIPE`]
/// for full inheritance.
pub struct Flux {
    pub ctx: CtxFlux,
    pub resources: ResFlux,
    pub env: EnvFlux,
}

impl Flux {
    pub const ISOLATE: Self = Self {
        ctx: CtxFlux::Isolate,
        resources: ResFlux::Isolate,
        env: EnvFlux::Isolate,
    };

    pub const PIPE: Self = Self {
        ctx: CtxFlux::Prepend,
        resources: ResFlux::Inherit,
        env: EnvFlux::Inherit,
    };

    /// Shift upstream state onto downstream values (serial flow).
    pub fn shift(
        &self,
        target_ctx: Context,
        source_ctx: &Context,
        target_res: Resources,
        source_res: &Resources,
        target_env: Environment,
        source_env: &Environment,
    ) -> (Context, Resources, Environment) {
        let ctx = apply_ctx(target_ctx, source_ctx, &self.ctx);
        let resources = apply_res(target_res, source_res, &self.resources);
        let env = apply_env(target_env, source_env, &self.env);
        (ctx, resources, env)
    }

    /// Fuse two sets of state (parallel flow).
    pub fn fuse(
        &self,
        left_ctx: Context,
        left_res: Resources,
        left_env: Environment,
        right_ctx: Context,
        right_res: Resources,
        right_env: Environment,
    ) -> (Context, Resources, Environment) {
        let ctx = apply_ctx(left_ctx, &right_ctx, &self.ctx);
        let resources = apply_res(left_res, &right_res, &self.resources);
        let env = apply_env(left_env, &right_env, &self.env);
        (ctx, resources, env)
    }
}

pub enum CtxFlux {
    Isolate,
    Prepend,
    Append,
    Replace,
}

pub enum ResFlux {
    Isolate,
    Inherit,
    Merge,
}

pub enum EnvFlux {
    Isolate,
    Inherit,
}

fn apply_ctx(mut target: Context, source: &Context, flux: &CtxFlux) -> Context {
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

fn apply_res(mut target: Resources, source: &Resources, flux: &ResFlux) -> Resources {
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

fn apply_env(mut target: Environment, source: &Environment, flux: &EnvFlux) -> Environment {
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
