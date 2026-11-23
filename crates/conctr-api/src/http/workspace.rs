use super::Problem;
use crate::settings::SETTINGS;
use conctr_auth::{AuthError, WorkspaceAuth, local_accounts::LocalAccounts};

use std::sync::LazyLock;

static WORKSPACE: LazyLock<Result<WorkspaceAuth, String>> = LazyLock::new(|| {
    let settings = SETTINGS.as_ref().map_err(Clone::clone)?;
    LocalAccounts::open(settings.accounts_file.clone())
        .map(WorkspaceAuth::new)
        .map_err(|err| format!("Cannot prepare the account file or first-login invitation: {err}"))
});

pub(super) fn current_workspace() -> Result<&'static WorkspaceAuth, Problem> {
    WORKSPACE
        .as_ref()
        .map_err(|_| AuthError::Unavailable.into())
}

/// Prepare the workspace before serving requests and report pending setup locally.
pub fn initialize() -> Result<(), String> {
    if let Some(path) = WORKSPACE.as_ref().map_err(Clone::clone)?.initialize()? {
        println!(
            "Conctr first login — username admin. Read the temporary password from {}",
            path.display()
        );
    }
    Ok(())
}
