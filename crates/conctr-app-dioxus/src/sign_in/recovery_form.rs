use super::form::UsernameField;
use conctr_api_dioxus::forgot_password;
use conctr_ui_dioxus::ui::{
    alert::{Alert, AlertVariant},
    button::{Button, ButtonSize, ButtonVariant},
};
use dioxus::prelude::*;

#[component]
pub(super) fn RecoveryForm(username: Signal<String>, on_back: EventHandler<()>) -> Element {
    let mut busy = use_signal(|| false);
    let mut sent = use_signal(|| false);
    let mut failure = use_signal(|| "");
    rsx! {
        div { class: "grid gap-1",
            h1 { tabindex: "-1",
                onmounted: move |event| async move { let _ = event.set_focus(true).await; },
                "Reset your password"
            }
            if !sent() {
                p { class: "text-sm text-muted-foreground",
                    "Enter your workspace username to request a temporary password."
                }
            }
        }
        if sent() {
            p { class: "text-sm text-muted-foreground", tabindex: "-1",
                onmounted: move |event| async move { let _ = event.set_focus(true).await; },
                "If this account exists, your workspace administrator can provide a temporary password. Sign in with it within 15 minutes to choose a new password."
            }
        } else {
            form {
                class: "grid gap-4", method: "post",
                onsubmit: move |event| {
                    event.prevent_default();
                    if busy() { return; }
                    let Some(dioxus::html::FormValue::Text(name)) = event.get_first("username") else { return; };
                    busy.set(true);
                    failure.set("");
                    spawn(async move {
                        match forgot_password(name).await {
                            Ok(()) => sent.set(true),
                            Err(ServerFnError::ServerError { code: 429, .. }) => {
                                failure.set("Too many requests. Try again in a minute.");
                            }
                            Err(_) => failure.set("Could not request a temporary password. Try again."),
                        }
                        busy.set(false);
                    });
                },
                UsernameField { id: "recovery-user", username }
                if !failure().is_empty() {
                    Alert { variant: AlertVariant::Destructive, "{failure}" }
                }
                Button { size: ButtonSize::Lg, class: "w-full", button_type: "submit", disabled: busy(),
                    if busy() { "Requesting…" } else { "Request temporary password" }
                }
            }
        }
        span { role: "status", class: "sr-only", if busy() { "Requesting a temporary password…" } }
        Button {
            variant: if sent() { ButtonVariant::Default } else { ButtonVariant::Link },
            size: if sent() { ButtonSize::Lg } else { ButtonSize::Default },
            class: if sent() { "w-full" } else { "px-0" },
            disabled: busy(),
            onclick: move |_| on_back.call(()),
            "Back to sign in"
        }
    }
}
