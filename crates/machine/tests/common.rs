#![allow(dead_code)]

use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;

use machine::{Action, Context, Environment, Inbox, Model, Policy, Reactor, Resources, Tool};

/// A policy that replays a fixed sequence of actions.
pub struct SeqPolicy {
    actions: Vec<Action>,
    pos: Mutex<usize>,
}

impl SeqPolicy {
    pub fn new(actions: Vec<Action>) -> Self {
        Self {
            actions,
            pos: Mutex::new(0),
        }
    }
}

impl Policy for SeqPolicy {
    fn decide<'a>(
        &'a self,
        _ctx: &'a Context,
        _env: &'a Environment,
        _resources: &'a Resources,
    ) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>> {
        let mut pos = self.pos.lock().unwrap();
        let action = self.actions[*pos].clone();
        *pos += 1;
        Box::pin(async move { action })
    }
}

/// A reactor that returns a fixed sequence of inboxes.
pub struct SeqReactor {
    responses: Vec<Inbox>,
    pos: Mutex<usize>,
}

impl SeqReactor {
    pub fn new(responses: Vec<Inbox>) -> Self {
        Self {
            responses,
            pos: Mutex::new(0),
        }
    }
}

impl Reactor for SeqReactor {
    fn react<'a>(
        &'a self,
        _ctx: &'a Context,
        _env: &'a Environment,
        _tools: &'a [&'a dyn Tool],
        _model: Option<&'a Model>,
    ) -> Pin<Box<dyn Future<Output = Inbox> + Send + 'a>> {
        let mut pos = self.pos.lock().unwrap();
        let response = if *pos >= self.responses.len() {
            Inbox::new()
        } else {
            self.responses[*pos].clone()
        };
        *pos += 1;
        Box::pin(async move { response })
    }
}

/// Build a test model.
pub fn test_model() -> Model {
    Model {
        name: "test".into(),
        provider: "test".into(),
        ..Default::default()
    }
}

/// Build test resources with a single model.
pub fn test_resources() -> Resources {
    Resources::new().with_model(test_model())
}
