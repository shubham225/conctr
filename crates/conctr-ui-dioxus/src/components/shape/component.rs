use crate::components::protocol::Protocol;
use dioxus::prelude::*;
/// Decorative protocol marker. Pair it with visible protocol text.
#[component]
pub fn Shape(protocol: Protocol) -> Element {
    rsx! { i { class: "cn-shape", "data-s": protocol.shape(), style: "--c:{protocol.color()}", aria_hidden: "true" } }
}
