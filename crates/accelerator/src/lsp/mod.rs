//! LSP integration for code intelligence and post-edit diagnostics.

mod client;
mod diagnostics;
mod pool;
mod servers;
mod transport;
mod uri;

pub use diagnostics::format_file_diagnostics;
pub use pool::touch_file;
