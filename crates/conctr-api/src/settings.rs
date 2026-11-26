//! Server startup settings. The native clients only need the public API origin.
use std::{
    env,
    io::{self, Write},
    path::{Path, PathBuf},
    sync::LazyLock,
};
use tempfile::NamedTempFile;

const DEFAULTS: &[u8] = include_bytes!("../../../.env.example");

/// Load once, before the server accepts requests. Real environment variables
/// keep precedence over .env values. Missing and empty paths use OS defaults.
pub(super) static SETTINGS: LazyLock<Result<Settings, String>> = LazyLock::new(|| {
    let cwd = env::current_dir().map_err(|err| err.to_string())?;
    let file = prepare_env(&cwd).map_err(|err| format!("Cannot prepare the server .env: {err}"))?;
    dotenvy::from_path(&file).map_err(|_| {
        format!(
            "Cannot read {}. Check its permissions and KEY=value entries.",
            file.display()
        )
    })?;
    let dir = file.parent().ok_or("Invalid .env path")?;
    let path = |key| {
        env::var_os(key)
            .filter(|value| !value.is_empty())
            .map(|value| dir.join(value))
    };
    Ok(Settings {
        database_url: match env::var("CONCTR_DATABASE_URL") {
            Ok(url) if !url.trim().is_empty() => url,
            Ok(_) | Err(env::VarError::NotPresent) => return Err(
                "Set CONCTR_DATABASE_URL in the server environment or .env before starting Conctr."
                    .into(),
            ),
            Err(_) => return Err("Invalid CONCTR_DATABASE_URL encoding".into()),
        },
        accounts_file: path("CONCTR_ACCOUNTS_FILE"),
    })
});

pub(super) struct Settings {
    pub database_url: String,
    pub accounts_file: Option<PathBuf>,
}

fn prepare_env(cwd: &Path) -> io::Result<PathBuf> {
    // dx can start the executable below the workspace root. The checked-in
    // example identifies where its editable .env belongs on a fresh checkout.
    let mut dir = cwd;
    for ancestor in cwd.ancestors() {
        let file = ancestor.join(".env");
        if file.try_exists()? {
            return Ok(file);
        }
        if ancestor.join(".env.example").is_file() {
            dir = ancestor;
            break;
        }
    }
    let file = dir.join(".env");
    let mut tmp = NamedTempFile::new_in(dir)?;
    tmp.write_all(DEFAULTS)?;
    tmp.as_file().sync_all()?;
    match tmp.persist_noclobber(&file) {
        Ok(_) => {}
        Err(err) if err.error.kind() == io::ErrorKind::AlreadyExists => {}
        Err(err) => return Err(err.error),
    }
    Ok(file)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn creates_workspace_defaults_once_and_preserves_user_edits() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::write(root.join(".env.example"), DEFAULTS).unwrap();
        let nested = root.join("build");
        fs::create_dir(&nested).unwrap();
        let file = prepare_env(&nested).unwrap();
        assert_eq!(file, root.join(".env"));
        assert_eq!(fs::read(&file).unwrap(), DEFAULTS);
        fs::write(&file, "CONCTR_ACCOUNTS_FILE=custom/users.json\n").unwrap();
        assert_eq!(prepare_env(&nested).unwrap(), file);
        assert_eq!(
            fs::read_to_string(file).unwrap(),
            "CONCTR_ACCOUNTS_FILE=custom/users.json\n"
        );
    }
}
