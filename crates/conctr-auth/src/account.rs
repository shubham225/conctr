use argon2::Argon2;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Roles are assigned by the workspace account store.
#[derive(Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// Access the workspace shell and authenticated general APIs.
    Member,
    /// Workspace administrator.
    Admin,
}

/// Values an application may use after a successful session check.
pub struct Principal<'a> {
    pub username: &'a str,
    pub role: Role,
    pub remaining_seconds: u64,
}

/// Provisioned account. Keeping the local ID independent of the login name lets
/// future identity providers map their verified (issuer, subject) to the same ID.
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Account {
    pub id: String,
    pub username: String,
    pub role: Role,
    pub password_hash: String,
}

impl Account {
    /// Apply the same password policy and Argon2id parameters in the CLI and
    /// the first-login form. Each account gets an independent random salt.
    pub fn new(username: String, role: Role, password: &str) -> Result<Self, &'static str> {
        if username.trim().is_empty() || username.len() > 128 {
            return Err("Choose a username between 1 and 128 bytes.");
        }
        let password_hash = hash_password(password)?;
        Ok(Self {
            id: Uuid::new_v4().to_string(),
            username,
            role,
            password_hash,
        })
    }
}

pub(crate) fn hash_password(password: &str) -> Result<String, &'static str> {
    use argon2::{PasswordHasher, password_hash::SaltString};
    if password.chars().count() < 15 || password.len() > 1024 {
        return Err("Use at least 15 characters and at most 1024 bytes.");
    }
    Ok(Argon2::default()
        .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
        .map_err(|_| "Cannot hash password.")?
        .to_string())
}
