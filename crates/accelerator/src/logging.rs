use std::path::PathBuf;

use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub struct LogGuard {
    _file_guard: Option<tracing_appender::non_blocking::WorkerGuard>,
}

pub fn init() -> LogGuard {
    let dir = log_dir();
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"))
        .add_directive("h2=warn".parse().unwrap())
        .add_directive("hyper=warn".parse().unwrap())
        .add_directive("reqwest=warn".parse().unwrap())
        .add_directive("rustls=warn".parse().unwrap());

    if let Some(log_dir) = dir {
        std::fs::create_dir_all(&log_dir).ok();
        let prefix = format!("accelerator-{}", std::process::id());
        let appender = tracing_appender::rolling::daily(&log_dir, prefix);
        let (file_writer, guard) = tracing_appender::non_blocking(appender);

        tracing_subscriber::registry()
            .with(
                tracing_subscriber::fmt::layer()
                    .compact()
                    .with_target(false),
            )
            .with(
                tracing_subscriber::fmt::layer()
                    .json()
                    .with_writer(file_writer)
                    .with_target(false),
            )
            .with(filter)
            .try_init()
            .ok();

        LogGuard {
            _file_guard: Some(guard),
        }
    } else {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::fmt::layer()
                    .compact()
                    .with_target(false),
            )
            .with(filter)
            .try_init()
            .ok();

        LogGuard { _file_guard: None }
    }
}

fn log_dir() -> Option<PathBuf> {
    let raw = std::env::var("ACCELERATOR_LOG_DIR").unwrap_or_default();
    if raw.eq_ignore_ascii_case("none") {
        return None;
    }
    if !raw.is_empty() {
        return Some(raw.into());
    }
    let home = std::env::var("HOME").ok()?;
    Some(PathBuf::from(home).join(".accelerator").join("logs"))
}
