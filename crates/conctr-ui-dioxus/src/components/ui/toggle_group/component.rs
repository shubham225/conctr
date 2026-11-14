use dioxus::prelude::*;
use dioxus_primitives::toggle_group::{self, ToggleGroupProps, ToggleItemProps};

#[css_module("/src/components/ui/toggle_group/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/toggle_group/style.css",
    manganis::AssetOptions::css_module()
);
#[css_module("/src/components/ui/toggle/style.css")]
struct ToggleStyles;

// Register for every document, including repeated SSR requests.
const TOGGLE_STYLES: Asset = asset!(
    "/src/components/ui/toggle/style.css",
    manganis::AssetOptions::css_module()
);

#[component]
pub fn ToggleGroup(props: ToggleGroupProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }
        document::Stylesheet { href: TOGGLE_STYLES }

        toggle_group::ToggleGroup {
            class: "cn-ui-toggle-group {Styles::cn_ui_toggle_group.inner} flex-wrap gap-1 inline-flex w-fit",
            default_pressed: props.default_pressed,
            pressed: props.pressed,
            on_pressed_change: props.on_pressed_change,
            disabled: props.disabled,
            allow_multiple_pressed: props.allow_multiple_pressed,
            horizontal: props.horizontal,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn ToggleItem(props: ToggleItemProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }
        document::Stylesheet { href: TOGGLE_STYLES }

        toggle_group::ToggleItem {
            class: "cn-ui-toggle-item cn-toggle {ToggleStyles::cn_toggle.inner} shrink-0 items-center gap-2 inline-flex justify-center whitespace-nowrap no-underline",
            index: props.index,
            disabled: props.disabled,
            attributes: props.attributes,
            {props.children}
        }
    }
}
