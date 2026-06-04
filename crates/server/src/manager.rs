use machine::{Context, Environment, Inbox, Machine, Resources, ToolRuntime};
use std::collections::HashMap;
use utils::MachineId;

pub struct Run {
    pub purpose: String,
    pub machine: Machine,
    pub ctx: Context,
    pub env: Environment,
    pub resources: Resources,
    pub tool_runtime: ToolRuntime,
    pub inbox: Inbox,
    pub step: u64,
    pub done: bool,
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
