//! Per-IP request-budget rate limiter.
//!
//! Distinct from the PIN-attempt lockout in [`crate::auth::attempts`]: the
//! PIN-attempt limiter throttles *failed* login attempts globally; this
//! one throttles the overall request rate per client IP using a
//! sliding-window counter.
//!
//! The limiter is in-memory and not distributed — suitable for a single
//! backend process. To use it across replicas, swap the storage layer.

use std::collections::HashMap;
use std::net::IpAddr;
use std::time::{Duration, Instant};

/// Default request budget per IP per window.
pub const DEFAULT_MAX_REQUESTS: usize = 100;
/// Default window length.
pub const DEFAULT_WINDOW: Duration = Duration::from_secs(60);

/// In-memory per-IP sliding-window counter map.
#[derive(Debug)]
pub struct RateLimiter {
    inner: HashMap<IpAddr, WindowState>,
    max_requests: usize,
    window: Duration,
}

#[derive(Debug)]
struct WindowState {
    start: Instant,
    count: usize,
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

impl RateLimiter {
    /// Construct a limiter with the default budget (100) and window (60s).
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
            max_requests: DEFAULT_MAX_REQUESTS,
            window: DEFAULT_WINDOW,
        }
    }

    /// Construct a limiter with a custom budget and window.
    #[must_use]
    pub fn with_limits(max_requests: usize, window: Duration) -> Self {
        Self {
            inner: HashMap::new(),
            max_requests,
            window,
        }
    }

    /// Record a hit from `ip` and return `true` if the request should be
    /// allowed (under budget), or `false` if the budget is exhausted.
    pub fn check(&mut self, ip: IpAddr) -> bool {
        let now = Instant::now();
        let state = self.inner.entry(ip).or_insert_with(|| WindowState {
            start: now,
            count: 0,
        });

        if now.duration_since(state.start) >= self.window {
            state.start = now;
            state.count = 0;
        }

        if state.count >= self.max_requests {
            false
        } else {
            state.count += 1;
            true
        }
    }

    /// Drop entries that no longer hold any timestamps inside the window.
    /// Call periodically from a background task to bound memory.
    pub fn cleanup(&mut self) {
        let now = Instant::now();
        self.inner.retain(|_, state| now.duration_since(state.start) < self.window);
    }

    /// Number of distinct IPs currently being tracked. Useful for
    /// `/health` telemetry.
    #[must_use]
    pub fn tracked_ips(&self) -> usize {
        self.inner.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn allows_under_budget() {
        let mut rl = RateLimiter::with_limits(3, Duration::from_secs(60));
        let ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1));
        assert!(rl.check(ip));
        assert!(rl.check(ip));
        assert!(rl.check(ip));
    }

    #[test]
    fn rejects_over_budget() {
        let mut rl = RateLimiter::with_limits(2, Duration::from_secs(60));
        let ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 2));
        assert!(rl.check(ip));
        assert!(rl.check(ip));
        assert!(!rl.check(ip));
        assert!(!rl.check(ip));
    }

    #[test]
    fn budget_is_per_ip() {
        let mut rl = RateLimiter::with_limits(1, Duration::from_secs(60));
        let a = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 3));
        let b = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 4));
        assert!(rl.check(a));
        assert!(!rl.check(a));
        assert!(rl.check(b));
    }

    #[test]
    fn cleanup_drops_idle_ips() {
        let mut rl = RateLimiter::with_limits(5, Duration::from_millis(10));
        let ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 5));
        assert!(rl.check(ip));
        assert_eq!(rl.tracked_ips(), 1);
        std::thread::sleep(Duration::from_millis(20));
        rl.cleanup();
        assert_eq!(rl.tracked_ips(), 0);
    }

    #[test]
    fn expired_window_replenishes_budget() {
        let mut rl = RateLimiter::with_limits(1, Duration::from_millis(5));
        let ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 6));
        assert!(rl.check(ip));
        assert!(!rl.check(ip));
        std::thread::sleep(Duration::from_millis(10));
        assert!(rl.check(ip));
    }
}
