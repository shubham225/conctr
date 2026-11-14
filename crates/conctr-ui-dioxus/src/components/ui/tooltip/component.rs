use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;
use dioxus_primitives::tooltip::{self, TooltipContentProps, TooltipProps, TooltipTriggerProps};

#[css_module("/src/components/ui/tooltip/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/tooltip/style.css",
    manganis::AssetOptions::css_module()
);

#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    let base = attributes!(div {
        class: "cn-ui-tooltip {Styles::cn_ui_tooltip.inner} relative inline-block w-fit max-w-full",
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        tooltip::Tooltip {
            disabled: props.disabled,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn TooltipTrigger(props: TooltipTriggerProps) -> Element {
    let base = attributes!(button {
        class: "cn-ui-tooltip-trigger {Styles::cn_ui_tooltip_trigger.inner}",
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        tooltip::TooltipTrigger { id: props.id, r#as: props.r#as, attributes: merged, {props.children} }
    }
}

#[component]
pub fn TooltipContent(props: TooltipContentProps) -> Element {
    let base = attributes!(div {
        class: "cn-ui-tooltip-content {Styles::cn_ui_tooltip_content.inner} bg-foreground text-card justify-self-start text-background text-left w-max",
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        tooltip::TooltipContent {
            id: props.id,
            side: props.side,
            align: props.align,
            attributes: merged,
            {props.children}
        }
    }
}
