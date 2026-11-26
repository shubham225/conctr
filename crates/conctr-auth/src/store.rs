//! Persistence operations required by workspace authentication.
use crate::{Account, local_accounts::SetupError};
use std::io;

/// Account backends serve a single workspace. Implementations must preserve
/// identity on password changes and guard initial provisioning against races.
pub trait AccountStore: Send + Sync {
    /// Distinguish a fresh installation from an existing account collection.
    fn provisioned(&self) -> io::Result<bool>;
    /// Load provisioned accounts for the workspace session authority.
    fn load(&self) -> io::Result<Vec<Account>>;
    /// Atomically provision the first account if the store is still fresh.
    fn create_first(&self, account: &Account) -> Result<(), SetupError>;
    /// Replace the hash only while the persisted record matches the caller's copy.
    fn replace_hash(&self, previous: &Account, hash: &str) -> Result<(), SetupError>;
}

#[cfg(test)]
#[derive(Clone, Default)]
pub(crate) struct TestStore(pub std::sync::Arc<std::sync::Mutex<Vec<Account>>>);

#[cfg(test)]
impl AccountStore for TestStore {
    fn provisioned(&self) -> io::Result<bool> {
        Ok(!self.0.lock().unwrap().is_empty())
    }
    fn load(&self) -> io::Result<Vec<Account>> {
        Ok(self.0.lock().unwrap().clone())
    }
    fn create_first(&self, account: &Account) -> Result<(), SetupError> {
        let mut rows = self.0.lock().unwrap();
        if !rows.is_empty() {
            return Err(SetupError::Denied);
        }
        rows.push(account.clone());
        Ok(())
    }
    fn replace_hash(&self, previous: &Account, hash: &str) -> Result<(), SetupError> {
        let mut rows = self.0.lock().unwrap();
        let row = rows
            .iter_mut()
            .find(|a| a.id == previous.id && a.password_hash == previous.password_hash)
            .ok_or(SetupError::Storage)?;
        row.password_hash = hash.into();
        Ok(())
    }
}
