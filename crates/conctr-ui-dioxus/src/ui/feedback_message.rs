use crate::ui::alert::{Alert, AlertDescription, AlertTitle, AlertVariant};
use dioxus::prelude::*;
#[component]
pub fn FeedbackMessage(
    title: String,
    description: String,
    #[props(default)] variant: AlertVariant,
) -> Element {
    rsx! { Alert { variant, AlertTitle { "{title}" } AlertDescription { "{description}" } } }
}
