#![allow(dead_code)]

use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};

use machine::{Action, Context, Environment, Inbox, Model, Policy, Reactor, Resources, Tool};

/// A policy that replays a fixed sequence of actions.
///
/// When the sequence is exhausted, it returns [`Action::Done`].
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

/// A reactor that returns a fixed sequence of inboxes.
pub struct SeqReactor {
    responses: Vec<Inbox>,
    pos: AtomicUsize,
}

impl SeqReactor {
    pub fn new(responses: Vec<Inbox>) -> Self {
        Self {
            responses,
            pos: AtomicUsize::new(0),
        }
    }
}

impl Reactor for SeqReactor {
    fn react<'a>(
        &'a self,
        _ctx: &'a Context,
        _env: &'a Environment,
        _resources: &'a Resources,
        inbox: &'a mut Inbox,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        let pos = self.pos.fetch_add(1, Ordering::SeqCst);
        if pos < self.responses.len() {
            for frag in self.responses[pos].clone() {
                inbox.push(frag);
            }
        }
        Box::pin(async move {})
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

/// Build test resources with a model and tools.
pub fn test_resources_with_tools() -> Resources {
    Resources::new()
        .with_model(test_model())
        .with_tool(Box::new(TestTool("tool-a")))
        .with_tool(Box::new(TestTool("tool-b")))
}

struct TestTool(&'static str);

impl Tool for TestTool {
    fn name(&self) -> &str {
        self.0
    }
    fn description(&self) -> &str {
        "test tool"
    }
    fn parameters(&self) -> serde_json::Value {
        serde_json::json!({})
    }
    fn execute<'a>(
        &'a self,
        _args: serde_json::Value,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<machine::ToolOutput, String>> + Send + 'a>,
    > {
        Box::pin(async move {
            Ok(machine::ToolOutput {
                content: "ok".into(),
                title: None,
            })
        })
    }
}
