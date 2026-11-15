use super::{Protocol, ProtocolBadge};
use dioxus::prelude::*;
#[component]
pub fn ConnectionNode(name: String, protocol: Protocol) -> Element {
    rsx! { div { class:"grid gap-2 rounded-md border border-border bg-card px-4 py-3", strong { "{name}" } ProtocolBadge { protocol } } }
}
