//! LSP integration for code intelligence and post-edit diagnostics.

mod client;
mod diagnostics;
mod pool;
mod servers;
mod transport;
mod uri;

pub use diagnostics::{DiagnosticSnapshot, format_file_diagnostics, new_error_diagnostics};
pub use pool::{snapshot, touch_file_from_disk, touch_file_with_text};
