//! Per-workspace, rolling password-attempt quotas.
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

/// Track per-name and total attempts within a bounded one-minute window.
#[derive(Default)]
pub struct LoginAttempts {
    attempts: VecDeque<(Instant, String)>,
}
impl LoginAttempts {
    /// Reserve one attempt using a monotonic clock.
    pub fn reserve(&mut self, now: Instant, username: &str) -> bool {
        self.attempts
            .retain(|(at, _)| now.duration_since(*at) < Duration::from_secs(60));
        if self.attempts.len() >= 120
            || self
                .attempts
                .iter()
                .filter(|(_, name)| name == username)
                .count()
                >= 10
        {
            return false;
        }
        self.attempts.push_back((now, username.to_owned()));
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn login_budget_is_per_name_and_expires() {
        let mut attempts = LoginAttempts::default();
        let now = Instant::now();
        for _ in 0..10 {
            assert!(attempts.reserve(now, "first"));
        }
        assert!(!attempts.reserve(now, "first"));
        assert!(attempts.reserve(now, "second"));
        assert!(attempts.reserve(now + Duration::from_secs(60), "first"));
    }
}
