use crate::database::{Database, Dialect, wait};
use conctr_auth::{Account, AccountStore, Authority, Role, local_accounts::SetupError};
use sqlx::{Any, Transaction};
use std::{fs, io, path::Path};

/// SQL account repository sharing the application database pool.
pub struct SqlAccounts {
    db: Database,
}

fn unavailable(_: sqlx::Error) -> io::Error {
    io::Error::other("Cannot access database account storage")
}

impl SqlAccounts {
    /// Apply versioned migrations and import the legacy file once, preserving
    /// account IDs, roles and hashes. Failed imports can be corrected and retried.
    pub fn open(db: Database, legacy_file: &Path) -> io::Result<Self> {
        wait(async {
            let mut tx = db.pool.begin().await.map_err(unavailable)?;
            let pending = sqlx::query("SELECT 1 FROM conctr_auth_installation WHERE singleton AND NOT import_complete FOR UPDATE")
                .fetch_optional(&mut *tx).await.map_err(unavailable)?.is_some();
            if pending {
                let existing = legacy_file.try_exists()?;
                if existing {
                    let accounts: Vec<Account> = serde_json::from_slice(&fs::read(legacy_file)?)
                        .map_err(|_| io::Error::other("Invalid legacy account file"))?;
                    Authority::validate_accounts(&accounts)
                        .map_err(|_| io::Error::other("Invalid legacy accounts"))?;
                    for account in &accounts {
                        insert(&mut tx, db.dialect, account).await?;
                    }
                }
                let mark_ready = if existing {
                    "UPDATE conctr_auth_installation SET import_complete = true, provisioned = true WHERE singleton"
                } else {
                    "UPDATE conctr_auth_installation SET import_complete = true WHERE singleton"
                };
                sqlx::query(mark_ready)
                    .execute(&mut *tx)
                    .await
                    .map_err(unavailable)?;
            }
            tx.commit().await.map_err(unavailable)?;
            Ok(Self { db })
        })
    }
}

fn role_name(role: Role) -> &'static str {
    match role {
        Role::Admin => "admin",
        Role::Member => "member",
    }
}

async fn insert(
    tx: &mut Transaction<'_, Any>,
    dialect: Dialect,
    account: &Account,
) -> io::Result<()> {
    let statement = match dialect {
        Dialect::Postgres => {
            "INSERT INTO conctr_accounts (id, username, role, password_hash) VALUES ($1, $2, $3, $4)"
        }
        Dialect::MariaDb => {
            "INSERT INTO conctr_accounts (id, username, role, password_hash) VALUES (?, ?, ?, ?)"
        }
    };
    sqlx::query(statement)
        .bind(&account.id)
        .bind(&account.username)
        .bind(role_name(account.role))
        .bind(&account.password_hash)
        .execute(&mut **tx)
        .await
        .map_err(unavailable)?;
    Ok(())
}

impl AccountStore for SqlAccounts {
    fn provisioned(&self) -> io::Result<bool> {
        wait(async {
            sqlx::query("SELECT 1 FROM conctr_auth_installation WHERE singleton AND provisioned")
                .fetch_optional(&self.db.pool)
                .await
                .map(|row| row.is_some())
                .map_err(unavailable)
        })
    }

    fn load(&self) -> io::Result<Vec<Account>> {
        wait(async {
            let rows = sqlx::query_as::<_, (String, String, String, String)>(
                "SELECT id, username, role, password_hash FROM conctr_accounts",
            )
            .fetch_all(&self.db.pool)
            .await
            .map_err(unavailable)?;
            rows.into_iter()
                .map(|(id, username, role, password_hash)| {
                    let role = match role.as_str() {
                        "admin" => Role::Admin,
                        "member" => Role::Member,
                        _ => return Err(io::Error::other("Invalid stored account role")),
                    };
                    Ok(Account {
                        id,
                        username,
                        role,
                        password_hash,
                    })
                })
                .collect()
        })
    }

    fn create_first(&self, account: &Account) -> Result<(), SetupError> {
        let claimed = wait(async {
            let mut tx = self.db.pool.begin().await.map_err(unavailable)?;
            let claimed = sqlx::query("UPDATE conctr_auth_installation SET provisioned = true WHERE singleton AND NOT provisioned")
                .execute(&mut *tx).await.map_err(unavailable)?.rows_affected() == 1;
            if claimed {
                insert(&mut tx, self.db.dialect, account).await?;
            }
            tx.commit().await.map_err(unavailable)?;
            Ok(claimed)
        })?;
        if claimed {
            Ok(())
        } else {
            Err(SetupError::Denied)
        }
    }

    fn replace_hash(&self, previous: &Account, hash: &str) -> Result<(), SetupError> {
        let changed = wait(async {
            let statement = match self.db.dialect {
                Dialect::Postgres => {
                    "UPDATE conctr_accounts SET password_hash = $1 WHERE id = $2 AND username = $3 AND role = $4 AND password_hash = $5"
                }
                Dialect::MariaDb => {
                    "UPDATE conctr_accounts SET password_hash = ? WHERE id = ? AND username = ? AND role = ? AND password_hash = ?"
                }
            };
            sqlx::query(statement)
                .bind(hash)
                .bind(&previous.id)
                .bind(&previous.username)
                .bind(role_name(previous.role))
                .bind(&previous.password_hash)
                .execute(&self.db.pool)
                .await
                .map(|result| result.rows_affected())
                .map_err(unavailable)
        })?;
        if changed == 1 {
            Ok(())
        } else {
            Err(SetupError::Storage)
        }
    }
}
