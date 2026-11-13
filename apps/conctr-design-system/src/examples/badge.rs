use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    Stack {
        Cluster { for protocol in [Protocol::Rest, Protocol::Soap, Protocol::OData, Protocol::GraphQl, Protocol::Grpc, Protocol::Webhook] { ProtocolBadge { protocol } } }
        Cluster { StatusBadge { status: Status::Success, "Healthy" } StatusBadge { status: Status::Warning, "Degraded" } StatusBadge { status: Status::Danger, "Failing" } StatusBadge { status: Status::Info, "Draft" } Badge { "Paused" } }
        Cluster { for protocol in [Protocol::Rest, Protocol::Soap, Protocol::OData, Protocol::GraphQl, Protocol::Grpc, Protocol::Webhook] { Shape { protocol } } }
    }
        }
}
