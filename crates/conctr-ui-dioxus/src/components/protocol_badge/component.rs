use crate::components::protocol::Protocol;
use dioxus::prelude::*;

/// Protocol identified by both shape and text.
#[component]
pub fn ProtocolBadge(protocol: Protocol) -> Element {
    rsx! { span { class: "cn-badge", i { class: "cn-shape", "data-s": protocol.shape(), style: "--c:{protocol.color()}", aria_hidden: "true" } "{protocol.label()}" } }
}
