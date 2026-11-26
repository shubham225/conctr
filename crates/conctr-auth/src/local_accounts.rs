//! First-run provisioning for a single-workspace installation.
use crate::{Account, Role};
use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};
use subtle::ConstantTimeEq;
use tempfile::NamedTempFile;
use uuid::Uuid;

/// The private first-login invitation belongs to one account file.
pub struct LocalAccounts {
    pub path: PathBuf,
    pub(crate) store: Box<dyn crate::AccountStore>,
    invitation: Option<PathBuf>,
    setup_finished: std::sync::atomic::AtomicBool,
}

impl LocalAccounts {
    /// Resolve the legacy file location and operator handoff directory.
    pub fn account_path(configured: Option<PathBuf>) -> io::Result<PathBuf> {
        Ok(match configured {
            Some(path) => path,
            None => dirs::config_dir()
                .ok_or_else(|| io::Error::other("Cannot locate user configuration directory"))?
                .join("conctr/accounts.json"),
        })
    }

    /// Use the given backend for account data and the path for private handoffs.
    pub fn with_store(path: PathBuf, store: Box<dyn crate::AccountStore>) -> io::Result<Self> {
        let dir = path
            .parent()
            .ok_or_else(|| io::Error::other("Expected an account file path"))?;
        let mut builder = fs::DirBuilder::new();
        builder.recursive(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            builder.mode(0o700);
        }
        builder.create(dir)?;
        if store.provisioned()? {
            return Ok(Self {
                path,
                invitation: None,
                setup_finished: true.into(),
                store,
            });
        }
        // Multiple account files can share a directory. Keep the historical
        // invitation name for accounts.json so pending setups survive upgrades.
        let filename = path
            .file_name()
            .ok_or_else(|| io::Error::other("Expected an account filename"))?;
        let invitation = if filename == "accounts.json" {
            dir.join("initial-admin-password")
        } else {
            let mut name = filename.to_owned();
            name.push(".initial-admin-password");
            dir.join(name)
        };
        if !invitation.try_exists()? {
            let secret = Uuid::new_v4().simple().to_string();
            match publish(&invitation, secret.as_bytes()) {
                Ok(()) => {}
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
                Err(e) => return Err(e),
            }
        }
        Ok(Self {
            path,
            invitation: Some(invitation),
            setup_finished: false.into(),
            store,
        })
    }

    /// Return the handoff path only while the store accepts initial provisioning.
    pub fn invitation(&self) -> io::Result<Option<&Path>> {
        if self
            .setup_finished
            .load(std::sync::atomic::Ordering::Acquire)
        {
            return Ok(None);
        }
        if self.store.provisioned()? {
            self.setup_finished
                .store(true, std::sync::atomic::Ordering::Release);
            Ok(None)
        } else {
            Ok(self.invitation.as_deref())
        }
    }

    pub fn accepts_invitation(&self, username: &str, password: &str) -> io::Result<bool> {
        let Some(path) = self.invitation()? else {
            return Ok(false);
        };
        let secret = fs::read(path)?;
        Ok(username == "admin"
            && secret.len() == 32
            && bool::from(secret.ct_eq(password.as_bytes())))
    }

    /// Recheck the invitation at commit time. Atomic creation ensures two
    /// browsers finishing setup concurrently cannot overwrite each other's choice.
    pub fn complete(&self, temporary: &str, password: &str) -> Result<(), SetupError> {
        if !self.accepts_invitation("admin", temporary)? {
            return Err(SetupError::Denied);
        }
        if password == temporary {
            return Err(SetupError::Invalid("Choose a different password."));
        }
        let account =
            Account::new("admin".into(), Role::Admin, password).map_err(SetupError::Invalid)?;
        self.store.create_first(&account)?;
        self.setup_finished
            .store(true, std::sync::atomic::Ordering::Release);
        // The account store is the commit point. The temporary password is
        // already invalid even if removing the private file fails.
        if let Some(path) = &self.invitation {
            let _ = fs::remove_file(path);
        }
        Ok(())
    }
}

/// Public errors contain no password or hash.
#[derive(Debug)]
pub enum SetupError {
    Denied,
    Invalid(&'static str),
    Storage,
}
impl From<io::Error> for SetupError {
    fn from(_: io::Error) -> Self {
        Self::Storage
    }
}
impl From<serde_json::Error> for SetupError {
    fn from(_: serde_json::Error) -> Self {
        Self::Storage
    }
}

// tempfile creates a private file and publishes the complete contents without
// replacing any existing file. Readers never observe a partial JSON document.
pub(crate) fn publish(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| io::Error::other("Missing account directory"))?;
    let mut file = NamedTempFile::new_in(parent)?;
    file.write_all(bytes)?;
    file.as_file().sync_all()?;
    file.persist_noclobber(path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AccountStore, Authority, store::TestStore};

    #[test]
    fn missing_custom_files_get_independent_private_invitations() {
        let dir = tempfile::tempdir().unwrap();
        let nested = dir.path().join("workspace config");
        let first =
            LocalAccounts::with_store(nested.join("operators.json"), Box::<TestStore>::default())
                .unwrap();
        let second =
            LocalAccounts::with_store(nested.join("members.json"), Box::<TestStore>::default())
                .unwrap();
        let code = fs::read_to_string(first.invitation().unwrap().unwrap()).unwrap();
        assert!(first.accepts_invitation("admin", &code).unwrap());
        assert!(!second.accepts_invitation("admin", &code).unwrap());
        first.complete(&code, "a new admin password").unwrap();
        assert!(!nested.join("operators.json").exists());
        assert!(second.invitation().unwrap().unwrap().is_file());
    }

    #[test]
    fn setup_survives_restart_and_is_consumed_once() {
        let dir = tempfile::tempdir().unwrap();
        let store = TestStore::default();
        let open = || {
            LocalAccounts::with_store(dir.path().join("accounts.json"), Box::new(store.clone()))
                .unwrap()
        };
        let accounts = open();
        let path = accounts.invitation().unwrap().unwrap();
        let secret = fs::read_to_string(path).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(fs::metadata(path).unwrap().permissions().mode() & 0o077, 0);
        }
        let restarted = open();
        assert!(restarted.accepts_invitation("admin", &secret).unwrap());
        assert!(matches!(
            accounts.complete("wrong", "a new admin password"),
            Err(SetupError::Denied)
        ));
        assert!(matches!(
            accounts.complete(&secret, "short"),
            Err(SetupError::Invalid(_))
        ));
        accounts.complete(&secret, "a new admin password").unwrap();
        assert!(!path.exists());
        assert!(matches!(
            restarted.complete(&secret, "another admin password"),
            Err(SetupError::Denied)
        ));
        let permanent = open();
        assert!(permanent.invitation().unwrap().is_none());
        let auth = Authority::from_accounts(store.load().unwrap()).unwrap();
        assert!(auth.password_login("admin", "a new admin password").is_ok());
        assert!(auth.password_login("admin", &secret).is_err());
    }
}
