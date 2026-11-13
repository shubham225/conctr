use dioxus::prelude::*;

/// Loading indicator with an accessible visible label.
#[component]
pub fn Spinner(#[props(default = "Loading".to_string())] label: String) -> Element {
    rsx! { div { class: "cn-cluster", role: "status", span { class: "cn-spinner", aria_hidden: "true" } "{label}" } }
}
