use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    ScrollArea { height: "calc(var(--cn-space-16) * 4)", tabindex: "0", aria_label: "Scrollable connection list",
        Stack { for index in 1..=20 { div { class: "ds-list-row", "Example connection {index}" } } }
    }
        }
}
