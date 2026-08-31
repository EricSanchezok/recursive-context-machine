use std::collections::HashMap;
use std::sync::Arc;

use machine::{Machine, MachineState, ToolRuntime};
use utils::MachineId;

pub struct Run {
    pub machine: Machine,
    pub state: MachineState,
    pub tool_runtime: ToolRuntime,
    /// Metered gateway for generative tools (context.compact). Published
    /// before every step so tools see the step-start document and model.
    pub assistant: Arc<accelerator::assistant::AssistantGateway>,
    /// Per-machine trajectory store. Always present unless opening the WAL
    /// failed — trajectory loss must never prevent a run from existing.
    pub store: Option<storage::Store>,
}

pub struct MachineManager {
    machines: HashMap<MachineId, Run>,
}

impl Default for MachineManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MachineManager {
    pub fn new() -> Self {
        Self {
            machines: HashMap::new(),
        }
    }

    pub fn insert(&mut self, id: MachineId, run: Run) {
        self.machines.insert(id, run);
    }

    pub fn get(&self, id: &MachineId) -> Option<&Run> {
        self.machines.get(id)
    }

    pub fn get_mut(&mut self, id: &MachineId) -> Option<&mut Run> {
        self.machines.get_mut(id)
    }

    pub fn destroy(&mut self, id: &MachineId) {
        self.machines.remove(id);
    }
}
