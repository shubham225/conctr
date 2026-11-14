use dioxus::prelude::*;
use dioxus_primitives::context_menu::{
    self, ContextMenuContentProps, ContextMenuItemProps, ContextMenuProps, ContextMenuTriggerProps,
};

#[css_module("/src/components/ui/context_menu/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/context_menu/style.css",
    manganis::AssetOptions::css_module()
);

#[component]
pub fn ContextMenu(props: ContextMenuProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        context_menu::ContextMenu {
            disabled: props.disabled,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn ContextMenuTrigger(props: ContextMenuTriggerProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        context_menu::ContextMenuTrigger { class: "cn-ui-context-menu-trigger bg-card border border-border rounded-md text-center select-none p-5 border-dashed cursor-context-menu", attributes: props.attributes, {props.children} }
    }
}

#[component]
pub fn ContextMenuContent(props: ContextMenuContentProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        context_menu::ContextMenuContent {
            class: "cn-ui-context-menu-content {Styles::cn_ui_context_menu_content.inner} bg-card rounded-md p-1 min-w-55",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn ContextMenuItem(props: ContextMenuItemProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        context_menu::ContextMenuItem {
            class: "cn-ui-context-menu-item {Styles::cn_ui_context_menu_item.inner} flex items-center text-sm select-none",
            disabled: props.disabled,
            value: props.value,
            index: props.index,
            on_select: props.on_select,
            attributes: props.attributes,
            {props.children}
        }
    }
}
