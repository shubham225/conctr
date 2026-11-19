//! Workspace credential reference directory.
use conctr_api_dioxus::{self as api, CredentialDraft, Purpose};
use conctr_ui_dioxus::ui::button::Button;
use dioxus::prelude::*;

fn label(purpose: Purpose) -> &'static str {
    match purpose {
        Purpose::InboundApiKey => "Inbound API key",
        Purpose::OutboundOAuth => "Outbound OAuth",
        Purpose::WebhookSigning => "Webhook signing",
    }
}

#[component]
pub(crate) fn Credentials() -> Element {
    let mut records = use_resource(api::list_credentials);
    let mut editing = use_signal(|| None::<(u64, u64)>);
    let mut draft = use_signal(CredentialDraft::default);
    let mut open = use_signal(|| false);
    let mut removing = use_signal(|| None::<u64>);
    let mut busy = use_signal(|| false);
    let mut message = use_signal(String::new);
    let mut code = use_signal(String::new);
    let fields = "grid gap-1 text-sm";
    let control = "rounded-md border border-border bg-background px-3 py-2";
    rsx! {
        section { class: "mx-auto grid max-w-5xl gap-5",
            h1 { class: "text-2xl font-semibold", "Credentials" }
            p { class: "text-sm text-muted-foreground",
                "Manage references to caller keys, outbound identities and webhook signing credentials."
            }
            if !message().is_empty() {
                p { role: "status", "{message}" }
            }
            match &*records.read() {
                None => rsx! {
                    p { "Loading credential references…" }
                },
                Some(Err(error)) => rsx! {
                    if let ServerFnError::ServerError { message: reason, code: 401, .. } = error {
                        p { "{reason}" }
                        form {
                            class: "grid max-w-md gap-3",
                            onsubmit: move |event| {
                                event.prevent_default();
                                async move {
                                    if busy() {
                                        return;
                                    }
                                    busy.set(true);
                                    let submitted = code();
                                    code.set(String::new());
                                    match api::local_session(submitted).await {
                                        Ok(()) => {
                                            message.set(String::new());
                                            records.restart();
                                        }
                                        Err(e) => message.set(e.to_string()),
                                    }
                                    busy.set(false);
                                }
                            },
                            label { class: fields,
                                "Local sign-in code"
                                input {
                                    class: control,
                                    r#type: "password",
                                    required: true,
                                    value: "{code}",
                                    oninput: move |e| code.set(e.value()),
                                }
                            }
                            button {
                                class: "rounded-md bg-primary px-4 py-2 text-primary-foreground",
                                r#type: "submit",
                                disabled: busy(),
                                "Sign in"
                            }
                        }
                    } else {
                        p { role: "alert", "{error}" }
                    }
                    Button { onclick: move |_| records.restart(), "Retry" }
                },
                Some(Ok(rows)) => rsx! {
                    div { class: "flex gap-2",
                        Button {
                            disabled: busy() || open(),
                            onclick: move |_| {
                                draft.set(Default::default());
                                editing.set(None);
                                removing.set(None);
                                open.set(true);
                            },
                            "Add reference"
                        }
                        Button { disabled: busy(), onclick: move |_| records.restart(), "Refresh" }
                    }
                    if rows.is_empty() {
                        p { "No credential references yet." }
                    }
                    for row in rows.iter().cloned() {
                        {
                            let id = row.id;
                            let revision = row.revision;
                            let metadata = row.metadata.clone();
                            rsx! {
                                article { key: "{id}", class: "grid gap-2 rounded-lg border border-border p-4",
                                    h2 { class: "font-semibold", "{row.metadata.name}" }
                                    p { "{label(row.metadata.purpose)} · {row.metadata.alias}" }
                                    div { class: "flex gap-2",
                                        Button {
                                            disabled: busy() || open(),
                                            onclick: move |_| {
                                                draft.set(metadata.clone());
                                                editing.set(Some((id, revision)));
                                                removing.set(None);
                                                open.set(true);
                                            },
                                            "Edit"
                                        }
                                        if removing() == Some(id) {
                                            Button {
                                                disabled: busy(),
                                                onclick: move |_| async move {
                                                    if busy() {
                                                        return;
                                                    }
                                                    busy.set(true);
                                                    match api::delete_credentials(id, revision).await {
                                                        Ok(_) => {
                                                            removing.set(None);
                                                            message.set("Reference deleted.".into());
                                                            records.restart();
                                                        }
                                                        Err(e) => message.set(e.to_string()),
                                                    }
                                                    busy.set(false);
                                                },
                                                "Confirm deletion"
                                            }
                                            Button { disabled: busy(), onclick: move |_| removing.set(None), "Cancel" }
                                        } else {
                                            Button {
                                                disabled: busy() || open(),
                                                onclick: move |_| removing.set(Some(id)),
                                                "Delete"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
            }
            if open() {
                form {
                    class: "grid gap-3 rounded-lg border border-border p-4",
                    onsubmit: move |e| {
                        e.prevent_default();
                        async move {
                            if busy() {
                                return;
                            }
                            busy.set(true);
                            let result = match editing() {
                                Some((id, v)) => api::update_credentials(id, v, draft()).await,
                                None => api::create_credentials(draft()).await,
                            };
                            match result {
                                Ok(_) => {
                                    open.set(false);
                                    message.set("Reference saved.".into());
                                    records.restart();
                                }
                                Err(e) => message.set(e.to_string()),
                            }
                            busy.set(false);
                        }
                    },
                    label { class: fields,
                        "Display name"
                        input {
                            class: control,
                            required: true,
                            maxlength: 100,
                            value: "{draft.read().name}",
                            oninput: move |e| draft.write().name = e.value(),
                        }
                    }
                    label { class: fields,
                        "Purpose"
                        select {
                            class: control,
                            value: label(draft.read().purpose),
                            onchange: move |e| {
                                draft.write().purpose = match e.value().as_str() {
                                    "Outbound OAuth" => Purpose::OutboundOAuth,
                                    "Webhook signing" => Purpose::WebhookSigning,
                                    _ => Purpose::InboundApiKey,
                                };
                            },
                            for p in [Purpose::InboundApiKey, Purpose::OutboundOAuth, Purpose::WebhookSigning] {
                                option { value: label(p), "{label(p)}" }
                            }
                        }
                    }
                    label { class: fields,
                        "Vault or key alias"
                        input {
                            class: control,
                            required: true,
                            maxlength: 128,
                            value: "{draft.read().alias}",
                            oninput: move |e| draft.write().alias = e.value(),
                        }
                    }
                    div { class: "flex gap-2",
                        button {
                            class: "rounded-md bg-primary px-4 py-2 text-primary-foreground",
                            r#type: "submit",
                            disabled: busy(),
                            "Save reference"
                        }
                        button {
                            r#type: "button",
                            disabled: busy(),
                            onclick: move |_| open.set(false),
                            "Cancel"
                        }
                    }
                }
            }
        }
    }
}
