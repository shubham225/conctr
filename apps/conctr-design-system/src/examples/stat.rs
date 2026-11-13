use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    div { class: "ds-grid", Card { Stat { label: "Example runs today", value: "128" } } Card { Stat { label: "Example success rate", value: "99.2%" } } } Hint { "Illustrative values, supplied by the application." }
        }
}
