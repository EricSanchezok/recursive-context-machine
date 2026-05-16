use machine::{Action, Context, Environment, Inbox, Model, Policy, Resources};
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};

/// A policy that replays a fixed sequence of actions, then Done.
#[derive(Clone)]
pub struct SeqPolicy {
    actions: Vec<Action>,
    pos: AtomicUsize,
}

impl SeqPolicy {
    pub fn new(actions: Vec<Action>) -> Self {
        Self {
            actions,
            pos: AtomicUsize::new(0),
        }
    }
}

impl Policy for SeqPolicy {
    fn clone_box(&self) -> Box<dyn Policy> {
        Box::new(self.clone())
    }

    fn decide<'a>(
    fn decide<'a>(
        &'a self,
        _ctx: &'a Context,
        _env: &'a Environment,
        _resources: &'a Resources,
        _inbox: &'a Inbox,
    ) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>> {
        let pos = self.pos.fetch_add(1, Ordering::SeqCst);
        let action = if pos >= self.actions.len() {
            Action::Done
        } else {
            self.actions[pos].clone()
        };
        Box::pin(async move { action })
    }
}

pub fn test_model() -> Model {
    Model {
        name: "test".into(),
        ..Default::default()
    }
}

pub fn test_resources() -> Resources {
    Resources::new().with_model(test_model())
}
