use super::{SessionState, Styles};
use conctr_api_dioxus::{LoginOutcome, current_session, first_password, login};
use conctr_ui_dioxus::ui::{
    button::Button,
    input::{Input, InputType},
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
    if recovering() {
        return rsx! {super::recovery_form::RecoveryForm { on_back:move |_|recovering.set(false) }};
    }
    rsx! {
        h1 { class: "text-2xl font-semibold", if setup() { "Set your new password" } else { "Sign in to your workspace" } }
        if setup() { p { class: "text-sm text-muted-foreground", "Choose a password of at least 15 characters for future sign-ins." } }
        form {
            class: "grid gap-4", method: "post",
            onsubmit: move |event| {
                event.prevent_default();
                if busy() { return; }
                busy.set(true);
                message.set("");
                let field = |name| match event.get_first(name) {
                    Some(dioxus::html::FormValue::Text(text)) => text,
                    _ => String::new(),
                };
                let username = field("username");
                let password = field("password");
                let replacement = field("new-password");
                if setup() && replacement != field("confirm-password") {
                    message.set("The new passwords differ. Enter them again.");
                    busy.set(false);
                    return;
                }
                spawn(async move {
                    let result = if setup() {
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
                        Err(ServerFnError::ServerError { code: 403, .. }) => message.set("This address is blocked. Open the configured public origin, or clear CONCTR_PUBLIC_ORIGIN and restart for localhost."),
                        Err(ServerFnError::ServerError { code: 503, .. }) => message.set("The server could not access its account storage. Check the account directory permissions."),
                        Err(ServerFnError::ServerError { code: 422, .. }) => message.set("Choose a different password with at least 15 characters and at most 1024 bytes."),
                        Err(ServerFnError::ServerError { code: 401, .. }) => message.set("Incorrect username or password."),
                        Err(ServerFnError::ServerError { code: 429, .. }) => message.set("Too many attempts. Try again in a minute."),
                        Err(_) => message.set("Refresh this page to load the current app, then try again. If the request fails again, check the dx serve terminal."),
                    }
                    busy.set(false);
                });
            },
            div { class: if setup() { "hidden" } else { "grid gap-1" },
                label { r#for: "username", class: "text-sm font-medium", "Username" }
                Input { class: "h-12", id: "username", name: "username", placeholder: "admin", required: true }
            }
            div { hidden: setup(),
                PasswordField { name: "password", label: "Password" }
            }
            if !setup() {
                button { r#type:"button", class:"justify-self-end text-sm font-medium text-primary hover:underline", disabled:busy(),onclick: move |_|recovering.set(true),"Forgot password?" }
            }
            if setup() {
                PasswordField { name: "new-password", label: "New password" }
                PasswordField { name: "confirm-password", label: "Confirm new password" }
            }
            if !message().is_empty() { p { role: "alert", class: "text-sm text-destructive", "{message}" } }
            Button { class: "mt-2 h-12 w-full", button_type: "submit", disabled: busy(),
                if busy() { "Signing in…" } else if setup() { "Save password and sign in" } else { "Sign in" }
            }
        }
    }
}

#[component]
fn PasswordField(name: &'static str, label: &'static str) -> Element {
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
