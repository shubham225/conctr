//! Mountable management API. All routes run the same origin and correlation checks.
mod problem;
mod request;
mod sessions;
mod system;
mod throttle;
mod workspace;
use axum::{
    Router,
    extract::Request,
    http::{HeaderValue, StatusCode, header},
    middleware::{self, Next},
    response::{IntoResponse, Response},
};
use problem::{Kind, Problem};
use tracing::Instrument;
pub use workspace::initialize;

/// Build the management endpoints under the shared version prefix.
pub fn router() -> Router {
    let routes = Router::new()
        .nest("/auth", sessions::router())
        .nest("/system", system::router())
        .fallback(|| async { Problem::new(Kind::Missing, "Unknown API endpoint.") })
        .layer(middleware::from_fn(handle_request));
    Router::new().nest(&crate::api_path(""), routes)
}

async fn handle_request(req: Request, next: Next) -> Response {
    let id = req
        .headers()
        .get("x-request-id")
        .filter(|v| {
            !v.is_empty()
                && v.len() <= 128
                && v.as_bytes()
                    .iter()
                    .all(|b| b.is_ascii_alphanumeric() || b"-_.".contains(b))
        })
        .cloned()
        .unwrap_or_else(|| HeaderValue::from_str(&uuid::Uuid::new_v4().to_string()).unwrap());
    let span = tracing::info_span!("management_request",request_id=?id);
    async move {
        let mut response = if request::allowed(req.headers()) {
            next.run(req).await
        } else {
            Problem::new(Kind::Origin, "Request origin denied.").into_response()
        };
        // Normalize framework rejections without exposing parser diagnostics.
        if (response.status().is_client_error() || response.status().is_server_error())
            && !response
                .headers()
                .get(header::CONTENT_TYPE)
                .is_some_and(|v| v == "application/problem+json")
        {
            let kind = match response.status() {
                StatusCode::METHOD_NOT_ALLOWED => Kind::Method,
                StatusCode::PAYLOAD_TOO_LARGE => Kind::TooLarge,
                StatusCode::UNSUPPORTED_MEDIA_TYPE => Kind::MediaType,
                s if s.is_client_error() => Kind::BadRequest,
                _ => Kind::Internal,
            };
            let allow = response.headers().get(header::ALLOW).cloned();
            response =
                Problem::new(kind, "The server could not process this request.").into_response();
            if let Some(allow) = allow {
                response.headers_mut().insert(header::ALLOW, allow);
            }
        }
        response.headers_mut().insert("x-request-id", id);
        response
            .headers_mut()
            .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
        tracing::info!(status = response.status().as_u16(), "request completed");
        response
    }
    .instrument(span)
    .await
}

// Password hashing and file operations run off the async executor. Carry the
// request span across the thread boundary so their events share its request ID.
fn run_blocking<T: Send + 'static>(
    work: impl FnOnce() -> T + Send + 'static,
) -> tokio::task::JoinHandle<T> {
    let span = tracing::Span::current();
    tokio::task::spawn_blocking(move || span.in_scope(work))
}
