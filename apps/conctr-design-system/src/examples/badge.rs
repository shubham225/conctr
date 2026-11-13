use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    Stack {
        Cluster { for protocol in [Protocol::Rest, Protocol::Soap, Protocol::OData, Protocol::GraphQl, Protocol::Grpc, Protocol::Webhook] { ProtocolBadge { protocol } } }
        Cluster { Badge { variant: BadgeVariant::Success, "Healthy" } Badge { variant: BadgeVariant::Warning, "Degraded" } Badge { variant: BadgeVariant::Error, "Failing" } Badge { variant: BadgeVariant::Info, "Draft" } Badge { "Paused" } Badge { variant: BadgeVariant::Secondary, "Disabled" } }
        Cluster { for protocol in [Protocol::Rest, Protocol::Soap, Protocol::OData, Protocol::GraphQl, Protocol::Grpc, Protocol::Webhook] { Shape { protocol } } }
    }
        }
}
