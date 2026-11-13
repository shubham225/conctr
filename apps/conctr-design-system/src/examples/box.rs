use conctr_ui_dioxus::{components::primitives, prelude::*};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! { primitives::Box { padding: Space::Xl, class: "ds-layout-cell", "A padded surface using space-6." } }
}
