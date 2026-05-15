use tracing_subscriber::EnvFilter;

/// Initialize structured logging for the machine runtime.
///
/// Reads `RUST_LOG` for level and module filtering (defaults to `info`).
/// Double-initialization is silently ignored so tests can call this freely.
pub fn init() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"))
        .add_directive("h2=warn".parse().unwrap())
        .add_directive("hyper=warn".parse().unwrap())
        .add_directive("reqwest=warn".parse().unwrap())
        .add_directive("rustls=warn".parse().unwrap());

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .compact()
        .try_init()
        .ok();
}
