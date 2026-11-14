use dioxus::prelude::*;
use dioxus_icons::lucide::BadgeCheck;

#[css_module("/src/components/ui/badge/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/badge/style.css",
    manganis::AssetOptions::css_module()
);

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
        document::Stylesheet { href: STYLES }

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
        document::Stylesheet { href: STYLES }

        span {
            class: "cn-ui-badge {Styles::cn_ui_badge.inner} cn-badge {Styles::cn_badge.inner} w-fit items-center h-6 inline-flex py-0 px-2 whitespace-nowrap",
            "data-style": props.variant.class(),
            ..props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn VerifiedIcon() -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        BadgeCheck { size: "12px", stroke: "var(--cn-text)" }
    }
}
