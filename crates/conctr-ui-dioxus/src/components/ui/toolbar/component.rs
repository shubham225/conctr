use dioxus::prelude::*;
use dioxus_primitives::toolbar::{self, ToolbarButtonProps, ToolbarProps, ToolbarSeparatorProps};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/ui/toolbar/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/toolbar/style.css",
    manganis::AssetOptions::css_module()
);

#[component]
pub fn Toolbar(props: ToolbarProps) -> Element {
    let base = attributes!(div {
        class: "cn-ui-toolbar {Styles::cn_ui_toolbar.inner}",
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        toolbar::Toolbar {
            aria_label: props.aria_label,
            disabled: props.disabled,
            horizontal: props.horizontal,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn ToolbarButton(props: ToolbarButtonProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        toolbar::ToolbarButton {
            index: props.index,
            disabled: props.disabled,
            on_click: props.on_click,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn ToolbarSeparator(props: ToolbarSeparatorProps) -> Element {
    let base = attributes!(div {
        class: "cn-ui-toolbar-separator {Styles::cn_ui_toolbar_separator.inner} h-6 bg-border w-px",
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        toolbar::ToolbarSeparator {
            decorative: props.decorative,
            horizontal: props.horizontal,
            attributes: merged,
        }
    }
}

#[component]
pub fn ToolbarGroup(
    #[props(extends = GlobalAttributes)]
    #[props(extends = div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "cn-ui-toolbar-group {Styles::cn_ui_toolbar_group.inner} flex flex-row",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        div { ..merged,{children} }
    }
}
