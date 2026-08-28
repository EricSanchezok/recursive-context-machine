//! Opt-in trajectory recording for primitive accelerators.
//!
//! A recorder is created only when the caller supplied a run directory;
//! without one, `fire()` behaves exactly as before. Graph components run
//! concurrently, so each recorder owns a per-machine subdirectory under
//! `<run_dir>/trajectory/` — never a shared WAL.

use std::path::{Path, PathBuf};

use machine::obs::Obs;
use machine::{LedgerTransition, MachineState, StoredEvent};
use storage::{Store, TrajectoryEvent};
use tracing::warn;

pub struct TrajectoryRecorder {
    store: Store,
    dir: PathBuf,
}

impl TrajectoryRecorder {
    /// Open (creating if needed) the trajectory directory for one machine.
    /// The machine label keeps concurrent graph components in separate WALs.
    pub fn open(run_dir: &Path, machine_label: &str) -> storage::WalResult<Self> {
        let dir = run_dir.join("trajectory").join(machine_label);
        let store = Store::open(&dir)?;
        Ok(Self { store, dir })
    }

    /// Record one decision point. WAL failure is a side-channel problem:
    /// warn and keep running rather than kill the run. Overlay presence is
    /// captured as counts inside `obs.overlay_status`; projected content is
    /// never persisted.
    pub fn record_step(
        &mut self,
        step: u64,
        obs: &Obs,
        ledger_transitions: &[LedgerTransition],
        event: &StoredEvent,
    ) {
        let trajectory = TrajectoryEvent {
            step,
            obs: obs.clone(),
            ledger_transitions: ledger_transitions.to_vec(),
            event: event.clone(),
        };
        if let Err(error) = self.store.record_trajectory(&trajectory) {
            warn!(
                dir = %self.dir.display(),
                step,
                ?error,
                "trajectory write failed; continuing without it"
            );
        }
    }

    /// Persist the final machine state so the run can be restored later.
    pub fn checkpoint(&mut self, state: &MachineState) {
        if let Err(error) = self.store.checkpoint(state) {
            warn!(
                dir = %self.dir.display(),
                ?error,
                "trajectory checkpoint failed; run output is unaffected"
            );
        }
    }
}
