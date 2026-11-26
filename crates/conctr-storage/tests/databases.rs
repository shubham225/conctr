//! SQLx gives each test its own database on the PostgreSQL test server.
use conctr_auth::{
    Account, AccountStore, AuthError, PasswordLogin, Role, WorkspaceAuth,
    local_accounts::LocalAccounts,
};
use conctr_storage::{Database, SqlAccounts};
use sqlx::{ConnectOptions, MySqlPool, PgPool};
use std::{fs, path::Path};

fn workspace(url: &str, path: &Path) -> WorkspaceAuth {
    let store = SqlAccounts::open(Database::connect(url).unwrap(), path).unwrap();
    WorkspaceAuth::new(LocalAccounts::with_store(path.into(), Box::new(store)).unwrap())
}

#[sqlx::test(migrations = false)]
#[ignore = "requires DATABASE_URL for a disposable PostgreSQL instance"]
async fn postgres_first_login_recovery_and_restart(pool: PgPool) {
    first_login_recovery_and_restart(pool.connect_options().to_url_lossy().as_str())
}

#[sqlx::test(migrations = false)]
#[ignore = "requires DATABASE_URL for a disposable MariaDB instance"]
async fn mariadb_first_login_recovery_and_restart(pool: MySqlPool) {
    first_login_recovery_and_restart(pool.connect_options().to_url_lossy().as_str())
}

fn first_login_recovery_and_restart(url: &str) {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("accounts.json");
    let auth = workspace(url, &path);
    let setup = fs::read_to_string(auth.initialize().unwrap().unwrap()).unwrap();
    let old = "original test password";
    let new = "different test password";
    let session = auth.first_password(&setup, old).unwrap();
    assert!(auth.first_password(&setup, new).is_err());
    assert!(!path.exists());
    auth.request_recovery("admin").unwrap();
    let handoff = fs::read_dir(dir.path())
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    let data: serde_json::Value = serde_json::from_slice(&fs::read(handoff).unwrap()).unwrap();
    auth.set_password("admin", data["temporary_password"].as_str().unwrap(), new)
        .unwrap();
    assert!(
        auth.session_authority()
            .unwrap()
            .principal(&session)
            .is_err()
    );
    drop(auth);
    let restarted = workspace(url, &path);
    assert!(restarted.initialize().unwrap().is_none());
    assert!(matches!(
        restarted.login("admin", old),
        Err(AuthError::InvalidCredentials)
    ));
    assert!(matches!(
        restarted.login("admin", new),
        Ok(PasswordLogin::Session(_))
    ));
}

#[sqlx::test(migrations = false)]
#[ignore = "requires DATABASE_URL for a disposable PostgreSQL instance"]
async fn postgres_imports_existing_records_once(pool: PgPool) {
    imports_existing_records_once(pool.connect_options().to_url_lossy().as_str())
}

#[sqlx::test(migrations = false)]
#[ignore = "requires DATABASE_URL for a disposable MariaDB instance"]
async fn mariadb_imports_existing_records_once(pool: MySqlPool) {
    imports_existing_records_once(pool.connect_options().to_url_lossy().as_str())
}

fn imports_existing_records_once(url: &str) {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("accounts.json");
    let account = Account::new("owner".into(), Role::Admin, "legacy test password").unwrap();
    fs::write(&path, "invalid").unwrap();
    assert!(SqlAccounts::open(Database::connect(url).unwrap(), &path).is_err());
    fs::write(&path, serde_json::to_vec(&[&account]).unwrap()).unwrap();
    let store = SqlAccounts::open(Database::connect(url).unwrap(), &path).unwrap();
    let rows = store.load().unwrap();
    assert_eq!(rows.len(), 1);
    assert!(rows[0].id == account.id && rows[0].password_hash == account.password_hash);
    assert!(store.create_first(&account).is_err());
    fs::write(&path, "invalid").unwrap();
    assert!(workspace(url, &path).initialize().unwrap().is_none());
}
