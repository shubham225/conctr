use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::dropdown_menu::{
    self, DropdownMenuContentProps, DropdownMenuItemProps, DropdownMenuProps,
    DropdownMenuTriggerProps,
};
use dioxus_primitives::merge_attributes;

#[css_module("/src/components/ui/dropdown_menu/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/dropdown_menu/style.css",
    manganis::AssetOptions::css_module()
);

#[component]
pub fn DropdownMenu(props: DropdownMenuProps) -> Element {
    let base = attributes!(div {
        class: "cn-ui-dropdown-menu relative inline-block",
    });
    let merged = merge_attributes(vec![base, props.attributes.clone()]);

    rsx! {
        document::Stylesheet { href: STYLES }

        dropdown_menu::DropdownMenu {
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            disabled: props.disabled,
            roving_loop: props.roving_loop,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn DropdownMenuTrigger(props: DropdownMenuTriggerProps) -> Element {
    let base = attributes!(button {
        class: "cn-ui-dropdown-menu-trigger {Styles::cn_ui_dropdown_menu_trigger.inner} rounded-md text-base border-0 cursor-pointer",
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        dropdown_menu::DropdownMenuTrigger { r#as: props.r#as, attributes: merged, {props.children} }
    }
}

#[component]
pub fn DropdownMenuContent(props: DropdownMenuContentProps) -> Element {
    let base = attributes!(div {
        class: "cn-ui-dropdown-menu-content {Styles::cn_ui_dropdown_menu_content.inner} cn-popup {Styles::cn_popup.inner} cn-popup--bordered bg-popover text-popover-foreground absolute left-0 p-1 top-full mt-1 min-w-50",
    });
    let merged = merge_attributes(vec![base, props.attributes.clone()]);

    rsx! {
        document::Stylesheet { href: STYLES }

        dropdown_menu::DropdownMenuContent { id: props.id, attributes: merged, {props.children} }
    }
}

#[component]
pub fn DropdownMenuItem<T: Clone + PartialEq + 'static>(
    props: DropdownMenuItemProps<T>,
) -> Element {
    let base = attributes!(div {
        class: "cn-ui-dropdown-menu-item {Styles::cn_ui_dropdown_menu_item.inner} flex items-center gap-2 select-none gap-3 w-full text-left h-9",
    });
    let merged = merge_attributes(vec![base, props.attributes.clone()]);

    rsx! {
        document::Stylesheet { href: STYLES }

        dropdown_menu::DropdownMenuItem {
            disabled: props.disabled,
            value: props.value,
            index: props.index,
            on_select: props.on_select,
            attributes: merged,
            {props.children}
        }
    }
}
