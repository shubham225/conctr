use dioxus::prelude::*;

/// A statistic; values are supplied by the application.
#[component]
pub fn Stat(label: String, value: String) -> Element {
    rsx! { div { class: "cn-stat", span { class: "cn-stat__label", "{label}" } span { class: "cn-stat__value", "{value}" } } }
}
