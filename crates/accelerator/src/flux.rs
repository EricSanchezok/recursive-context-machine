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
