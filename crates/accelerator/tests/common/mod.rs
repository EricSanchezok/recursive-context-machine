use std::future::Future;
use std::pin::Pin;

use accelerator::{Accelerator, State};
use machine::{Action, Context, Environment, Inbox, Policy, Purpose, Resources, ToolRuntime};

#[derive(Clone)]
pub struct DonePolicy;

impl Policy for DonePolicy {
    fn clone_box(&self) -> Box<dyn Policy> {
        Box::new(self.clone())
    }

    fn decide<'a>(
        &'a self,
        _purpose: &'a Purpose,
        _ctx: &'a Context,
        _env: &'a Environment,
        _resources: &'a Resources,
        _inbox: &'a Inbox,
    ) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>> {
        Box::pin(async { Action::Done })
    }
}

pub fn primitive(purpose: &str) -> Accelerator {
    Accelerator::primitive(
        State {
            purpose: purpose.to_string(),
            ..State::default()
        },
        Box::new(DonePolicy),
        ToolRuntime::new(),
        purpose,
    )
}
