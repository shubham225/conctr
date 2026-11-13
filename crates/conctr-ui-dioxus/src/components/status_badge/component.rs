use crate::components::status::Status;
use dioxus::prelude::*;

/// A status badge always includes a readable label.
#[component]
pub fn StatusBadge(status: Status, children: Element) -> Element {
    rsx! { span { class: "cn-badge cn-badge--{status.class()}", {children} } }
}
