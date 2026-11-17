//! Conctr notification messages and independent trigger controls.
pub use crate::rust_ui::ui::toast::*;
use dioxus::prelude::*;

/// A notification surface shared by previews and live toasts.
/// Pass `on_dismiss` to include an accessible close control inside the surface.
#[component]
pub fn ToastMessage(
    title: String,
    description: String,
    #[props(default)] variant: ToastType,
    #[props(default)] on_dismiss: Option<EventHandler<()>>,
) -> Element {
    let (tone, alert_variant) = match variant {
        ToastType::Success => ("success", crate::ui::alert::AlertVariant::Success),
        ToastType::Error => ("danger", crate::ui::alert::AlertVariant::Destructive),
        ToastType::Warning => ("warning", crate::ui::alert::AlertVariant::Warning),
        _ => ("info", crate::ui::alert::AlertVariant::Info),
    };
    rsx! {
        div { class: "cn-toast-message", "data-tone": tone,
            span { class: "cn-toast-icon", "aria-hidden": "true",
                if variant == ToastType::Success {
                    icons::Check { class: "size-5" }
                } else if variant == ToastType::Error {
                    icons::CircleX { class: "size-5" }
                } else if variant == ToastType::Warning {
                    icons::TriangleAlert { class: "size-5" }
                } else {
                    icons::Info { class: "size-5" }
                }
            }
            div { class: "cn-toast-copy",
                crate::ui::feedback_message::FeedbackMessage { title, description, variant: alert_variant }
            }
            if let Some(on_dismiss) = on_dismiss {
                button {
                    r#type: "button",
                    class: "cn-toast-close",
                    "aria-label": "Dismiss notification",
                    onclick: move |_| on_dismiss.call(()),
                    icons::X { class: "size-4" }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
struct Notification {
    id: u64,
    title: String,
    description: String,
    variant: ToastType,
}
#[derive(Clone, Copy, PartialEq)]
struct ToastStore {
    entries: Signal<Vec<Notification>>,
    next: Signal<u64>,
}
impl ToastStore {
    fn push(mut self, title: String, description: String, variant: ToastType) {
        let id = (self.next)();
        self.next.set(id + 1);
        self.entries.write().push(Notification {
            id,
            title,
            description,
            variant,
        });
    }
}
/// One shared notification stack for all descendant triggers. ThemeProvider mounts this automatically.
#[component]
pub fn ToastProvider(children: Element) -> Element {
    let entries = use_signal(Vec::new);
    let next = use_signal(|| 0);
    let store = ToastStore { entries, next };
    use_context_provider(|| store);
    rsx! {
        {children}
        ToastViewport { store }
    }
}
#[component]
fn ToastViewport(mut store: ToastStore) -> Element {
    let entries = (store.entries)();
    rsx! {
        if !entries.is_empty() {
            div {
                class: "cn-toast-stack cn-toast-region",
                role: "region",
                "aria-label": "Notifications",
                for entry in entries {
                    div { key: "{entry.id}", class: "cn-toast-stack-item",
                        ToastMessage {
                            title: entry.title,
                            description: entry.description,
                            variant: entry.variant,
                            on_dismiss: move |_| store.entries.write().retain(|item| item.id != entry.id),
                        }
                    }
                }
            }
        }
    }
}
/// Repeated clicks enqueue independent notifications, including from different triggers.
#[allow(non_snake_case)]
pub fn ToastTrigger(props: ToastTriggerProps) -> Element {
    let local_entries = use_signal(Vec::new);
    let local_next = use_signal(|| 0);
    let shared = try_consume_context::<ToastStore>();
    let store = shared.unwrap_or(ToastStore {
        entries: local_entries,
        next: local_next,
    });
    rsx! {
        crate::ui::button::Button {
            variant: crate::ui::button::ButtonVariant::Outline,
            class: props.class.unwrap_or_default(),
            onclick: move |_| store.push(props.title.clone(), props.description.clone(), props.variant),
            {props.children}
        }
        if shared.is_none() {
            ToastViewport { store }
        }
    }
}
