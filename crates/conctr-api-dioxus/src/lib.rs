//! Dioxus HTTP client for the shared Conctr management API.
use conctr_api::response::{ApiResponse, ProblemDetails};
use conctr_api::{
    api_path,
    session::{FirstPassword, Login},
};
pub use conctr_api::{
    session::{LoginOutcome, SessionInfo},
    system::SystemDetails,
};
use dioxus::{
    fullstack::{ClientRequest, ClientResponse},
    prelude::ServerFnError,
};
use serde::Serialize;
use serde::de::DeserializeOwned;

#[cfg(feature = "server")]
pub use conctr_api::http::{initialize, router};

async fn post(path: &str, body: impl Serialize) -> Result<ClientResponse, ServerFnError> {
    let response = ClientRequest::new(http::Method::POST, api_path(path), &())
        .send_json(&body)
        .await?;
    checked(response).await
}

async fn checked(response: ClientResponse) -> Result<ClientResponse, ServerFnError> {
    let code = response.status().as_u16();
    if response.status().is_success() {
        return Ok(response);
    }
    let problem: ProblemDetails = response.json().await?;
    Err(ServerFnError::ServerError {
        code,
        message: problem.detail,
        details: None,
    })
}

async fn data<T: DeserializeOwned>(path: &str, body: impl Serialize) -> Result<T, ServerFnError> {
    Ok(post(path, body).await?.json::<ApiResponse<T>>().await?.data)
}
/// Exchange a local password for a session or a first-login challenge.
pub async fn login(username: String, password: String) -> Result<LoginOutcome, ServerFnError> {
    data("/auth/login", Login { username, password }).await
}
/// Redeem the installation invitation and start the admin session.
pub async fn first_password(
    username: String,
    temporary: String,
    password: String,
) -> Result<(), ServerFnError> {
    post(
        "/auth/first-password",
        FirstPassword {
            username: Some(username),
            temporary,
            password,
        },
    )
    .await
    .map(drop)
}
/// Look up the current cookie-backed session.
pub async fn current_session() -> Result<Option<SessionInfo>, ServerFnError> {
    data("/auth/session", ()).await
}
/// Revoke the active session and clear its cookie.
pub async fn logout() -> Result<(), ServerFnError> {
    post("/auth/logout", ()).await.map(drop)
}
/// Retrieve build details from the signed-in workspace server.
pub async fn system_details() -> Result<SystemDetails, ServerFnError> {
    let response = ClientRequest::new(http::Method::GET, api_path("/system/details"), &())
        .send_empty_body()
        .await?;
    Ok(checked(response)
        .await?
        .json::<ApiResponse<SystemDetails>>()
        .await?
        .data)
}

/// Request private temporary credentials from the workspace operator.
pub async fn forgot_password(username: String) -> Result<(), ServerFnError> {
    post(
        "/auth/forgot-password",
        conctr_api::session::RecoveryRequest { username },
    )
    .await
    .map(drop)
}
