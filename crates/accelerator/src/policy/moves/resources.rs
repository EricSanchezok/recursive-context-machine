use machine::{Action, Resources};

use super::super::Step;

pub(crate) fn activate(resources: &Resources) -> Step {
    if resources.active_model.is_empty()
        && let Some(model_name) = resources.model_order.first()
    {
        return Step::Emit(Action::Model(model_name.clone()));
    }

    if let Some(tool_name) = resources
        .tools
        .keys()
        .find(|tool_name| !resources.active_tools.contains(*tool_name))
    {
        return Step::Emit(Action::Activate(tool_name.clone()));
    }

    Step::Ready
}
