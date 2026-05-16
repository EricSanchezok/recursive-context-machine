use std::future::Future;
use std::pin::Pin;

use machine::{Context, Environment, Machine, Policy, Resources};

/// An atomic agent — runs the context machine once.
pub struct Agent {
    pub(crate) purpose: String,
    pub(crate) ctx: Box<Context>,
    pub(crate) resources: Box<Resources>,
    pub(crate) env: Box<Environment>,
    pub(crate) policy: Box<dyn Policy>,
}

impl Agent {
    pub fn run(self) -> Pin<Box<dyn Future<Output = (Context, Resources, Environment)> + Send>> {
        Box::pin(async move {
            let mut ctx = *self.ctx;
            let mut env = *self.env;
            let mut resources = *self.resources;

            if !self.purpose.is_empty() && ctx.purpose.is_empty() {
                ctx.purpose.clone_from(&self.purpose);
            }

            let machine = Machine::new(self.policy);
            machine.run(&mut ctx, &mut env, &mut resources).await;
            (ctx, resources, env)
        })
    }
}
