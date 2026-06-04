use std::path::Path;

use machine::{
    Action, ActionOutcome, Context, Environment, Fragment, Inbox, MachineEvent, Resources,
};
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
            state
                .context
                .insert(*after, fragment.clone())
                .map_err(|error| error.to_string())?;
        }
        Action::Replace { id, fragment } => {
            state
                .context
                .replace(*id, fragment.clone())
                .map_err(|error| error.to_string())?;
        }
        Action::Remove(id) => {
            state
                .context
                .remove(*id)
                .map_err(|error| error.to_string())?;
        }
        Action::Swap(first_id, second_id) => {
            state
                .context
                .swap(*first_id, *second_id)
                .map_err(|error| error.to_string())?;
        }
        Action::Model(name) => {
            state
                .resources
                .use_model(name)
                .map_err(|error| error.to_string())?;
        }
        Action::Activate(name) => {
            state
                .resources
                .enable(name)
                .map_err(|error| error.to_string())?;
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
            ActionOutcome::StateOnly => {
                return Err("halt event is missing reactor output".to_string());
            }
        },
        Action::Done => {
            state.done = true;
        }
    }

    if !matches!(event.action, Action::Halt) && !matches!(event.outcome, ActionOutcome::StateOnly) {
        apply_outcome(state, &event.outcome)?;
    }

    Ok(())
}

fn apply_outcome(state: &mut MachineState, outcome: &ActionOutcome) -> Result<(), String> {
    match outcome {
        ActionOutcome::StateOnly => Ok(()),
        ActionOutcome::Reactor { fragments, .. } => {
            for fragment in fragments {
                state.inbox.push(fragment.clone());
            }
            Ok(())
        }
    }
}

pub fn state_from_parts(
    context: Context,
    environment: Environment,
    resources: Resources,
    inbox: Inbox,
    step: u64,
    done: bool,
) -> MachineState {
    MachineState {
        context,
        environment,
        resources,
        inbox,
        step,
        done,
    }
}

pub fn inbox_from_fragments(fragments: impl IntoIterator<Item = Fragment>) -> Inbox {
    fragments.into_iter().collect()
}
