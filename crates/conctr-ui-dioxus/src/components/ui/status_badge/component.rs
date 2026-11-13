use crate::components::ui::{
    badge::{Badge, BadgeVariant},
    status::Status,
};
use dioxus::prelude::*;

/// Compatibility adapter. New code can use `Badge` with a semantic variant.
#[component]
pub fn StatusBadge(status: Status, children: Element) -> Element {
    let variant = match status {
        Status::Success => BadgeVariant::Success,
        Status::Warning => BadgeVariant::Warning,
        Status::Danger => BadgeVariant::Error,
        Status::Info => BadgeVariant::Info,
    };
    rsx! { Badge { variant, {children} } }
}
