use dioxus::prelude::*;
use dioxus_icons::lucide::ChevronDown;
use dioxus_primitives::accordion::{
    self, AccordionContentProps, AccordionItemProps, AccordionProps, AccordionTriggerProps,
};

#[css_module("/src/components/ui/accordion/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/accordion/style.css",
    manganis::AssetOptions::css_module()
);

#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        accordion::Accordion {
            class: "cn-ui-accordion {Styles::cn_ui_accordion.inner}",
            width: "15rem",
            id: props.id,
            allow_multiple_open: props.allow_multiple_open,
            disabled: props.disabled,
            collapsible: props.collapsible,
            horizontal: props.horizontal,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn AccordionItem(props: AccordionItemProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        accordion::AccordionItem {
            class: "cn-ui-accordion-item {Styles::cn_ui_accordion_item.inner} box-border overflow-hidden",
            disabled: props.disabled,
            default_open: props.default_open,
            on_change: props.on_change,
            on_trigger_click: props.on_trigger_click,
            index: props.index,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn AccordionTrigger(props: AccordionTriggerProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        accordion::AccordionTrigger {
            class: "cn-ui-accordion-trigger {Styles::cn_ui_accordion_trigger.inner} flex items-center bg-transparent w-full box-border flex-row justify-between p-0 pt-4 pb-4 text-left py-4",
            id: props.id,
            attributes: props.attributes,
            {props.children}
            ChevronDown {
                class: "cn-ui-accordion-expand-icon {Styles::cn_ui_accordion_expand_icon.inner}",
                size: "20px",
                stroke: "var(--cn-text)",
            }
        }
    }
}

#[component]
pub fn AccordionContent(props: AccordionContentProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        accordion::AccordionContent {
            class: "cn-ui-accordion-content {Styles::cn_ui_accordion_content.inner} grid text-muted-foreground",
            style: "--collapsible-content-width: 140px",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}
