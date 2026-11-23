//! Shared product UI and routing for Conctr launchers.
mod dashboard;
mod routes;
mod shell;
mod sign_in;
use shell::Shell;

/// Host the management router alongside the server-rendered product app.
#[cfg(feature = "server")]
pub fn serve_workspace() -> Result<(), String> {
    use dioxus::server::{DioxusRouterExt, ServeConfig};
    conctr_api_dioxus::initialize()?;
    dioxus::serve(|| async {
        Ok(conctr_api_dioxus::router().merge(
            dioxus::server::axum::Router::new()
                .serve_dioxus_application(ServeConfig::default(), App),
        ))
    })
}
pub use routes::Route;

use dioxus::prelude::*;

/// Call before launching a native renderer. Dioxus installs its local development
/// URL during launch. Packaged clients should send passwords only to an explicit
/// HTTPS endpoint chosen when building the app.
pub fn configure_native_api() -> Result<(), &'static str> {
    match option_env!("CONCTR_NATIVE_API_ORIGIN") {
        Some(origin) if origin.starts_with("https://") => {
            dioxus::fullstack::set_server_url(origin);
            Ok(())
        }
        None if cfg!(debug_assertions) => Ok(()),
        _ => Err("Build native clients with an HTTPS CONCTR_NATIVE_API_ORIGIN."),
    }
}

/// Shared application. Native launchers may supply their own window title bar.
#[component]
pub fn App() -> Element {
    rsx! { AppWithTitlebar {} }
}

/// Shared application with an optional platform-provided title bar.
#[component]
pub fn AppWithTitlebar(#[props(default)] window_titlebar: Option<Element>) -> Element {
    use_context_provider(|| window_titlebar);
    rsx! { sign_in::SessionGate {} }
}
