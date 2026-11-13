use conctr_ui_dioxus::{components::primitives, prelude::*};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! { Flex { justify: Justify::Between, align: Align::Center, wrap: true, primitives::Box { "Connection settings" } Button { variant: ButtonVariant::Secondary, "Edit connection" } } }
}
