//! One private operator handoff per account. Only this process can redeem it.
use std::{
    collections::HashMap,
    io::Write,
    path::Path,
    time::{Duration, Instant},
};
use subtle::ConstantTimeEq;
use tempfile::{NamedTempFile, TempPath};
use uuid::Uuid;

struct Invitation {
    secret: String,
    expires: Instant,
    _file: TempPath,
}
#[derive(Default)]
pub(crate) struct Recovery {
    pending: HashMap<String, Invitation>,
}

impl Recovery {
    pub fn issue(&mut self, account_file: &Path, username: &str) -> std::io::Result<()> {
        let now = Instant::now();
        self.pending.retain(|_, invite| invite.expires > now);
        if self.pending.contains_key(username) {
            return Ok(());
        }
        let secret = format!("reset-{}", Uuid::new_v4().simple());
        let dir = account_file
            .parent()
            .ok_or_else(|| std::io::Error::other("Missing account directory"))?;
        let mut file = NamedTempFile::with_prefix_in("conctr-password-reset-", dir)?;
        serde_json::to_writer(
            file.as_file_mut(),
            &serde_json::json!({"username":username,"temporary_password":secret}),
        )?;
        file.flush()?;
        let path = file.into_temp_path();
        println!(
            "Conctr password recovery — private operator handoff: {}",
            path.display()
        );
        self.pending.insert(
            username.into(),
            Invitation {
                secret,
                expires: now + Duration::from_secs(900),
                _file: path,
            },
        );
        Ok(())
    }

    pub fn accepts(&self, username: &str, temporary: &str) -> bool {
        self.pending.get(username).is_some_and(|invite| {
            invite.expires > Instant::now()
                && bool::from(invite.secret.as_bytes().ct_eq(temporary.as_bytes()))
        })
    }

    pub fn consume(&mut self, username: &str) {
        self.pending.remove(username);
    }
}
