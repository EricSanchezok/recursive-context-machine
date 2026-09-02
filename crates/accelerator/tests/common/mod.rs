use std::future::Future;
use std::pin::Pin;

use accelerator::Accelerator;
use machine::{Action, Policy, PolicyView, Purpose, RunState, ToolRuntime};

#[derive(Clone)]
pub struct DonePolicy;

impl Policy for DonePolicy {
    fn clone_box(&self) -> Box<dyn Policy> {
        Box::new(self.clone())
    }

    fn decide<'a>(
        &'a self,
        _view: PolicyView<'a>,
    ) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>> {
        Box::pin(async { Action::Done })
    }
}

pub fn primitive(purpose: &str) -> Accelerator {
    Accelerator::primitive(
        RunState {
            purpose: Purpose::new(purpose),
            ..RunState::default()
        },
        Box::new(DonePolicy),
        ToolRuntime::new(),
        purpose,
    )
}
