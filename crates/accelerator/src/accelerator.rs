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
    input_hint: InputHint,
}

#[derive(Clone, Default)]
struct InputHint {
    purpose: Option<String>,
}

#[derive(Clone, Default)]
pub struct InputState {
    pub purpose: Option<String>,
}

impl Accelerator {
    pub fn primitive(config: AcceleratorConfig, name: impl Into<String>) -> Self {
        Self {
            id: AcceleratorId::new(),
            name: Name::new(name).expect("accelerator name must be valid"),
            body: AcceleratorBody::Primitive(PrimitiveAccelerator { config }),
            input_hint: InputHint::default(),
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
            input_hint: InputHint::default(),
        }
    }

    pub fn id(&self) -> &AcceleratorId {
        &self.id
    }

    pub fn with_input(mut self, input: InputState) -> Self {
        self.input_hint.purpose = input.purpose.or(self.input_hint.purpose);
        self
    }

    pub fn run_with(self, input: State) -> Pin<Box<dyn Future<Output = State> + Send>> {
        Box::pin(async move {
            let input = self.merge_input(input);
            match self.body {
                AcceleratorBody::Primitive(primitive) => primitive.fire(input).await,
                AcceleratorBody::Composite(graph) => graph.run(input).await,
            }
        })
    }

    pub fn state(&self) -> Option<&State> {
        match &self.body {
            AcceleratorBody::Primitive(p) => Some(&p.config.base),
            AcceleratorBody::Composite(_) => None,
        }
    }

    fn merge_input(&self, input: State) -> State {
        let mut state = input;
        if let AcceleratorBody::Primitive(primitive) = &self.body {
            let base = &primitive.config.base;
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
        if let Some(purpose) = &self.input_hint.purpose {
            state.purpose.clone_from(purpose);
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
pub struct AcceleratorConfig {
    pub base: State,
}

impl Default for AcceleratorConfig {
    fn default() -> Self {
        Self {
            base: State::default(),
        }
    }
}

#[derive(Clone)]
struct PrimitiveAccelerator {
    config: AcceleratorConfig,
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
