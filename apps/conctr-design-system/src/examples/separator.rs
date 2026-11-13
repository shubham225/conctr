use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    Stack { span { "Source configuration" } Separator { horizontal: true, decorative: false } span { "Target configuration" } Cluster { span { "REST" } Separator { horizontal: false, height: "var(--cn-space-6)" } span { "SOAP" } } }
        }
}
