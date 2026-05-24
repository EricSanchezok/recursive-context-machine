use machine::{Context, Environment, Inbox, Resources};
use std::collections::HashMap;
use uuid::Uuid;

use machine::Machine;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MachineId(String);

impl MachineId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for MachineId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<MachineId> for String {
    fn from(id: MachineId) -> Self {
        id.0
    }
}

pub struct Run {
    pub machine: Machine,
    pub ctx: Context,
    pub env: Environment,
    pub resources: Resources,
    pub inbox: Inbox,
    pub step: u64,
    pub done: bool,
}

pub struct MachineManager {
    machines: HashMap<MachineId, Run>,
}

impl MachineManager {
    pub fn new() -> Self {
        Self {
            machines: HashMap::new(),
        }
    }

    pub fn create(&mut self, state: Run) -> MachineId {
        let id = MachineId::new();
        self.machines.insert(id.clone(), state);
        id
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
