use std::path::Path;

use machine::{Machine, MachineState, StoredEvent};
use serde::{Deserialize, Serialize};

use crate::{Wal, WalError, WalResult};

/// One recorded decision point: the observation the policy saw, the action
/// it chose, and the effects the machine applied. This is the training-data
/// unit for offline policy learning and the audit trail for replay.
///
/// Overlay declarations are persisted as counts inside `obs.overlay_status`
/// only — projected content is ephemeral and re-derived by the policy on
/// every turn, so it never enters the audit trail.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrajectoryEvent {
    pub step: u64,
    pub obs: machine::Obs,
    /// Ledger state migrations caused by this step's tool calls, lifted
    /// from ledger tool results by `machine::ledger_transitions_in`.
    pub ledger_transitions: Vec<machine::LedgerTransition>,
    pub event: StoredEvent,
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

    pub fn record_trajectory(&mut self, trajectory: &TrajectoryEvent) -> WalResult<u64> {
        let payload = encode(trajectory)?;
        let (offset, _) = self.wal.append(&payload)?;
        Ok(offset)
    }

    pub fn checkpoint(&mut self, state: &MachineState) -> WalResult<()> {
        let payload = encode(state)?;
        // Trajectory stores retain their events: the WAL is the payload,
        // not a disposable prefix of it.
        self.wal
            .checkpoint_retaining_events(self.wal.next_offset(), &payload)
    }

    pub async fn restore(&self) -> WalResult<Option<MachineState>> {
        let (start_offset, mut state, had_checkpoint) = match self.wal.load()? {
            Some((offset, payload)) => (offset, decode::<MachineState>(&payload)?, true),
            None => (0, MachineState::default(), false),
        };

        let machine = Machine::new("recovered", "recovered");
        let mut applied_event = false;
        for item in self.wal.replay(start_offset)? {
            let (_, payload) = item?;
            let trajectory = decode::<TrajectoryEvent>(&payload)?;
            state.frame.step = trajectory.event.step;
            machine.replay_effects(&mut state, &trajectory.event.effects);
            applied_event = true;
        }

        if had_checkpoint || applied_event {
            Ok(Some(state))
        } else {
            Ok(None)
        }
    }

    /// Read back the full recorded trajectory (envelopes, not bare events).
    /// Used by tests and offline training pipelines to consume
    /// `(obs, action, effects)` tuples exactly as the policy experienced them.
    pub async fn trajectories(&self) -> WalResult<Vec<TrajectoryEvent>> {
        let mut trajectories = Vec::new();
        // From origin, not from the last checkpoint: trajectory stores
        // retain all events, and the training pipeline wants every
        // (obs, action, effects) pair ever recorded.
        for item in self.wal.replay_from_origin()? {
            let (_, payload) = item?;
            trajectories.push(decode::<TrajectoryEvent>(&payload)?);
        }
        Ok(trajectories)
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
