use std::future::Future;
use std::pin::Pin;

use crate::context::Context;
use crate::env::Environment;
use crate::fragment::Fragment;
use crate::inbox::Inbox;
use crate::purpose::Purpose;
use crate::resources::Resources;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Append(Fragment),
    Insert { after: u64, fragment: Fragment },
    Replace { id: u64, fragment: Fragment },
    Remove(u64),
    Swap(u64, u64),
    Model(String),
    Activate(String),
    Deactivate(String),
    Take,
    Halt,
    Done,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PhaseOutcome {
    Action(Action),
    Done,
}

pub trait Phase: Send + Sync {
    fn clone_box(&self) -> Box<dyn Phase>;
    fn name(&self) -> &str;

    fn decide(
        &self,
        purpose: &Purpose,
        ctx: &Context,
        env: &Environment,
        resources: &Resources,
    ) -> PhaseOutcome;
}

impl Clone for Box<dyn Phase> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub trait Policy: Send + Sync {
    fn clone_box(&self) -> Box<dyn Policy>;

    fn pre(&self) -> Vec<Box<dyn Phase>> {
        Vec::new()
    }

    fn post(&self) -> Vec<Box<dyn Phase>> {
        Vec::new()
    }

    fn pre_halt(&self) -> Vec<Box<dyn Phase>> {
        Vec::new()
    }

    fn post_halt(&self) -> Vec<Box<dyn Phase>> {
        Vec::new()
    }

    fn decide<'a>(
        &'a self,
        purpose: &'a Purpose,
        ctx: &'a Context,
        env: &'a Environment,
        resources: &'a Resources,
        inbox: &'a Inbox,
    ) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>>;
}

impl Clone for Box<dyn Policy> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
