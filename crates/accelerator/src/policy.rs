use std::future::Future;
use std::pin::Pin;

use machine::{Action, Context, Environment, Inbox, Policy, Resources};

/// Default policy — simple strategy.
///
/// 1. Set the first available model.
/// 2. Catch the first available tool.
/// 3. Take all fragments from the inbox.
/// 4. Halt to trigger the Reactor.
/// 5. Take all new fragments from the inbox.
/// 6. Done.
pub struct DefaultPolicy {
    phase: std::sync::Mutex<Phase>,
}

#[derive(PartialEq)]
enum Phase {
    Model,
    Catch,
    TakeInbox,
    Halt,
    TakeResult,
    Done,
}

impl Default for DefaultPolicy {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultPolicy {
    pub fn new() -> Self {
        Self {
            phase: std::sync::Mutex::new(Phase::Model),
        }
    }
}

impl Policy for DefaultPolicy {
    fn decide<'a>(
        &'a self,
        _ctx: &'a Context,
        _env: &'a Environment,
        resources: &'a Resources,
        inbox: &'a Inbox,
    ) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>> {
        let mut phase = self.phase.lock().unwrap();

        let action = loop {
            match *phase {
                Phase::Model => {
                    *phase = Phase::Catch;
                    if !resources.models.is_empty() {
                        break Action::Model(resources.models[0].name.clone());
                    }
                }
                Phase::Catch => {
                    *phase = Phase::TakeInbox;
                    if !resources.tools.is_empty() {
                        break Action::Catch(resources.tools[0].name().to_string());
                    }
                }
                Phase::TakeInbox => {
                    if !inbox.is_empty() {
                        break Action::Take;
                    }
                    *phase = Phase::Halt;
                }
                Phase::Halt => {
                    *phase = Phase::TakeResult;
                    break Action::Halt;
                }
                Phase::TakeResult => {
                    if !inbox.is_empty() {
                        break Action::Take;
                    }
                    *phase = Phase::Done;
                }
                Phase::Done => break Action::Done,
            }
        };

        Box::pin(async move { action })
    }
}
