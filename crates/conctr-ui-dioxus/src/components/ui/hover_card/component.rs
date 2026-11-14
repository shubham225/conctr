use dioxus::prelude::*;
use dioxus_primitives::hover_card::{
    self, HoverCardContentProps, HoverCardProps, HoverCardTriggerProps,
};

#[css_module("/src/components/ui/hover_card/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/hover_card/style.css",
    manganis::AssetOptions::css_module()
);

#[component]
pub fn HoverCard(props: HoverCardProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        hover_card::HoverCard {
            class: "cn-ui-hover-card {Styles::cn_ui_hover_card.inner} relative inline-block w-fit max-w-full",
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            disabled: props.disabled,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn HoverCardTrigger(props: HoverCardTriggerProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        hover_card::HoverCardTrigger {
            class: "cn-ui-hover-card-trigger {Styles::cn_ui_hover_card_trigger.inner} inline-block underline",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn HoverCardContent(props: HoverCardContentProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        hover_card::HoverCardContent {
            class: "cn-ui-hover-card-content {Styles::cn_ui_hover_card_content.inner} cn-popup {Styles::cn_popup.inner} cn-popup--bordered min-w-50 gap-3 text-left w-max bg-popover text-popover-foreground",
            side: props.side,
            align: props.align,
            id: props.id,
            force_mount: props.force_mount,
            attributes: props.attributes,
            {props.children}
        }
    }
}
