use super::{Problem, sessions::principal};
use crate::{response::ApiResponse, system::SystemDetails};
use axum::{Json, Router, http::HeaderMap, routing::get};

pub(super) fn router() -> Router {
    Router::new().route("/details", get(details))
}

async fn details(headers: HeaderMap) -> Result<Json<ApiResponse<SystemDetails>>, Problem> {
    principal(&headers)?;
    Ok(Json(ApiResponse::new(SystemDetails {
        version: env!("CARGO_PKG_VERSION").into(),
        os: std::env::consts::OS.into(),
        architecture: std::env::consts::ARCH.into(),
    })))
}
