#[cfg(test)]
use crate::Role;
use crate::{Account, Principal};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode, get_current_timestamp,
};
use rand::{RngCore, rngs::OsRng};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Mutex};
use uuid::Uuid;

/// Every login has an absolute lifetime, including sessions that are actively used.
pub const SESSION_SECONDS: u64 = 15 * 60;
const ISSUER: &str = "conctr";
const AUDIENCE: &str = "conctr-api";

/// Signed JWT payload. Role and workspace policy are looked up on each request.
#[derive(Serialize, Deserialize)]
struct Claims {
    iss: String,
    aud: String,
    sub: String,
    exp: u64,
    jti: String,
}

/// Failure categories safe to translate into HTTP status codes.
#[derive(Debug, PartialEq)]
pub enum AuthError {
    InvalidCredentials,
    InvalidSession,
    Unavailable,
}

/// One workspace server owns one account directory and its active sessions.
pub struct Authority {
    accounts: Vec<Account>,
    signing_key: [u8; 32],
    sessions: Mutex<HashMap<String, (u64, String)>>,
}

impl Authority {
    /// Load trusted operator configuration once at startup (or first request).
    /// Each process generates a new signing key, so restarting signs everyone out.
    pub fn from_json(json: &[u8]) -> Result<Self, AuthError> {
        let accounts: Vec<Account> =
            serde_json::from_slice(json).map_err(|_| AuthError::Unavailable)?;
        if accounts.is_empty() {
            return Err(AuthError::Unavailable);
        }
        for (index, account) in accounts.iter().enumerate() {
            let hash =
                PasswordHash::new(&account.password_hash).map_err(|_| AuthError::Unavailable)?;
            if hash.algorithm.as_str() != "argon2id"
                || hash.hash.is_none()
                || hash.salt.is_none()
                || account.username.trim().is_empty()
                || account.username.len() > 128
                || account.id.trim().is_empty()
                || accounts[..index]
                    .iter()
                    .any(|other| other.id == account.id || other.username == account.username)
            {
                return Err(AuthError::Unavailable);
            }
        }
        let mut signing_key = [0; 32];
        OsRng
            .try_fill_bytes(&mut signing_key)
            .map_err(|_| AuthError::Unavailable)?;
        Ok(Self {
            accounts,
            signing_key,
            sessions: Mutex::default(),
        })
    }

    pub(crate) fn account(&self, username: &str) -> Option<&Account> {
        self.accounts
            .iter()
            .find(|account| account.username == username)
    }

    pub(crate) fn replace_password(
        &mut self,
        path: &std::path::Path,
        username: &str,
        password: &str,
    ) -> Result<String, crate::local_accounts::SetupError> {
        use crate::local_accounts::SetupError;
        let account = self
            .accounts
            .iter_mut()
            .find(|a| a.username == username)
            .ok_or(SetupError::Denied)?;
        let hash = Account::new(username.into(), account.role, password)
            .map_err(SetupError::Invalid)?
            .password_hash;
        let mut stored: Vec<Account> = serde_json::from_slice(&std::fs::read(path)?)?;
        let record = stored
            .iter_mut()
            .find(|a| {
                a.id == account.id
                    && a.username == username
                    && a.password_hash == account.password_hash
                    && a.role == account.role
            })
            .ok_or(SetupError::Storage)?;
        record.password_hash.clone_from(&hash);
        let mut tmp = tempfile::NamedTempFile::new_in(path.parent().ok_or(SetupError::Storage)?)?;
        serde_json::to_writer_pretty(&mut tmp, &stored)?;
        tmp.as_file().sync_all()?;
        let sessions = self.sessions.get_mut().map_err(|_| SetupError::Storage)?;
        tmp.persist(path).map_err(|e| e.error)?;
        account.password_hash = hash;
        sessions.retain(|_, (_, id)| id != &account.id);
        self.issue_session(self.account(username).ok_or(SetupError::Storage)?)
            .map_err(|_| SetupError::Storage)
    }

    /// Run on a blocking thread. Unknown usernames still incur a password hash
    /// check to avoid an obvious timing difference that reveals existing users.
    pub fn password_login(&self, username: &str, password: &str) -> Result<String, AuthError> {
        if username.len() > 128 || password.len() > 1024 || password.is_empty() {
            return Err(AuthError::InvalidCredentials);
        }
        let account = self.accounts.iter().find(|a| a.username == username);
        let encoded = &account.unwrap_or(&self.accounts[0]).password_hash;
        let hash = PasswordHash::new(encoded).map_err(|_| AuthError::Unavailable)?;
        Argon2::default()
            .verify_password(password.as_bytes(), &hash)
            .map_err(|_| AuthError::InvalidCredentials)?;
        self.issue_session(account.ok_or(AuthError::InvalidCredentials)?)
    }

    // A future OIDC callback should resolve a verified identity to a local
    // account first, then enter here. Provider tokens never authorize Conctr APIs.
    fn issue_session(&self, account: &Account) -> Result<String, AuthError> {
        let now = get_current_timestamp();
        let claims = Claims {
            iss: ISSUER.into(),
            aud: AUDIENCE.into(),
            sub: account.id.clone(),
            jti: Uuid::new_v4().to_string(),
            exp: now + SESSION_SECONDS,
        };
        let token = encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(&self.signing_key),
        )
        .map_err(|_| AuthError::Unavailable)?;
        let mut sessions = self.sessions.lock().map_err(|_| AuthError::Unavailable)?;
        sessions.retain(|_, (exp, _)| *exp > now);
        sessions.insert(claims.jti, (claims.exp, claims.sub));
        Ok(token)
    }

    fn verify(&self, token: &str) -> Result<Claims, AuthError> {
        let mut rules = Validation::new(Algorithm::HS256);
        rules.set_issuer(&[ISSUER]);
        rules.set_audience(&[AUDIENCE]);
        rules.set_required_spec_claims(&["iss", "aud", "sub", "exp"]);
        rules.leeway = 0;
        let claims = decode::<Claims>(token, &DecodingKey::from_secret(&self.signing_key), &rules)
            .map_err(|_| AuthError::InvalidSession)?
            .claims;
        let sessions = self.sessions.lock().map_err(|_| AuthError::Unavailable)?;
        if claims.exp <= get_current_timestamp()
            || sessions.get(&claims.jti).map(|s| s.0) != Some(claims.exp)
        {
            return Err(AuthError::InvalidSession);
        }
        Ok(claims)
    }

    /// Return only the identity, role and expiry needed by the application.
    pub fn principal(&self, token: &str) -> Result<Principal<'_>, AuthError> {
        let claims = self.verify(token)?;
        let account = self
            .accounts
            .iter()
            .find(|a| a.id == claims.sub)
            .ok_or(AuthError::InvalidSession)?;
        Ok(Principal {
            username: &account.username,
            role: account.role,
            remaining_seconds: claims.exp.saturating_sub(get_current_timestamp()),
        })
    }

    /// Revoke the individual login, including any copies of its cookie.
    pub fn logout(&self, token: &str) -> Result<(), AuthError> {
        let claims = self.verify(token)?;
        self.sessions
            .lock()
            .map_err(|_| AuthError::Unavailable)?
            .remove(&claims.jti);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use argon2::{PasswordHasher, password_hash::SaltString};

    fn authority() -> Authority {
        let hash = Argon2::default()
            .hash_password(
                b"temporary test password",
                &SaltString::generate(&mut OsRng),
            )
            .unwrap()
            .to_string();
        let account = Account {
            id: "local-id".into(),
            username: "operator".into(),
            role: Role::Member,
            password_hash: hash,
        };
        Authority::from_json(&serde_json::to_vec(&[account]).unwrap()).unwrap()
    }

    #[test]
    fn password_login_and_per_session_logout() {
        let auth = authority();
        for (name, password) in [
            ("unknown", "temporary test password"),
            ("operator", "incorrect"),
        ] {
            assert_eq!(
                auth.password_login(name, password),
                Err(AuthError::InvalidCredentials)
            );
        }
        let first = auth
            .password_login("operator", "temporary test password")
            .unwrap();
        let second = auth.issue_session(&auth.accounts[0]).unwrap();
        assert!(matches!(
            auth.principal(&first),
            Ok(Principal {
                username: "operator",
                role: Role::Member,
                ..
            })
        ));
        auth.logout(&first).unwrap();
        assert!(matches!(
            auth.principal(&first),
            Err(AuthError::InvalidSession)
        ));
        assert!(auth.principal(&second).is_ok());
    }

    #[test]
    fn token_checks_require_our_signature_claims_and_active_session() {
        let auth = authority();
        let token = auth.issue_session(&auth.accounts[0]).unwrap();
        assert!(authority().principal(&token).is_err());
        for field in ["issuer", "audience", "expiry", "session", "subject"] {
            let mut claims = auth.verify(&token).unwrap();
            match field {
                "issuer" => claims.iss = "external".into(),
                "audience" => claims.aud = "different-api".into(),
                "expiry" => claims.exp = get_current_timestamp() - 1,
                "session" => claims.jti = "unknown".into(),
                _ => claims.sub = "deleted".into(),
            }
            let token = encode(
                &Header::new(Algorithm::HS256),
                &claims,
                &EncodingKey::from_secret(&auth.signing_key),
            )
            .unwrap();
            assert!(auth.principal(&token).is_err(), "{field}");
        }
    }
}
