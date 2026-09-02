use std::future::Future;
use std::pin::Pin;

use accelerator::{Accelerator, Graph};
use machine::{Action, Policy, PolicyView, RunState, ToolRuntime};

#[derive(Clone)]
struct PanicPolicy;

impl Policy for PanicPolicy {
    fn clone_box(&self) -> Box<dyn Policy> {
        Box::new(self.clone())
    }

    fn decide<'a>(
        &'a self,
        _view: PolicyView<'a>,
    ) -> Pin<Box<dyn Future<Output = Action> + Send + 'a>> {
        Box::pin(async { panic!("component policy panic") })
    }
}

#[tokio::test]
#[should_panic(expected = "component policy panic")]
async fn graph_propagates_component_panics() {
    let accelerator = Accelerator::primitive(
        RunState::default(),
        Box::new(PanicPolicy),
        ToolRuntime::new(),
        "panicking_component",
    );
    let mut graph = Graph::named("failure_propagation");
    graph.add_accelerator("panicking_component", accelerator);

    graph.run(RunState::default()).await;
}
