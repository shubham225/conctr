use crate::composed::{Protocol, ProtocolBadge, ProtocolShape};
use dioxus::prelude::*;
#[component]
pub fn BadgesAndShapesDemo() -> Element {
    rsx! {
        div { class: "grid gap-4",
            div { class: "flex flex-wrap gap-3",
                ProtocolBadge { protocol: Protocol::Rest }
                ProtocolBadge { protocol: Protocol::Soap }
                ProtocolBadge { protocol: Protocol::OData }
            }
            div { class: "flex gap-3",
                ProtocolShape { protocol: Protocol::GraphQl }
                ProtocolShape { protocol: Protocol::Grpc }
                ProtocolShape { protocol: Protocol::Webhook }
            }
        }
    }
}
