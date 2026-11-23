use conctr_auth::LoginAttempts;
use std::{
    sync::{LazyLock, Mutex},
    time::Instant,
};
use tokio::sync::Semaphore;

static WORKERS: Semaphore = Semaphore::const_new(2);
static ATTEMPTS: LazyLock<Mutex<LoginAttempts>> = LazyLock::new(Mutex::default);

pub(super) fn acquire(
    username: &str,
) -> Result<tokio::sync::SemaphorePermit<'static>, super::Problem> {
    reserve(username, &ATTEMPTS)
}
pub(super) fn acquire_recovery(
    username: &str,
) -> Result<tokio::sync::SemaphorePermit<'static>, super::Problem> {
    static RECOVERY: LazyLock<Mutex<LoginAttempts>> = LazyLock::new(Mutex::default);
    reserve(username, &RECOVERY)
}
fn reserve(
    username: &str,
    attempts: &Mutex<LoginAttempts>,
) -> Result<tokio::sync::SemaphorePermit<'static>, super::Problem> {
    let busy = || {
        super::Problem::new(
            super::Kind::RateLimited,
            "Too many sign-in attempts. Try again in a minute.",
        )
    };
    let permit = WORKERS.try_acquire().map_err(|_| busy())?;
    {
        let mut attempts = attempts
            .lock()
            .map_err(|_| super::Problem::from(conctr_auth::AuthError::Unavailable))?;
        if !attempts.reserve(Instant::now(), username) {
            return Err(busy());
        }
    }
    Ok(permit)
}
