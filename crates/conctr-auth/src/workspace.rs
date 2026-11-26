use crate::{
    AuthError, Authority,
    local_accounts::{LocalAccounts, SetupError},
};
use std::{
    path::Path,
    sync::{Mutex, OnceLock, RwLock, RwLockReadGuard},
};

/// Local password authentication and the session authority for one workspace.
pub struct WorkspaceAuth {
    accounts: LocalAccounts,
    authority: OnceLock<Result<RwLock<Authority>, String>>,
    recovery: Mutex<crate::recovery::Recovery>,
}

/// A valid installation invitation requires a new password before issuing a session.
pub enum PasswordLogin {
    SetPassword,
    Session(String),
}

impl WorkspaceAuth {
    /// Bind this service to a previously opened account store.
    pub fn new(accounts: LocalAccounts) -> Self {
        Self {
            accounts,
            authority: OnceLock::new(),
            recovery: Mutex::default(),
        }
    }

    /// Prepare sessions or return the invitation path for the operator.
    pub fn initialize(&self) -> Result<Option<&Path>, String> {
        let invitation = self
            .accounts
            .invitation()
            .map_err(|_| "Cannot read the admin setup file.")?;
        if invitation.is_none() {
            self.load().map_err(Clone::clone)?;
        }
        Ok(invitation)
    }

    fn load(&self) -> Result<&RwLock<Authority>, &String> {
        self.authority
            .get_or_init(|| load_authority(self.accounts.store.as_ref()).map(RwLock::new))
            .as_ref()
    }

    /// Pending setup must never cache a missing account file.
    pub fn authority(&self) -> Result<RwLockReadGuard<'_, Authority>, AuthError> {
        if self
            .accounts
            .invitation()
            .map_err(|_| AuthError::Unavailable)?
            .is_some()
        {
            return Err(AuthError::InvalidSession);
        }
        self.load()
            .map_err(|_| AuthError::Unavailable)?
            .read()
            .map_err(|_| AuthError::Unavailable)
    }

    /// Access sessions already issued by this process without waiting for storage.
    pub fn session_authority(&self) -> Result<RwLockReadGuard<'_, Authority>, AuthError> {
        self.authority
            .get()
            .ok_or(AuthError::InvalidSession)?
            .as_ref()
            .map_err(|_| AuthError::Unavailable)?
            .read()
            .map_err(|_| AuthError::Unavailable)
    }

    /// Validate an invitation or sign in with a provisioned account.
    pub fn login(&self, username: &str, password: &str) -> Result<PasswordLogin, AuthError> {
        if self
            .accounts
            .invitation()
            .map_err(|_| AuthError::Unavailable)?
            .is_some()
        {
            return match self.accounts.accepts_invitation(username, password) {
                Ok(true) => Ok(PasswordLogin::SetPassword),
                Ok(false) => Err(AuthError::InvalidCredentials),
                Err(_) => Err(AuthError::Unavailable),
            };
        }
        if self
            .recovery
            .lock()
            .map_err(|_| AuthError::Unavailable)?
            .accepts(username, password)
        {
            return Ok(PasswordLogin::SetPassword);
        }
        self.authority()?
            .password_login(username, password)
            .map(PasswordLogin::Session)
    }

    /// Generate a private, expiring password for an existing account.
    pub fn request_recovery(&self, username: &str) -> Result<(), AuthError> {
        if self
            .accounts
            .invitation()
            .map_err(|_| AuthError::Unavailable)?
            .is_some()
        {
            return Ok(());
        }
        let mut recovery = self.recovery.lock().map_err(|_| AuthError::Unavailable)?;
        if self.authority()?.account(username).is_some() {
            recovery
                .issue(&self.accounts.path, username)
                .map_err(|_| AuthError::Unavailable)?;
        }
        Ok(())
    }
    /// Exchange a temporary password for a permanent password and a new session.
    pub fn set_password(
        &self,
        username: &str,
        temporary: &str,
        password: &str,
    ) -> Result<String, SetupError> {
        if self.accounts.invitation()?.is_some() && username == "admin" {
            return self.first_password(temporary, password);
        }
        let mut recovery = self.recovery.lock().map_err(|_| SetupError::Storage)?;
        if !recovery.accepts(username, temporary) {
            return Err(SetupError::Denied);
        }
        if password == temporary {
            return Err(SetupError::Invalid("Choose a different password."));
        }
        let mut authority = self
            .load()
            .map_err(|_| SetupError::Storage)?
            .write()
            .map_err(|_| SetupError::Storage)?;
        let token = authority.replace_password(self.accounts.store.as_ref(), username, password)?;
        recovery.consume(username);
        Ok(token)
    }

    /// Redeem the installation invitation and issue the administrator's first session.
    pub fn first_password(&self, temporary: &str, password: &str) -> Result<String, SetupError> {
        self.accounts.complete(temporary, password)?;
        self.authority()
            .and_then(|authority| authority.password_login("admin", password))
            .map_err(|e| match e {
                AuthError::InvalidCredentials => SetupError::Denied,
                _ => SetupError::Storage,
            })
    }
}

// These diagnostics stay in the server terminal. Account contents (including
// password hashes) must never be interpolated into configuration errors.
fn load_authority(store: &dyn crate::AccountStore) -> Result<Authority, String> {
    let accounts = store
        .load()
        .map_err(|_| "Cannot read provisioned accounts. Check the configured account store.")?;
    Authority::from_accounts(accounts).map_err(|_| {
        "Invalid provisioned accounts. Check account IDs, usernames and password hashes.".into()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recovery_preserves_identity_and_revokes_only_the_recovered_account() {
        use crate::{Account, Role};
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("accounts.json");
        let old = "previous local password";
        let new = "new local test password";
        let member = Account::new("member".into(), Role::Member, old).unwrap();
        let admin = Account::new("admin".into(), Role::Admin, old).unwrap();
        let id = member.id.clone();
        let store = crate::store::TestStore::default();
        *store.0.lock().unwrap() = vec![member, admin];
        let open = || {
            WorkspaceAuth::new(
                LocalAccounts::with_store(path.clone(), Box::new(store.clone())).unwrap(),
            )
        };
        let workspace = open();
        workspace.request_recovery("missing").unwrap();
        assert_eq!(dir.path().read_dir().unwrap().count(), 0);
        workspace.request_recovery("member").unwrap();
        workspace.request_recovery("member").unwrap();
        assert_eq!(dir.path().read_dir().unwrap().count(), 1);
        let file = dir
            .path()
            .read_dir()
            .unwrap()
            .map(|e| e.unwrap().path())
            .find(|p| p != &path)
            .unwrap();
        let handoff: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&file).unwrap()).unwrap();
        let temp = handoff["temporary_password"].as_str().unwrap();
        let login = |name| {
            workspace
                .authority()
                .unwrap()
                .password_login(name, old)
                .unwrap()
        };
        let member_session = login("member");
        let admin_session = login("admin");
        assert!(matches!(
            workspace.login("member", temp),
            Ok(PasswordLogin::SetPassword)
        ));
        assert!(matches!(
            workspace.set_password("admin", temp, new),
            Err(SetupError::Denied)
        ));
        assert!(matches!(
            workspace.set_password("member", temp, "short"),
            Err(SetupError::Invalid(_))
        ));
        let session = workspace.set_password("member", temp, new).unwrap();
        let auth = workspace.authority().unwrap();
        assert!(auth.principal(&admin_session).is_ok());
        assert!(auth.principal(&member_session).is_err());
        assert!(auth.principal(&session).unwrap().role == Role::Member);
        drop(auth);
        assert!(!file.exists());
        assert!(matches!(
            workspace.set_password("member", temp, new),
            Err(SetupError::Denied)
        ));
        let stored = crate::AccountStore::load(&store).unwrap();
        assert_eq!(stored[0].id, id);
        assert!(matches!(
            open().login("member", old),
            Err(AuthError::InvalidCredentials)
        ));
        assert!(matches!(
            open().login("member", new),
            Ok(PasswordLogin::Session(_))
        ));
    }

    #[test]
    fn pending_setup_can_become_an_active_session_without_restarting() {
        let dir = tempfile::tempdir().unwrap();
        let accounts = LocalAccounts::with_store(
            dir.path().join("accounts.json"),
            Box::<crate::store::TestStore>::default(),
        )
        .unwrap();
        let workspace = WorkspaceAuth::new(accounts);
        let invitation = workspace.initialize().unwrap().unwrap();
        let temporary = std::fs::read_to_string(invitation).unwrap();
        assert!(matches!(
            workspace.authority(),
            Err(AuthError::InvalidSession)
        ));
        assert!(matches!(
            workspace.login("admin", &temporary),
            Ok(PasswordLogin::SetPassword)
        ));

        let password = "workspace test password";
        let token = workspace.first_password(&temporary, password).unwrap();
        assert!(workspace.initialize().unwrap().is_none());
        assert_eq!(
            workspace
                .authority()
                .unwrap()
                .principal(&token)
                .unwrap()
                .username,
            "admin"
        );
        assert!(matches!(
            workspace.first_password(&temporary, password),
            Err(SetupError::Denied)
        ));
        assert!(matches!(
            workspace.login("admin", password),
            Ok(PasswordLogin::Session(_))
        ));
    }
}
