use crate::response::ProblemDetails;
use axum::{
    Json,
    http::{HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
};
use conctr_auth::AuthError;

#[derive(Clone, Copy)]
pub(super) enum Kind {
    BadRequest,
    InvalidCredentials,
    Session,
    Origin,
    Missing,
    Method,
    Validation,
    RateLimited,
    Unavailable,
    Internal,
    TooLarge,
    MediaType,
}
impl Kind {
    fn properties(self) -> (StatusCode, &'static str, &'static str) {
        use StatusCode as S;
        match self {
            Self::BadRequest => (S::BAD_REQUEST, "INVALID_REQUEST", "Invalid request"),
            Self::InvalidCredentials => (
                S::UNAUTHORIZED,
                "INVALID_CREDENTIALS",
                "Invalid credentials",
            ),
            Self::Session => (S::UNAUTHORIZED, "SESSION_REQUIRED", "Session required"),
            Self::Origin => (S::FORBIDDEN, "ORIGIN_DENIED", "Request origin denied"),
            Self::Missing => (S::NOT_FOUND, "NOT_FOUND", "Resource not found"),
            Self::Method => (
                S::METHOD_NOT_ALLOWED,
                "METHOD_NOT_ALLOWED",
                "Method not allowed",
            ),
            Self::Validation => (
                S::UNPROCESSABLE_ENTITY,
                "VALIDATION_FAILED",
                "Validation failed",
            ),
            Self::RateLimited => (
                S::TOO_MANY_REQUESTS,
                "LOGIN_RATE_LIMITED",
                "Too many login attempts",
            ),
            Self::Unavailable => (
                S::SERVICE_UNAVAILABLE,
                "SERVICE_UNAVAILABLE",
                "Service unavailable",
            ),
            Self::Internal => (
                S::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                "Internal server error",
            ),
            Self::TooLarge => (S::PAYLOAD_TOO_LARGE, "BODY_TOO_LARGE", "Request too large"),
            Self::MediaType => (
                S::UNSUPPORTED_MEDIA_TYPE,
                "UNSUPPORTED_MEDIA_TYPE",
                "Unsupported media type",
            ),
        }
    }
}
pub(super) struct Problem {
    kind: Kind,
    errors: Vec<crate::response::FieldError>,
    detail: String,
}
impl Problem {
    pub(super) fn new(kind: Kind, detail: &'static str) -> Self {
        Self {
            kind,
            detail: detail.into(),
            errors: Vec::new(),
        }
    }
}
impl From<AuthError> for Problem {
    fn from(e: AuthError) -> Self {
        match e {
            AuthError::InvalidCredentials => {
                Self::new(Kind::InvalidCredentials, "Incorrect username or password.")
            }
            AuthError::InvalidSession => Self::new(Kind::Session, "Sign in to continue."),
            AuthError::Unavailable => {
                Self::new(Kind::Unavailable, "Authentication service unavailable.")
            }
        }
    }
}
impl IntoResponse for Problem {
    fn into_response(self) -> Response {
        let (status, code, title) = self.kind.properties();
        let mut response = (
            status,
            Json(ProblemDetails {
                problem_type: format!("/problems/{}", code.to_ascii_lowercase().replace('_', "-")),
                title: title.into(),
                status: status.as_u16(),
                detail: self.detail,
                code: code.into(),
                errors: self.errors,
            }),
        )
            .into_response();
        response.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/problem+json"),
        );
        if matches!(self.kind, Kind::RateLimited) {
            response
                .headers_mut()
                .insert(header::RETRY_AFTER, HeaderValue::from_static("60"));
        }
        response
    }
}
