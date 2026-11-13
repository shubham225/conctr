use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    CodeBlock { span { class: "c", "// Example request\n" } span { class: "k", "GET" } " /api/connections\n" span { class: "s", "Accept: application/json" } }
        }
}
