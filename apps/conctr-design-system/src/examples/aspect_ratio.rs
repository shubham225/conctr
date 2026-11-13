use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    div { class: "ds-ratio-frame", AspectRatio { ratio: 16.0 / 9.0, div { class: "ds-ratio-content", Logo {} span { "16:9 aspect ratio" } } } }
        }
}
