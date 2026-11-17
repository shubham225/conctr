use conctr_ui_dioxus::ui::toast::{ToastMessage, ToastTrigger, ToastType};
use dioxus::prelude::*;

#[component]
pub fn ToastDemo() -> Element {
    rsx! {
        div { class: "cn-toast-demo",
            section {
                class: "cn-toast-controls",
                "aria-label": "Notification controls",
                span { class: "cn-data-demo-label", "Interactive demo" }
                h2 { "Keep people informed." }
                p { "Confirm what happened, then give a clear next step." }
                div { class: "cn-toast-buttons",
                    ToastTrigger {
                        variant: ToastType::Success,
                        title: "Connection created",
                        description: "Run a test to confirm both systems respond.",
                        icons::Check { class: "size-4" }
                        "Show success feedback"
                    }
                    ToastTrigger {
                        variant: ToastType::Error,
                        title: "Stock sync failed",
                        description: "The target returned 401. Check the credentials, then run it again.",
                        icons::CircleX { class: "size-4" }
                        "Show error feedback"
                    }
                    ToastTrigger {
                        variant: ToastType::Warning,
                        title: "Orders to ERP is slow",
                        description: "The target may be rate limiting. Inspect the latest run before retrying.",
                        icons::TriangleAlert { class: "size-4" }
                        "Show warning feedback"
                    }
                }
            }
            section { class: "cn-toast-previews", "aria-label": "Message previews",
                div { class: "cn-toast-preview-label",
                    span { "Message previews" }
                    span { "Light & dark" }
                }
                ToastMessage {
                    variant: ToastType::Success,
                    title: "Connection created",
                    description: "Run a test to confirm both systems respond.",
                }
                ToastMessage {
                    variant: ToastType::Error,
                    title: "Stock sync failed",
                    description: "The target returned 401. Check the credentials, then run it again.",
                }
                ToastMessage {
                    variant: ToastType::Warning,
                    title: "Orders to ERP is slow",
                    description: "The target may be rate limiting. Inspect the latest run before retrying.",
                }
            }
        }
    }
}
