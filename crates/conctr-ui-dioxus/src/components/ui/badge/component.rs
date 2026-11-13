use dioxus::prelude::*;
use dioxus_icons::lucide::BadgeCheck;

#[css_module("/src/components/ui/badge/style.css")]
struct Styles;

#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum BadgeVariant {
    #[default]
    Neutral,
    /// Compatibility variant; the kit uses a neutral default badge.
    Primary,
    Success,
    Warning,
    Info,
    Error,
    /// Alias for Error using the kit’s danger vocabulary.
    Danger,
    Secondary,
    Destructive,
    Outline,
}

impl BadgeVariant {
    pub fn class(&self) -> &'static str {
        match self {
            BadgeVariant::Neutral => "neutral",
            BadgeVariant::Success => "success",
            BadgeVariant::Warning => "warning",
            BadgeVariant::Info => "info",
            BadgeVariant::Error | BadgeVariant::Danger => "error",
            BadgeVariant::Primary => "primary",
            BadgeVariant::Secondary => "secondary",
            BadgeVariant::Destructive => "destructive",
            BadgeVariant::Outline => "outline",
        }
    }
}

/// The props for the [`Badge`] component.
#[derive(Props, Clone, PartialEq)]
pub struct BadgeProps {
    #[props(default)]
    pub variant: BadgeVariant,

    /// Additional attributes to extend the badge element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the badge element
    pub children: Element,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    rsx! {
        BadgeElement {
            "padding": true,
            variant: props.variant,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
fn BadgeElement(props: BadgeProps) -> Element {
    rsx! {
        span {
            class: format!("{} cn-badge", Styles::dx_badge),
            "data-style": props.variant.class(),
            ..props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn VerifiedIcon() -> Element {
    rsx! {
        BadgeCheck {
            size: "12px",
            stroke: "var(--cn-text)",
        }
    }
}
