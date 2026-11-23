//! Management API contracts and optional HTTP server.
/// Version segment shared by the management router and its clients.
pub const API_VERSION: &str = "v1";

/// Absolute management API path for a feature-relative route beginning with a slash.
pub fn api_path(route: &str) -> String {
    format!("/api/{API_VERSION}{route}")
}

#[cfg(feature = "server")]
pub mod http;
pub mod response;
pub mod session;
#[cfg(feature = "server")]
mod settings;
pub mod system;
