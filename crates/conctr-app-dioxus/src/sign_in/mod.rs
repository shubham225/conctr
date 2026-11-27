//! The UI remembers only the account name and expiry. The browser or native
//! HTTP cookie jar automatically attaches the secret cookie to API requests.
use crate::Route;
mod form;
mod recovery_form;
mod scenes;
use conctr_api_dioxus::{SessionInfo, current_session};
use conctr_ui_dioxus::{
    assets::images,
    composed::loading::ConctrLoader,
    ui::{
        theme::{Theme, ThemeProvider},
        toast::{ToastType, use_toast},
    },
};
use dioxus::prelude::*;
use form::LoginForm;

#[css_module("/src/sign_in/style.css")]
struct Styles;
const CSS: Asset = asset!("/src/sign_in/style.css", AssetOptions::css_module());

pub(super) type SessionState = Signal<Option<SessionInfo>>;

#[component]
pub(super) fn SessionGate() -> Element {
    let session = use_context_provider(|| Signal::new(None::<SessionInfo>));
    let ready = use_signal(|| false);
    let mut dark = use_signal(|| false);
    let titlebar = use_context::<Option<Element>>();
    if let Some(info) = session() {
        return rsx! { SignedIn { remaining_seconds: info.remaining_seconds } };
    }
    rsx! {
        document::Stylesheet { href: CSS }
        document::Title { "Sign in · Conctr" }
        ThemeProvider { theme: if dark() { Theme::Dark } else { Theme::Light },
            div { class: Styles::screen.inner,
                {titlebar}
                if !ready() {
                  SessionCheck { ready }
                } else {
                  main { class: Styles::panel.inner,
                    scenes::ScenePanel { dark: dark(), on_theme_change: move |value| dark.set(value) }
                    section { class: Styles::pane.inner,
                      div { class: Styles::form_box.inner,
                        img { class: Styles::mark.inner, alt: "", src: if dark() { images::HUB_DARK } else { images::HUB } }
                        LoginForm {}
                        TestToastButton {}
                      }
                    }
                  }
                }
            }
        }
    }
}

#[component]
fn TestToastButton() -> Element {
    let toast = use_toast();
    rsx! {
        button {
            r#type: "button",
            onclick: move |_| toast.push(
                "Session check failed",
                "Make sure the Conctr fullstack server is running, then sign in again.",
                ToastType::Error,
            ),
            "Test error toast"
        }
    }
}

#[component]
fn SessionCheck(mut ready: Signal<bool>) -> Element {
    let mut session = use_context::<SessionState>();
    let toast = use_toast();
    // Run after hydration so the initial response can be shared safely.
    use_effect(move || {
        spawn(async move {
            match current_session().await {
                Ok(info) => session.set(info),
                Err(ServerFnError::ServerError { code: 403, .. }) => toast.push(
                    "Session check blocked",
                    "Open the localhost address printed by the server, or use the configured public HTTPS address.",
                    ToastType::Warning,
                ),
                Err(_) => toast.push(
                    "Session check failed",
                    "Make sure the Conctr fullstack server is running, then sign in again.",
                    ToastType::Error,
                ),
            }
            ready.set(true);
        });
    });

    rsx! { ConctrLoader { fullscreen: true, text: "Checking your session…" } }
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
