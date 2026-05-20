use std::sync::atomic::{AtomicU32, Ordering};
use tokio::time::{Duration, sleep};

pub const DEFAULT_BACKOFF_INITIAL: Duration = Duration::from_secs(1);
pub const DEFAULT_BACKOFF_MAX: Duration = Duration::from_secs(60);
pub const DEFAULT_BACKOFF_MULTIPLIER: u64 = 2;
pub const DEFAULT_MAX_RETRIES: u32 = 3;

pub const HTTP_UNAUTHORIZED: u16 = 401;
pub const HTTP_FORBIDDEN: u16 = 403;
pub const HTTP_RATE_LIMITED: u16 = 429;
pub const HTTP_SERVER_ERROR: u16 = 500;
pub const HTTP_BAD_GATEWAY: u16 = 502;
pub const HTTP_SERVICE_UNAVAILABLE: u16 = 503;
pub const HTTP_GATEWAY_TIMEOUT: u16 = 504;

/// Retry budget with exponential backoff.
pub struct Retry {
    attempts: AtomicU32,
    backoff_initial: Duration,
    backoff_max: Duration,
    backoff_multiplier: u64,
    max_retries: u32,
}

impl Retry {
    pub fn new(
        backoff_initial: Duration,
        backoff_max: Duration,
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

    /// Returns `false` when the retry budget is exhausted.
    pub async fn backoff(&self) -> bool {
        let attempt = self.attempts.fetch_add(1, Ordering::Relaxed);
        if attempt >= self.max_retries {
            return false;
        }

        let multiplier = self.backoff_multiplier.saturating_pow(attempt) as u32;
        let delay = self
            .backoff_initial
            .checked_mul(multiplier)
            .unwrap_or(self.backoff_max)
            .min(self.backoff_max);

        sleep(delay).await;
        true
    }

    pub fn reset(&self) {
        self.attempts.store(0, Ordering::Relaxed);
    }

    pub fn count(&self) -> u32 {
        self.attempts.load(Ordering::Relaxed)
    }
}

impl Default for Retry {
    fn default() -> Self {
        Self::new(
            DEFAULT_BACKOFF_INITIAL,
            DEFAULT_BACKOFF_MAX,
            DEFAULT_BACKOFF_MULTIPLIER,
            DEFAULT_MAX_RETRIES,
        )
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
