use crate::{composed::FeedbackMessage, ui::alert::AlertVariant};
use dioxus::prelude::*;
#[component]
pub fn FeedbackDemo() -> Element {
    rsx! {
        div { class: "grid gap-3",
            FeedbackMessage {
                variant: AlertVariant::Success,
                title: "Connection created",
                description: "Run a test to confirm both systems respond.",
            }
            FeedbackMessage {
                variant: AlertVariant::Warning,
                title: "Orders to ERP is slow",
                description: "Inspect the latest run before retrying.",
            }
            FeedbackMessage {
                variant: AlertVariant::Destructive,
                title: "Credentials rejected",
                description: "Update the credentials, then test the connection again.",
            }
        }
    }
}
