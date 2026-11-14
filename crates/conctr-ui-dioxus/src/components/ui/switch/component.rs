use dioxus::prelude::*;
use dioxus_primitives::switch::{self, SwitchProps};

#[css_module("/src/components/ui/switch/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/switch/style.css",
    manganis::AssetOptions::css_module()
);

#[component]
pub fn Switch(props: SwitchProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        switch::Switch {
            class: "cn-ui-switch {Styles::cn_ui_switch.inner} cn-switch-track {Styles::cn_switch_track.inner} inline-flex",
            checked: props.checked,
            default_checked: props.default_checked,
            disabled: props.disabled,
            required: props.required,
            name: props.name,
            value: props.value,
            on_checked_change: props.on_checked_change,
            attributes: props.attributes,
            switch::SwitchThumb { class: "cn-ui-switch-thumb {Styles::cn_ui_switch_thumb.inner} cn-switch-thumb {Styles::cn_switch_thumb.inner}" }
        }
    }
}
