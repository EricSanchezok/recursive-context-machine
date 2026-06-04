use std::path::Path;

use machine::{ApplyMode, Machine, MachineEvent, MachineState};
use serde::{Deserialize, Serialize};

use crate::{Wal, WalError, WalResult};

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

    pub async fn restore(&self) -> WalResult<Option<MachineState>> {
        let (start_offset, mut state, had_checkpoint) = match self.wal.load()? {
            Some((offset, payload)) => (offset, decode::<MachineState>(&payload)?, true),
            None => (0, MachineState::default(), false),
        };

        let mut applied_event = false;
        let mut machine = Machine::new("recovered", "recovered");
        for item in self.wal.replay(start_offset)? {
            let (offset, payload) = item?;
            let event = decode::<MachineEvent>(&payload)?;

            let action = event.action.clone();
            let replay_outcome = event.outcome.clone();

            let result = machine
                .apply_state(
                    action,
                    &mut state,
                    ApplyMode::Replay {
                        cached_outcome: replay_outcome,
                    },
                )
                .await;
            if result.done != event.action.is_done() {
                return Err(WalError::ReplayFailed {
                    offset,
                    detail: format!(
                        "replay done mismatch: apply returned {}, event has {}",
                        result.done,
                        event.action.is_done()
                    ),
                });
            }
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
