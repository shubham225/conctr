use dioxus::prelude::*;

/// Visual separator.
#[component]
pub fn Divider() -> Element {
    rsx! { hr { class: "cn-divider" } }
}
