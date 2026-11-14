use dioxus::prelude::*;
use dioxus_primitives::popover::{
    self, PopoverContentProps, PopoverRootProps, PopoverTriggerProps,
};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/ui/popover/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/popover/style.css",
    manganis::AssetOptions::css_module()
);

#[component]
pub fn PopoverRoot(props: PopoverRootProps) -> Element {
    let base = attributes!(div {
        class: "cn-ui-popover {Styles::cn_ui_popover.inner} relative inline-block w-fit max-w-full"
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        popover::PopoverRoot {
            is_modal: props.is_modal,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn PopoverTrigger(props: PopoverTriggerProps) -> Element {
    let base = attributes!(button {
        class: "cn-ui-popover-trigger {Styles::cn_ui_popover_trigger.inner} border border-solid border-border rounded-md text-foreground cursor-pointer h-(--cn-control-md)"
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        popover::PopoverTrigger { attributes: merged, {props.children} }
    }
}

#[component]
pub fn PopoverContent(props: PopoverContentProps) -> Element {
    let class = format!(
        "cn-ui-popover-content {} cn-popup {} cn-popup--bordered flex-col box-border min-w-50 gap-3 w-max bg-popover text-popover-foreground {}",
        Styles::cn_ui_popover_content.inner,
        Styles::cn_popup.inner,
        props.class.unwrap_or_default(),
    );

    rsx! {
        document::Stylesheet { href: STYLES }

        popover::PopoverContent {
            class,
            id: props.id,
            side: props.side,
            align: props.align,
            attributes: props.attributes,
            {props.children}
        }
    }
}
