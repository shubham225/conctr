use super::FeedbackMessage;
use crate::ui::alert::AlertVariant;
use dioxus::prelude::*;
/// Demonstrate the shared semantic feedback treatments used by alerts and toasts.
#[component]
pub fn Feedback() -> Element {
    rsx! {
        div { class: "grid gap-3",
            FeedbackMessage {
                variant: AlertVariant::Destructive,
                title: "Stock sync failed at the target.",
                description: "The SOAP endpoint returned 401. Check the credentials for this connection, then run it again.",
            }
            FeedbackMessage {
                variant: AlertVariant::Warning,
                title: "Orders to ERP is slow.",
                description: "The target may be rate limiting. Inspect the latest run before retrying.",
            }
            FeedbackMessage {
                variant: AlertVariant::Success,
                title: "Connection created.",
                description: "Run a test to confirm both systems respond.",
            }
            FeedbackMessage {
                variant: AlertVariant::Info,
                title: "OAuth token refreshes automatically.",
                description: "You don't need to update this connection when the token expires.",
            }
        }
    }
}
