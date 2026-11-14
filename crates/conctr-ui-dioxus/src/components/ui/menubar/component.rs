use dioxus::prelude::*;
use dioxus_primitives::menubar::{
    self, MenubarContentProps, MenubarItemProps, MenubarMenuProps, MenubarProps,
    MenubarTriggerProps,
};

#[css_module("/src/components/ui/menubar/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/menubar/style.css",
    manganis::AssetOptions::css_module()
);

#[component]
pub fn Menubar(props: MenubarProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        menubar::Menubar {
            class: "cn-ui-menubar {Styles::cn_ui_menubar.inner} flex bg-card gap-1 box-border p-1",
            disabled: props.disabled,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn MenubarMenu(props: MenubarMenuProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        menubar::MenubarMenu {
            class: "cn-ui-menubar-menu {Styles::cn_ui_menubar_menu.inner} relative",
            index: props.index,
            disabled: props.disabled,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn MenubarTrigger(props: MenubarTriggerProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        menubar::MenubarTrigger { class: "cn-ui-menubar-trigger {Styles::cn_ui_menubar_trigger.inner}", attributes: props.attributes, {props.children} }
    }
}

#[component]
pub fn MenubarContent(props: MenubarContentProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        menubar::MenubarContent {
            class: "cn-ui-menubar-content {Styles::cn_ui_menubar_content.inner} absolute left-0 bg-card rounded-md p-1 top-full min-w-50",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn MenubarItem(props: MenubarItemProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        menubar::MenubarItem {
            class: "cn-ui-menubar-item {Styles::cn_ui_menubar_item.inner} text-sm block",
            index: props.index,
            value: props.value,
            disabled: props.disabled,
            on_select: props.on_select,
            attributes: props.attributes,
            {props.children}
        }
    }
}
