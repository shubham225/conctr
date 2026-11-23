use super::{
    Kind, Problem,
    request::{cookie, token},
    throttle,
    workspace::current_workspace,
};
use crate::session::{FirstPassword, Login};
use crate::{
    response::ApiResponse,
    session::{LoginOutcome, SessionInfo},
};
use axum::{
    Json,
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response},
};
use conctr_auth::{AuthError, PasswordLogin, SESSION_SECONDS, local_accounts::SetupError};

pub(super) fn principal(headers: &HeaderMap) -> Result<(), Problem> {
    let token = token(headers).ok_or(AuthError::InvalidSession)?;
    current_workspace()?.authority()?.principal(token)?;
    Ok(())
}

async fn login(Json(input): Json<Login>) -> Result<Response, Problem> {
    if input.username.len() > 128 {
        return Err(AuthError::InvalidCredentials.into());
    }
    let workspace = current_workspace()?;
    let permit = throttle::acquire(&input.username)?;
    let outcome = super::run_blocking(move || {
        let _permit = permit;
        workspace.login(&input.username, &input.password)
    })
    .await
    .map_err(|_| AuthError::Unavailable)??;
    Ok(match outcome {
        PasswordLogin::SetPassword => {
            Json(ApiResponse::new(LoginOutcome::SetPassword)).into_response()
        }
        PasswordLogin::Session(token) => (
            [(header::SET_COOKIE, cookie(&token, SESSION_SECONDS))],
            Json(ApiResponse::new(LoginOutcome::SignedIn)),
        )
            .into_response(),
    })
}

async fn first_password(Json(input): Json<FirstPassword>) -> Result<Response, Problem> {
    let workspace = current_workspace()?;
    let username = input.username.as_deref().unwrap_or("admin");
    if username.len() > 128 {
        return Err(AuthError::InvalidCredentials.into());
    }
    let permit = throttle::acquire(username)?;
    let token = super::run_blocking(move || {
        let _permit = permit;
        workspace.set_password(
            input.username.as_deref().unwrap_or("admin"),
            &input.temporary,
            &input.password,
        )
    })
    .await
    .map_err(|_| AuthError::Unavailable)?
    .map_err(|e| match e {
        SetupError::Invalid(message) => Problem::new(Kind::Validation, message),
        SetupError::Denied => AuthError::InvalidCredentials.into(),
        SetupError::Storage => AuthError::Unavailable.into(),
    })?;
    Ok((
        StatusCode::NO_CONTENT,
        [(header::SET_COOKIE, cookie(&token, SESSION_SECONDS))],
    )
        .into_response())
}
async fn current(headers: HeaderMap) -> Result<Json<ApiResponse<Option<SessionInfo>>>, Problem> {
    let info = if let Some(token) = token(&headers) {
        match current_workspace()?.authority()?.principal(token) {
            Ok(p) => Some(SessionInfo {
                username: p.username.into(),
                remaining_seconds: p.remaining_seconds,
            }),
            Err(AuthError::InvalidSession) => None,
            Err(e) => return Err(e.into()),
        }
    } else {
        None
    };
    Ok(Json(ApiResponse::new(info)))
}
async fn logout(headers: HeaderMap) -> Result<Response, Problem> {
    if let Some(token) = token(&headers) {
        match current_workspace()?.authority()?.logout(token) {
            Ok(()) | Err(AuthError::InvalidSession) => {}
            Err(e) => return Err(e.into()),
        }
    }
    Ok((
        StatusCode::NO_CONTENT,
        [(header::SET_COOKIE, cookie("", 0))],
    )
        .into_response())
}

async fn forgot_password(
    Json(input): Json<crate::session::RecoveryRequest>,
) -> Result<StatusCode, Problem> {
    if input.username.len() <= 128 {
        let workspace = current_workspace()?;
        let permit = throttle::acquire_recovery(&input.username)?;
        super::run_blocking(move || {
            let _permit = permit;
            workspace.request_recovery(&input.username)
        })
        .await
        .map_err(|_| AuthError::Unavailable)??;
    }
    Ok(StatusCode::NO_CONTENT)
}

pub(super) fn router() -> axum::Router {
    use axum::routing::post;
    axum::Router::new()
        .route("/login", post(login))
        .route("/forgot-password", post(forgot_password))
        .route("/first-password", post(first_password))
        .route("/session", post(current))
        .route("/logout", post(logout))
}
