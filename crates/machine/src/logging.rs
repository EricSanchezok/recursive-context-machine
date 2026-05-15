use std::path::PathBuf;

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

/// Holds the file writer guard. Dropping it flushes buffered logs.
pub struct LogGuard {
    _guard: Option<WorkerGuard>,
}

/// Initialize structured logging.
///
/// Reads `RUST_LOG` for level and module filtering (defaults to `info`).
///
/// Logs go to:
///   - stdout: compact human‑readable
///   - `~/.accelerator/logs/`: JSON files rotated daily
///
/// Set `RUST_LOG_DIR=none` to disable file logs.
/// Double-initialization is silently ignored.
pub fn init() -> LogGuard {
    let log_dir = log_dir();
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"))
        .add_directive("h2=warn".parse().unwrap())
        .add_directive("hyper=warn".parse().unwrap())
        .add_directive("reqwest=warn".parse().unwrap())
        .add_directive("rustls=warn".parse().unwrap());

    if let Some(dir) = log_dir {
        std::fs::create_dir_all(&dir).ok();
        let file_appender = tracing_appender::rolling::daily(&dir, "accelerator");
        let (writer, g) = tracing_appender::non_blocking(file_appender);

        tracing_subscriber::registry()
            .with(filter)
            .with(
                tracing_subscriber::fmt::layer()
                    .compact()
                    .with_target(false)
                    .boxed(),
            )
            .with(
                tracing_subscriber::fmt::layer()
                    .json()
                    .with_writer(writer)
                    .with_target(false)
                    .boxed(),
            )
            .try_init()
            .ok();
        LogGuard { _guard: Some(g) }
    } else {
        tracing_subscriber::registry()
            .with(filter)
            .with(
                tracing_subscriber::fmt::layer()
                    .compact()
                    .with_target(false)
                    .boxed(),
            )
            .try_init()
            .ok();
        LogGuard { _guard: None }
    }
}

/// Determine log directory.
///
/// Order of precedence:
///   1. `ACCELERATOR_LOG_DIR` environment variable
///   2. `$HOME/.accelerator/logs`
///   3. `none` → disabled
fn log_dir() -> Option<PathBuf> {
    let raw = std::env::var("ACCELERATOR_LOG_DIR").unwrap_or_default();
    if raw.eq_ignore_ascii_case("none") {
        return None;
    }
    if !raw.is_empty() {
        return Some(PathBuf::from(raw));
    }
    let home = std::env::var("HOME").ok()?;
    Some(PathBuf::from(home).join(".accelerator").join("logs"))
}
