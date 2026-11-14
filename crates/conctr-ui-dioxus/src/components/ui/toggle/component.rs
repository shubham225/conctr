use dioxus::prelude::*;
use dioxus_primitives::toggle::{self, ToggleProps};

#[css_module("/src/components/ui/toggle/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/toggle/style.css",
    manganis::AssetOptions::css_module()
);

#[component]
pub fn Toggle(props: ToggleProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        toggle::Toggle {
            class: "cn-ui-toggle cn-toggle {Styles::cn_toggle.inner} shrink-0 items-center gap-2 inline-flex justify-center whitespace-nowrap no-underline",
            pressed: props.pressed,
            default_pressed: props.default_pressed,
            disabled: props.disabled,
            on_pressed_change: props.on_pressed_change,
            onmounted: props.onmounted,
            onfocus: props.onfocus,
            onkeydown: props.onkeydown,
            attributes: props.attributes,
            {props.children}
        }
    }
}
