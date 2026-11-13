use conctr_ui_dioxus::{components::primitives, prelude::*};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! { Stack { gap: Space::Xl, for name in ["Source", "Mapping", "Target"] { primitives::Box { padding: Space::Lg, class: "ds-layout-cell", "{name}" } } } }
}
