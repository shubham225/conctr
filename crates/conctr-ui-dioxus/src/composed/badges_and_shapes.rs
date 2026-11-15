use super::{Protocol, ProtocolBadge};
use crate::ui::badge::{Badge, BadgeVariant};
use dioxus::prelude::*;
#[component]
pub fn BadgesAndShapes() -> Element {
    rsx! { div { class:"grid gap-3", div { class:"flex flex-wrap gap-3", for protocol in [Protocol::Rest,Protocol::Soap,Protocol::OData,Protocol::GraphQl,Protocol::Grpc,Protocol::Webhook] { ProtocolBadge { protocol } } }
     div { class:"flex flex-wrap gap-3", for (label,variant) in [("Healthy",BadgeVariant::Success),("Degraded",BadgeVariant::Warning),("Failing",BadgeVariant::Destructive),("Draft",BadgeVariant::Info)] { Badge { variant, "{label}" } } } }
    }
}
