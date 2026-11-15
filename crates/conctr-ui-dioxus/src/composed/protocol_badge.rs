use super::{Protocol, ProtocolShape};
use crate::ui::badge::{Badge, BadgeVariant};
use dioxus::prelude::*;
#[component]
pub fn ProtocolBadge(protocol: Protocol, #[props(default)] label: Option<String>) -> Element {
    rsx! { Badge { variant: BadgeVariant::Secondary, ProtocolShape { protocol } {label.unwrap_or_else(|| protocol.label().into())} } }
}
