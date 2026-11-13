use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    Field { Label { html_for: "label-example", "Connection name" } Input { id: "label-example", placeholder: "Order sync" } Hint { "Select the label to focus the input." } }
        }
}
