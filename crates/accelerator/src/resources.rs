use machine::Resources;

use crate::model::nex_n1;
use crate::tools::builtin_tools;

/// Build the default resource kit — built-in tools, prompts, and model.
///
/// The kit includes:
///   - all built-in tools (activated by default)
///   - the default system prompt
///   - the Nex N1 model (active by default)
///
/// # Example
///
/// ```no_run
/// use accelerator::kit;
///
/// let resources = kit();
/// ```
pub fn kit() -> Resources {
    let mut resources = Resources::new();

    // Register and activate all built-in tools.
    for t in builtin_tools() {
        let name = t.name().to_string();
        resources = resources.with_tool(t);
        resources.catch_tool(name);
    }

    // Load default system prompt.
    resources.prompts.insert(
        "default".to_string(),
        include_str!("prompts/default.txt").to_string(),
    );

    // Register and activate the default model.
    resources = resources.with_model(nex_n1());
    resources.set_active_model("nex-agi/nex-n1");

    resources
}
