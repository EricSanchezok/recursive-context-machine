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
    purpose_override: Option<String>,
}

impl Accelerator {
    pub fn primitive(state: State, name: impl Into<String>) -> Self {
        Self {
            id: AcceleratorId::new(),
            name: Name::new(name).expect("accelerator name must be valid"),
            body: AcceleratorBody::Primitive(PrimitiveAccelerator { state }),
            purpose_override: None,
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
            purpose_override: None,
        }
    }

    pub fn id(&self) -> &AcceleratorId {
        &self.id
    }

    pub fn set_purpose_override(&mut self, purpose: String) {
        self.purpose_override = Some(purpose);
    }

    pub fn run_with(mut self, input: State) -> Pin<Box<dyn Future<Output = State> + Send>> {
        let purpose_override = self.purpose_override.take();
        Box::pin(async move {
            let input = self.merge_input(input, purpose_override);
            match self.body {
                AcceleratorBody::Primitive(primitive) => primitive.fire(input).await,
                AcceleratorBody::Composite(graph) => graph.run(input).await,
            }
        })
    }

    pub fn internal_state(&self) -> Option<&State> {
        match &self.body {
            AcceleratorBody::Primitive(primitive) => Some(&primitive.state),
            AcceleratorBody::Composite(_) => None,
        }
    }

    fn merge_input(&self, input: State, purpose_override: Option<String>) -> State {
        let mut state = input;
        if let AcceleratorBody::Primitive(primitive) = &self.body {
            let base = &primitive.state;
            if state.purpose.is_empty() {
                state.purpose.clone_from(&base.purpose);
            }
            state.policy = base.policy.clone_box();
            if state.ctx.is_empty() {
                state.ctx = base.ctx.clone();
            }
            if state.env.cwd.as_os_str().is_empty() {
                state.env = base.env.clone();
            }
            state.res.models.clone_from(&base.res.models);
            state.res.model_order.clone_from(&base.res.model_order);
            state.res.tools.clone_from(&base.res.tools);
            state.res.prompts.clone_from(&base.res.prompts);
            state.res.active_model.clone_from(&base.res.active_model);
            state.res.active_tools.clone_from(&base.res.active_tools);
        }
        if let Some(purpose) = purpose_override {
            state.purpose = purpose;
        }
        state
    }
}

#[derive(Clone)]
enum AcceleratorBody {
    Primitive(PrimitiveAccelerator),
    Composite(Graph),
}

#[derive(Clone)]
struct PrimitiveAccelerator {
    state: State,
}

impl PrimitiveAccelerator {
    async fn fire(self, mut state: State) -> State {
        let purpose = Purpose::new(&state.purpose);
        let machine = Machine::new(state.policy.clone());
        machine
            .run(&purpose, &mut state.ctx, &mut state.env, &mut state.res)
            .await;
        state
    }
}
