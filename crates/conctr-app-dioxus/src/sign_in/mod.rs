//! The UI remembers only the account name and expiry. The browser or native
//! HTTP cookie jar automatically attaches the secret cookie to API requests.
use crate::Route;
mod form;
mod protocol_hub;
mod recovery_form;
use conctr_api_dioxus::{SessionInfo, current_session};
use conctr_ui_dioxus::{assets::images, ui::theme::ThemeProvider};
use dioxus::prelude::*;
use form::LoginForm;

#[css_module("/src/sign_in/style.css")]
struct Styles;
const CSS: Asset = asset!("/src/sign_in/style.css", AssetOptions::css_module());

pub(super) type SessionState = Signal<Option<SessionInfo>>;

#[component]
pub(super) fn SessionGate() -> Element {
    let mut session = use_context_provider(|| Signal::new(None::<SessionInfo>));
    let mut ready = use_signal(|| false);
    let mut failure = use_signal(|| "");
    let titlebar = use_context::<Option<Element>>();
    // Effects run on the client after hydration. The first HTML response can
    // therefore be shared safely and never contains a protected product page.
    use_effect(move || {
        spawn(async move {
            match current_session().await {
                Ok(info) => {
                    session.set(info);
                    failure.set("");
                }
                Err(ServerFnError::ServerError { code: 403, .. }) => failure.set("Open the localhost address printed by the server, or use the configured public HTTPS address."),
                Err(_) => failure.set("Session check failed. Make sure the Conctr fullstack server is running, then sign in again."),
            }
            ready.set(true);
        });
    });
    if let Some(info) = session() {
        return rsx! { SignedIn { remaining_seconds: info.remaining_seconds } };
    }
    rsx! {
        document::Stylesheet { href: CSS }
        document::Title { "Sign in · Conctr" }
        ThemeProvider {
            div { class: format!("{} flex flex-col bg-background", Styles::screen.inner),
                {titlebar}
                main { class: "flex flex-1 items-center justify-center p-4",
                    div { class: Styles::panel.inner,
                        aside { class: Styles::brand.inner, "data-theme": "dark",
                            img { src: images::HORIZONTAL_COLOR_DARK_SVG, alt: "Conctr", width: 148, height: 40 }
                            protocol_hub::ProtocolHub {}
                            p { class: Styles::tagline.inner, "Where systems meet." }
                        }
                        section { class: Styles::pane.inner,
                        if !ready() {
                            p { role: "status", "Checking your session…" }
                        } else {
                            if !failure().is_empty() { p { role: "alert", "{failure}" } }
                            LoginForm {}
                        }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SignedIn(remaining_seconds: u64) -> Element {
    let mut session = use_context::<SessionState>();
    // Unmounting this component cancels its timer. The API independently
    // checks the deadline on every request, even when this window is asleep.
    use_future(move || async move {
        #[cfg(target_arch = "wasm32")]
        gloo_timers::future::TimeoutFuture::new(
            (remaining_seconds.saturating_mul(1000)).min(u32::MAX as u64) as u32,
        )
        .await;
        #[cfg(not(target_arch = "wasm32"))]
        tokio::time::sleep(std::time::Duration::from_secs(remaining_seconds)).await;
        session.set(None);
    });
    rsx! { Router::<Route> {} }
}
