use std::sync::atomic::{AtomicU32, Ordering};
use tokio::time::{Duration, sleep};

/// Default initial backoff duration (seconds).
pub const DEFAULT_BACKOFF_INITIAL: u64 = 1;
/// Default maximum backoff duration (seconds).
pub const DEFAULT_BACKOFF_MAX: u64 = 60;
/// Default backoff multiplier (doubles each attempt).
pub const DEFAULT_BACKOFF_MULTIPLIER: u64 = 2;
/// Default maximum retry attempts.
pub const DEFAULT_MAX_RETRIES: u32 = 3;

/// Common HTTP status codes used in hitch classification.
pub const HTTP_UNAUTHORIZED: u16 = 401;
pub const HTTP_FORBIDDEN: u16 = 403;
pub const HTTP_RATE_LIMITED: u16 = 429;
pub const HTTP_SERVER_ERROR: u16 = 500;
pub const HTTP_BAD_GATEWAY: u16 = 502;
pub const HTTP_SERVICE_UNAVAILABLE: u16 = 503;
pub const HTTP_GATEWAY_TIMEOUT: u16 = 504;

/// Whether a hitch with this HTTP code is categorically not worth retrying.
/// Auth failures (401/403) will not succeed on retry; transient codes (429,
/// 5xx) might.
pub fn is_permanent(code: u16) -> bool {
    matches!(code, HTTP_UNAUTHORIZED | HTTP_FORBIDDEN)
}

/// Human-readable label for common HTTP codes.
pub fn http_status_label(code: u16) -> &'static str {
    match code {
        HTTP_UNAUTHORIZED => "unauthorized",
        HTTP_FORBIDDEN => "forbidden",
        HTTP_RATE_LIMITED => "rate limited",
        HTTP_SERVER_ERROR => "server error",
        HTTP_BAD_GATEWAY => "bad gateway",
        HTTP_SERVICE_UNAVAILABLE => "unavailable",
        HTTP_GATEWAY_TIMEOUT => "gateway timeout",
        _ => "unknown",
    }
}

/// Retry budget with exponential backoff.
///
/// Embed in any Policy that needs retry-tracking. Call [`bump`] on each hitch
/// turn, [`reset`] on any non-hitch turn. The retry decides whether to proceed
/// and sleeps for the backoff duration via [`backoff`].
///
/// Clone produces a fresh counter (a cloned Policy is a separate logical run).
pub struct Retry {
    attempts: AtomicU32,
    backoff_initial: u64,
    backoff_max: u64,
    backoff_multiplier: u64,
    max_retries: u32,
}

impl Retry {
    pub fn new(
        backoff_initial: u64,
        backoff_max: u64,
        backoff_multiplier: u64,
        max_retries: u32,
    ) -> Self {
        Self {
            attempts: AtomicU32::new(0),
            backoff_initial,
            backoff_max,
            backoff_multiplier,
            max_retries,
        }
    }

    /// Create with default parameters.
    pub fn default() -> Self {
        Self::new(
            DEFAULT_BACKOFF_INITIAL,
            DEFAULT_BACKOFF_MAX,
            DEFAULT_BACKOFF_MULTIPLIER,
            DEFAULT_MAX_RETRIES,
        )
    }

    /// Atomically increment and return the new consecutive count.
    pub fn bump(&self) -> u32 {
        self.attempts.fetch_add(1, Ordering::Relaxed) + 1
    }

    /// Reset the counter and backoff state to zero.
    pub fn reset(&self) {
        self.attempts.store(0, Ordering::Relaxed);
    }

    /// Current consecutive attempt count.
    pub fn count(&self) -> u32 {
        self.attempts.load(Ordering::Relaxed)
    }

    /// Returns `true` if retry should proceed (within max_retries).
    /// Also sleeps for the exponentially increasing backoff duration.
    pub async fn backoff(&self) -> bool {
        let attempt = self.count();
        if attempt > self.max_retries {
            return false;
        }

        let delay_secs = self
            .backoff_initial
            .saturating_mul(self.backoff_multiplier.saturating_pow(attempt - 1))
            .min(self.backoff_max);

        sleep(Duration::from_secs(delay_secs)).await;
        true
    }
}

impl Clone for Retry {
    fn clone(&self) -> Self {
        Self {
            attempts: AtomicU32::new(0),
            backoff_initial: self.backoff_initial,
            backoff_max: self.backoff_max,
            backoff_multiplier: self.backoff_multiplier,
            max_retries: self.max_retries,
        }
    }
}
