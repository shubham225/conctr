//! Conctr semantic feedback over Rust UI's alert structure.
pub use crate::rust_ui::ui::alert::{
    AlertDescription, AlertDescriptionProps, AlertTitle, AlertTitleProps,
};
use dioxus::prelude::*;
#[derive(Clone, PartialEq, Default)]
pub enum AlertVariant {
    #[default]
    Default,
    Info,
    Success,
    Warning,
    Destructive,
}
#[component]
pub fn Alert(
    #[props(default)] variant: AlertVariant,
    #[props(default)] class: Option<String>,
    children: Element,
) -> Element {
    let background = match variant {
        AlertVariant::Default | AlertVariant::Info => "bg-info-light",
        AlertVariant::Success => "bg-success-light",
        AlertVariant::Warning => "bg-warning-light",
        AlertVariant::Destructive => "bg-destructive-light",
    };
    let class = tw_merge::tw_merge!(
        "rounded-md border-transparent px-4 py-3 text-sm text-foreground [&_h4]:font-sans [&_h4]:text-sm [&_h4]:font-semibold",
        background,
        class.as_deref().unwrap_or("")
    );
    rsx! { crate::rust_ui::ui::alert::Alert { class, {children} } }
}
