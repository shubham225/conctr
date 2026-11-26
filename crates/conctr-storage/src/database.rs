//! SQLx pool and schema selection shared by storage repositories.
use sqlx::{AnyPool, any::AnyPoolOptions};
use std::{future::Future, io, sync::OnceLock};
use tokio::runtime::{Builder, Runtime};

/// One pool for the database selected by the server connection URL.
#[derive(Clone)]
pub struct Database {
    pub(crate) pool: AnyPool,
    pub(crate) dialect: Dialect,
}

#[derive(Clone, Copy)]
pub(crate) enum Dialect {
    Postgres,
    MariaDb,
}

impl Database {
    /// Connect to the configured database and apply its versioned migrations.
    pub fn connect(url: &str) -> io::Result<Self> {
        let (dialect, migrations) = match url.split_once("://").map(|(scheme, _)| scheme) {
            Some("postgres" | "postgresql") => {
                (Dialect::Postgres, &sqlx::migrate!("./migrations/postgres"))
            }
            Some("mysql") => (Dialect::MariaDb, &sqlx::migrate!("./migrations/mariadb")),
            _ => {
                return Err(io::Error::other(
                    "CONCTR_DATABASE_URL requires a postgresql:// or mysql:// URL",
                ));
            }
        };
        sqlx::any::install_default_drivers();
        wait(async {
            let pool = AnyPoolOptions::new().max_connections(4).connect(url).await
                .map_err(|_| io::Error::other("Cannot connect using CONCTR_DATABASE_URL. Check the database service, database name, role and authentication."))?;
            migrations.run(&pool).await.map_err(|_| io::Error::other("Cannot apply database migrations. Check schema permissions and migration history."))?;
            Ok(Self { pool, dialect })
        })
    }
}
/// Run SQLx on its own I/O runtime. The existing auth API runs synchronous
/// password and storage operations on blocking workers. A separate driver
/// also supports CLI callers and startup before the web runtime exists.
pub(crate) fn wait<T>(operation: impl Future<Output = io::Result<T>>) -> io::Result<T> {
    static DRIVER: OnceLock<io::Result<Runtime>> = OnceLock::new();
    let driver = DRIVER
        .get_or_init(|| {
            Builder::new_multi_thread()
                .worker_threads(1)
                .enable_all()
                .build()
        })
        .as_ref()
        .map_err(|_| io::Error::other("Cannot start database runtime"))?;
    let _entered = driver.enter();
    futures_executor::block_on(operation)
}
