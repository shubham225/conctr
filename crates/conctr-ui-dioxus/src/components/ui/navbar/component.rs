use dioxus::prelude::*;
use dioxus_icons::lucide::ChevronDown;
use dioxus_primitives::navbar::{
    self, NavbarContentProps, NavbarItemProps, NavbarNavProps, NavbarProps, NavbarTriggerProps,
};

#[css_module("/src/components/ui/navbar/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/navbar/style.css",
    manganis::AssetOptions::css_module()
);

#[component]
pub fn Navbar(props: NavbarProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        navbar::Navbar {
            class: "cn-ui-navbar {Styles::cn_ui_navbar.inner} gap-1 box-border",
            disabled: props.disabled,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn NavbarNav(props: NavbarNavProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        navbar::NavbarNav {
            class: "cn-ui-navbar-nav {Styles::cn_ui_navbar_nav.inner} relative",
            index: props.index,
            disabled: props.disabled,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn NavbarTrigger(props: NavbarTriggerProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        navbar::NavbarTrigger { class: "cn-ui-navbar-trigger {Styles::cn_ui_navbar_trigger.inner}", attributes: props.attributes,
            {props.children}
            ChevronDown {
                class: "cn-ui-navbar-expand-icon {Styles::cn_ui_navbar_expand_icon.inner}",
                size: "20px",
                stroke: "var(--cn-text)",
            }
        }
    }
}

#[component]
pub fn NavbarContent(props: NavbarContentProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        navbar::NavbarContent {
            class: "cn-ui-navbar-content {Styles::cn_ui_navbar_content.inner} bg-card rounded-md p-1 min-w-50",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn NavbarItem(props: NavbarItemProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        navbar::NavbarItem {
            class: format!("cn-ui-navbar-item {} text-sm no-underline", Styles::cn_ui_navbar_item.inner),
            index: props.index,
            value: props.value,
            disabled: props.disabled,
            new_tab: props.new_tab,
            to: props.to,
            active_class: props.active_class,
            attributes: props.attributes,
            on_select: props.on_select,
            onclick: props.onclick,
            onmounted: props.onmounted,
            {props.children}
        }
    }
}
