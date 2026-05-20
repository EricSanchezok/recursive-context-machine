use machine::{Machine, Purpose};
use std::future::Future;
use std::pin::Pin;
use utils::{AcceleratorId, Name};

use crate::graph::Graph;
use crate::state::State;

#[derive(Clone)]
pub struct Accelerator {
    id: AcceleratorId,
    pub name: Name,
    body: AcceleratorBody,
}

impl Accelerator {
    pub fn primitive(state: State) -> Self {
        Self::primitive_named("accelerator", state)
    }

    pub fn primitive_named(name: impl Into<String>, state: State) -> Self {
        Self {
            id: AcceleratorId::new(),
            name: Name::new(name).expect("accelerator name must be valid"),
            body: AcceleratorBody::Primitive(PrimitiveAccelerator { state }),
        }
    }

    pub fn composite(graph: Graph) -> Self {
        let name = graph.name.clone();
        Self::composite_named(name.as_str(), graph)
    }

    pub fn composite_named(name: impl Into<String>, graph: Graph) -> Self {
        Self {
            id: AcceleratorId::new(),
            name: Name::new(name).expect("accelerator name must be valid"),
            body: AcceleratorBody::Composite(graph),
        }
    }

    pub fn id(&self) -> &AcceleratorId {
        &self.id
    }

    pub fn body(&self) -> &AcceleratorBody {
        &self.body
    }

    pub fn set_purpose(&mut self, purpose: impl Into<String>) -> Result<(), String> {
        match &mut self.body {
            AcceleratorBody::Primitive(primitive) => {
                primitive.state.purpose = purpose.into();
                Ok(())
            }
            AcceleratorBody::Composite(_) => {
                Err("graph accelerator purpose override is not supported".to_string())
            }
        }
    }

    pub fn run(self) -> Pin<Box<dyn Future<Output = State> + Send>> {
        let input = self.default_input();
        self.run_with(input)
    }

    pub(crate) fn default_input(&self) -> State {
        match &self.body {
            AcceleratorBody::Primitive(primitive) => primitive.state.clone(),
            AcceleratorBody::Composite(_) => State::default(),
        }
    }

    pub(crate) fn run_with(self, input: State) -> Pin<Box<dyn Future<Output = State> + Send>> {
        Box::pin(async move {
            match self.body {
                AcceleratorBody::Primitive(_) => fire(input).await,
                AcceleratorBody::Composite(graph) => graph.run(input).await,
            }
        })
    }
}

#[derive(Clone)]
pub enum AcceleratorBody {
    Primitive(PrimitiveAccelerator),
    Composite(Graph),
}

#[derive(Clone)]
pub struct PrimitiveAccelerator {
    state: State,
}

impl PrimitiveAccelerator {
    pub fn state(&self) -> &State {
        &self.state
    }
}

pub(crate) async fn fire(mut state: State) -> State {
    let purpose = Purpose::new(&state.purpose);
    let machine = Machine::new(state.policy.clone());
    machine
        .run(&purpose, &mut state.ctx, &mut state.env, &mut state.res)
        .await;
    state
}
