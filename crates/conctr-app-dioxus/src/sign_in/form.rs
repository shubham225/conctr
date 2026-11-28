use super::{SessionState, Styles};
use conctr_api_dioxus::{LoginOutcome, current_session, first_password, login};
use conctr_ui_dioxus::ui::{
    alert::{Alert, AlertVariant},
    button::{Button, ButtonSize, ButtonVariant},
    input::InputType,
    input_group::{
        InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupButton, InputGroupButtonSize,
        InputGroupInput,
    },
};
use dioxus::prelude::*;

#[component]
pub(super) fn LoginForm() -> Element {
    let mut session = use_context::<SessionState>();
    let mut busy = use_signal(|| false);
    let mut recovering = use_signal(|| false);
    let mut setup = use_signal(|| false);
    let mut message = use_signal(|| "");
    let mut mismatch = use_signal(|| false);
    let username = use_signal(String::new);
    let mut returning_from_recovery = use_signal(|| false);
    if recovering() {
        return rsx! {
            super::recovery_form::RecoveryForm {
                username,
                on_back: move |_| {
                    returning_from_recovery.set(true);
                    recovering.set(false);
                }
            }
        };
    }
    let progress = if setup() {
        "Saving your password…"
    } else {
        "Signing in…"
    };
    rsx! {
        div { class: "grid gap-1",
            if setup() {
                h1 { tabindex: "-1",
                    onmounted: move |event| async move { let _ = event.set_focus(true).await; },
                    "Set your new password"
                }
            } else {
                h1 { tabindex: "-1",
                    onmounted: move |event| async move {
                        if returning_from_recovery() {
                            returning_from_recovery.set(false);
                            let _ = event.set_focus(true).await;
                        }
                    },
                    "Welcome back"
                }
                p { class: "text-sm text-muted-foreground", "Sign in to your Conctr workspace." }
            }
        }
        form {
            class: "grid gap-4", method: "post",
            onsubmit: move |event| {
                event.prevent_default();
                if busy() { return; }
                message.set("");
                let setting_password = setup();
                let mut fields = event.values();
                let mut field = |name: &str| match fields.iter_mut().find(|(key, _)| key == name) {
                    Some((_, dioxus::html::FormValue::Text(text))) => std::mem::take(text),
                    _ => String::new(),
                };
                let username = field("username");
                let password = field("password");
                let replacement = if setting_password {
                    let replacement = field("new-password");
                    if replacement != field("confirm-password") {
                        mismatch.set(true);
                        return;
                    }
                    Some(replacement)
                } else {
                    None
                };
                mismatch.set(false);
                busy.set(true);
                spawn(async move {
                    let result = if let Some(replacement) = replacement {
                        first_password(username,password, replacement).await.map(|_| LoginOutcome::SignedIn)
                    } else {
                        login(username, password).await
                    };
                    match result {
                        Ok(LoginOutcome::SetPassword) => setup.set(true),
                        Ok(LoginOutcome::SignedIn) => match current_session().await {
                            Ok(Some(info)) => session.set(Some(info)),
                            _ => message.set("Your session could not be restored. Check that cookies are enabled."),
                        },
                        Err(ServerFnError::ServerError { code: 403, .. }) => message.set("Open the address provided by your workspace administrator and try again."),
                        Err(ServerFnError::ServerError { code: 503, .. }) => message.set("Sign-in is temporarily unavailable. Try again or contact your workspace administrator."),
                        Err(ServerFnError::ServerError { code: 422, .. }) => message.set("Choose a different password with at least 15 characters and at most 1024 bytes."),
                        Err(ServerFnError::ServerError { code: 401, .. }) if setting_password => message.set("Refresh the page and sign in again with a valid temporary password."),
                        Err(ServerFnError::ServerError { code: 401, .. }) => message.set("Incorrect username or password."),
                        Err(ServerFnError::ServerError { code: 429, .. }) => message.set("Too many attempts. Try again in a minute."),
                        Err(_) => message.set("Could not complete sign-in. Refresh the page and try again."),
                    }
                    busy.set(false);
                });
            },
            div { hidden: setup(),
                div { class: "grid gap-4",
                    UsernameField { id: "username", username }
                    PasswordField { name: "password", label: "Password", autocomplete: "current-password" }
                }
            }
            if setup() {
                fieldset {
                    class: "grid min-w-0 gap-4",
                    aria_describedby: if mismatch() { "password-rule password-mismatch" } else { "password-rule" },
                    oninput: move |_| mismatch.set(false),
                    legend { class: "sr-only", "Choose a new password" }
                    p { id: "password-rule", class: "text-sm text-muted-foreground", "Use at least 15 characters." }
                    PasswordField { name: "new-password", label: "New password", autocomplete: "new-password" }
                    PasswordField { name: "confirm-password", label: "Confirm new password", autocomplete: "new-password" }
                    if mismatch() {
                        p { id: "password-mismatch", tabindex: "-1", class: "text-sm text-destructive", role: "alert",
                            onmounted: move |event| async move { let _ = event.set_focus(true).await; },
                            "Passwords don’t match. Re-enter the confirmation."
                        }
                    }
                }
            }
            if !message().is_empty() {
                Alert { variant: AlertVariant::Destructive, "{message}" }
            }
            Button { size: ButtonSize::Lg, class: "w-full", button_type: "submit", disabled: busy(),
                if busy() { "{progress}" } else if setup() { "Save password and sign in" } else { "Sign in" }
            }
        }
        span { role: "status", class: "sr-only", if busy() { "{progress}" } }
        if !setup() {
            Button { variant: ButtonVariant::Link, class: "px-0", disabled: busy(),
                onclick: move |_| {
                    message.set("");
                    recovering.set(true);
                },
                "Forgot password?"
            }
        }
    }
}

#[component]
pub(super) fn UsernameField(id: &'static str, mut username: Signal<String>) -> Element {
    rsx! {
        div { class: "grid gap-1",
            label { r#for: id, class: "text-sm font-medium", "Username" }
            InputGroup { class: "h-12",
                InputGroupInput {
                    class: "h-full",
                    id,
                    name: "username",
                    autocomplete: "username",
                    value: username(),
                    oninput: move |event: FormEvent| username.set(event.value()),
                    required: true,
                }
            }
        }
    }
}

#[component]
fn PasswordField(name: &'static str, label: &'static str, autocomplete: &'static str) -> Element {
    let mut visible = use_signal(|| false);
    let mut value = use_signal(String::new);
    let action = if visible() { "Hide" } else { "Show" };
    rsx! {
        div { class: "grid gap-1",
            label { r#for: name, class: "text-sm font-medium", "{label}" }
            InputGroup { class: "h-12",
                InputGroupInput {
                    class: "h-full",
                    id: name,
                    name,
                    autocomplete,
                    r#type: if visible() { InputType::Text } else { InputType::Password },
                    value: value(),
                    oninput: move |event: FormEvent| value.set(event.value()),
                    required: true,
                }
                InputGroupAddon { align: InputGroupAddonAlign::InlineEnd,
                    InputGroupButton {
                        size: InputGroupButtonSize::IconSm,
                        class: format!("{} justify-center rounded-md hover:bg-accent hover:text-accent-foreground", Styles::password_toggle.inner),
                        aria_label: format!("{action} {}", label.to_ascii_lowercase()),
                        onclick: move |_| visible.toggle(),
                        if visible() {
                            icons::EyeOff { class: "size-4" }
                        } else {
                            icons::Eye { class: "size-4" }
                        }
                    }
                }
            }
        }
    }
}
