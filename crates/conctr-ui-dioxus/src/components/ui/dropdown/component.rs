use dioxus::prelude::*;

/// Native disclosure menu. Browser handles focus and keyboard activation.
#[component]
pub fn Dropdown(label: String, children: Element) -> Element {
    rsx! { details { summary { class: "cn-btn cn-btn--secondary", "{label}" } div { class: "cn-menu", {children} } } }
}
