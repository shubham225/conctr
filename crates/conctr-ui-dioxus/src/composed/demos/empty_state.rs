use crate::{
    composed::EmptyState,
    ui::button::{Button, ButtonVariant},
};
use dioxus::prelude::*;
#[component]
pub fn EmptyStateDemo() -> Element {
    let mut started = use_signal(|| false);
    rsx! {
        EmptyState {
            title: if started() { "Ready to configure" } else { "No connections yet" },
            description: "Choose a source and target to start moving data.",
            Button {
                variant: ButtonVariant::Outline,
                onclick: move |_| started.set(!started()),
                "Create connection"
            }
        }
    }
}
