//! Conctr pill badges over the unchanged Rust UI control.
pub use crate::rust_ui::ui::badge::*;
use dioxus::prelude::*;
#[allow(non_snake_case)]
pub fn Badge(mut props: BadgeProps) -> Element {
    let colors = match props.variant {
        BadgeVariant::Success => "bg-success-light text-success-dark border-transparent",
        BadgeVariant::Warning => "bg-warning-light text-warning-dark border-transparent",
        BadgeVariant::Destructive => {
            "bg-destructive-light text-destructive-dark border-transparent hover:bg-destructive-light"
        }
        BadgeVariant::Info | BadgeVariant::Default => {
            "bg-info-light text-info-dark border-transparent hover:bg-info-light"
        }
        _ => "bg-muted text-muted-foreground border-border",
    };
    props.class = Some(tw_merge::tw_merge!(
        "h-6 rounded-full gap-1.5 px-2 py-0 text-xs font-medium shadow-none",
        colors,
        props.class.as_deref().unwrap_or("")
    ));
    crate::rust_ui::ui::badge::Badge(props)
}
