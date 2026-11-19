//! This crate contains all shared fullstack server functions.
use dioxus::prelude::*;

/// Echo the user input on the server.
#[post("/api/echo")]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}

pub use conctr_api::{Credential, CredentialDraft, Purpose};

/// List credential references in the authorized workspace.
#[post("/api/credentials/list")]
pub async fn list_credentials() -> Result<Vec<Credential>, ServerFnError> {
    dispatch(conctr_api::directory::Operation::List).await
}

/// Create credential references in the authorized workspace.
#[post("/api/credentials/create")]
pub async fn create_credentials(draft: CredentialDraft) -> Result<Vec<Credential>, ServerFnError> {
    dispatch(conctr_api::directory::Operation::Create(draft)).await
}

/// Update credential references in the authorized workspace.
#[post("/api/credentials/update")]
pub async fn update_credentials(
    id: u64,
    revision: u64,
    draft: CredentialDraft,
) -> Result<Vec<Credential>, ServerFnError> {
    dispatch(conctr_api::directory::Operation::Update(
        id, revision, draft,
    ))
    .await
}

/// Delete credential references in the authorized workspace.
#[post("/api/credentials/delete")]
pub async fn delete_credentials(id: u64, revision: u64) -> Result<Vec<Credential>, ServerFnError> {
    dispatch(conctr_api::directory::Operation::Delete(id, revision)).await
}

#[cfg(feature = "server")]
async fn dispatch(op: conctr_api::directory::Operation) -> Result<Vec<Credential>, ServerFnError> {
    let directory = authorized_directory()?;
    tokio::task::spawn_blocking(move || {
        conctr_api::directory::execute(std::path::Path::new(&directory), op)
    })
    .await
    .map_err(|_| ServerFnError::new("Directory unavailable."))?
    .map_err(|error| match error {
        conctr_api::directory::DirectoryError::Invalid(message) => response_error(422, message),
        conctr_api::directory::DirectoryError::Conflict(message) => response_error(409, message),
        conctr_api::directory::DirectoryError::Unavailable => {
            response_error(503, "Credential directory unavailable.")
        }
    })
}

#[cfg(feature = "server")]
fn response_error(code: u16, message: &str) -> ServerFnError {
    ServerFnError::ServerError {
        code,
        message: message.into(),
        details: None,
    }
}

#[cfg(feature = "server")]
fn authorized_directory() -> Result<std::path::PathBuf, ServerFnError> {
    use subtle::ConstantTimeEq;
    let denied = || response_error(403, "Credential management access required.");
    #[cfg(debug_assertions)]
    if local::enabled() {
        return local::directory();
    }
    let expected = std::env::var("CONCTR_CREDENTIAL_ADMIN_TOKEN").map_err(|_| denied())?;
    let context = dioxus_fullstack_core::FullstackContext::current().ok_or_else(denied)?;
    let authorized = context
        .parts_mut()
        .headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .is_some_and(|value| {
            !expected.is_empty() && bool::from(value.as_bytes().ct_eq(expected.as_bytes()))
        });
    if !authorized {
        return Err(denied());
    }
    let directory = std::env::var_os("CONCTR_CREDENTIAL_DIRECTORY")
        .filter(|v| !v.is_empty())
        .ok_or_else(|| ServerFnError::new("Configure the workspace credential directory."))?;
    Ok(directory.into())
}

#[cfg(all(feature = "server", debug_assertions))]
mod local;

/// Authenticate a local development browser using the private launch code.
#[post("/api/credentials/local-session")]
pub async fn local_session(code: String) -> Result<(), ServerFnError> {
    #[cfg(debug_assertions)]
    if local::enabled() {
        return local::sign_in(&code);
    }
    let _ = code;
    Err(response_error(403, "Local sign-in is disabled."))
}
