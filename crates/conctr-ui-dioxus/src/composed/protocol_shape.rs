use super::Protocol;
use dioxus::prelude::*;
/// Keep protocol identity recognizable without relying on color alone.
#[component]
pub fn ProtocolShape(protocol: Protocol) -> Element {
    let (shape, color) = match protocol {
        Protocol::Rest => ("rounded-full", "var(--cn-teal)"),
        Protocol::Soap => ("rounded-[2px]", "var(--cn-coral)"),
        Protocol::OData => (
            "[clip-path:polygon(50%_0,100%_100%,0_100%)]",
            "var(--cn-amber)",
        ),
        Protocol::GraphQl => (
            "[clip-path:polygon(50%_0,100%_50%,50%_100%,0_50%)]",
            "var(--cn-teal)",
        ),
        Protocol::Grpc => (
            "[clip-path:polygon(50%_0,100%_25%,100%_75%,50%_100%,0_75%,0_25%)]",
            "var(--cn-primary)",
        ),
        Protocol::Webhook => ("rounded-full border-2", "var(--cn-coral)"),
    };
    rsx! {
        i {
            "aria-hidden": "true",
            class: "inline-block size-[10px] shrink-0 {shape}",
            style: if protocol == Protocol::Webhook { format!("border-color:{color}") } else { format!("background:{color}") },
        }
    }
}
