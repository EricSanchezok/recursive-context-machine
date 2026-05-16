/// Emit an application-level hook event.
///
/// Hook events are dispatched through the `tracing` subscriber at `TRACE` level
/// with target `"hook"`. Consumers filter by target to observe application events
/// independently of logging.
///
/// # Example
///
/// ```ignore
/// hook!(event = "appended", id, role = ?frag.role, preview = %preview(frag));
/// ```
#[macro_export]
macro_rules! hook {
    ($($tokens:tt)*) => {
        tracing::event!(
            target: "hook",
            tracing::Level::TRACE,
            $($tokens)*
        );
    };
}
