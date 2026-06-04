use std::path::Path;

use machine::{Action, ActionOutcome, Context, Environment, Inbox, MachineEvent, Resources};
use serde::{Deserialize, Serialize};

use crate::{Wal, WalError, WalResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineState {
    pub context: Context,
    pub environment: Environment,
    pub resources: Resources,
    pub inbox: Inbox,
    pub step: u64,
    pub done: bool,
}

impl Default for MachineState {
    fn default() -> Self {
        Self {
            context: Context::new(),
            environment: Environment::new("."),
            resources: Resources::new(),
            inbox: Inbox::new(),
            step: 0,
            done: false,
        }
    }
}

pub struct Store {
    wal: Wal,
}

impl Store {
    pub fn open(dir: impl AsRef<Path>) -> WalResult<Self> {
        Ok(Self {
            wal: Wal::open(dir.as_ref())?,
        })
    }

    pub fn record(&mut self, event: &MachineEvent) -> WalResult<u64> {
        let payload = encode(event)?;
        let (offset, _) = self.wal.append(&payload)?;
        Ok(offset)
    }

    pub fn checkpoint(&mut self, state: &MachineState) -> WalResult<()> {
        let payload = encode(state)?;
        self.wal.checkpoint(self.wal.next_offset(), &payload)
    }

    pub fn restore(&self) -> WalResult<Option<MachineState>> {
        let (start_offset, mut state, had_checkpoint) = match self.wal.load()? {
            Some((offset, payload)) => (offset, decode::<MachineState>(&payload)?, true),
            None => (0, MachineState::default(), false),
        };

        let mut applied_event = false;
        for item in self.wal.replay(start_offset)? {
            let (offset, payload) = item?;
            let event = decode::<MachineEvent>(&payload)?;
            apply_event(&mut state, &event)
                .map_err(|detail| WalError::ReplayFailed { offset, detail })?;
            applied_event = true;
        }

        if had_checkpoint || applied_event {
            Ok(Some(state))
        } else {
            Ok(None)
        }
    }

    pub fn next_offset(&self) -> u64 {
        self.wal.next_offset()
    }
}

fn encode(value: &impl Serialize) -> WalResult<Vec<u8>> {
    serde_json::to_vec(value).map_err(|error| WalError::Codec {
        detail: error.to_string(),
    })
}

fn decode<Value: for<'de> Deserialize<'de>>(payload: &[u8]) -> WalResult<Value> {
    serde_json::from_slice(payload).map_err(|error| WalError::Codec {
        detail: error.to_string(),
    })
}

fn apply_event(state: &mut MachineState, event: &MachineEvent) -> Result<(), String> {
    state.step = event.step;

    match &event.action {
        Action::Append(fragment) => {
            state.context.append(fragment.clone());
        }
        Action::Insert { after, fragment } => {
            if let Err(error) = state.context.insert(*after, fragment.clone()) {
                recover_state_outcome(state, &event.outcome, error.to_string())?;
                return Ok(());
            }
        }
        Action::Replace { id, fragment } => {
            if let Err(error) = state.context.replace(*id, fragment.clone()) {
                recover_state_outcome(state, &event.outcome, error.to_string())?;
                return Ok(());
            }
        }
        Action::Remove(id) => {
            if let Err(error) = state.context.remove(*id) {
                recover_state_outcome(state, &event.outcome, error.to_string())?;
                return Ok(());
            }
        }
        Action::Swap(first_id, second_id) => {
            if let Err(error) = state.context.swap(*first_id, *second_id) {
                recover_state_outcome(state, &event.outcome, error.to_string())?;
                return Ok(());
            }
        }
        Action::Model(name) => {
            if let Err(error) = state.resources.use_model(name) {
                recover_state_outcome(state, &event.outcome, error.to_string())?;
                return Ok(());
            }
        }
        Action::Activate(name) => {
            if let Err(error) = state.resources.enable(name) {
                recover_state_outcome(state, &event.outcome, error.to_string())?;
                return Ok(());
            }
        }

        Action::Deactivate(name) => {
            state.resources.disable(name);
        }
        Action::Take => {
            if let Some(fragment) = state.inbox.pop() {
                state.context.append(fragment);
            }
        }
        Action::Halt => match &event.outcome {
            ActionOutcome::Reactor { fragments, .. } => {
                state.inbox.extend(fragments.iter().cloned());
            }
            ActionOutcome::State { .. } => {
                return Err("halt event is missing reactor output".to_string());
            }
        },
        Action::Done => {
            state.done = true;
        }
    }

    if let ActionOutcome::State { inbox } = &event.outcome {
        state.inbox.extend(inbox.iter().cloned());
    }

    Ok(())
}

fn recover_state_outcome(
    state: &mut MachineState,
    outcome: &ActionOutcome,
    fallback_error: String,
) -> Result<(), String> {
    match outcome {
        ActionOutcome::State { inbox } if !inbox.is_empty() => {
            state.inbox.extend(inbox.iter().cloned());
            Ok(())
        }
        _ => Err(fallback_error),
    }
}
