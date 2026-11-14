use dioxus::prelude::*;
use dioxus_primitives::radio_group::{self, RadioGroupProps, RadioItemProps};

#[css_module("/src/components/ui/radio_group/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/radio_group/style.css",
    manganis::AssetOptions::css_module()
);

#[component]
pub fn RadioGroup(props: RadioGroupProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        radio_group::RadioGroup {
            class: "cn-ui-radio-group flex gap-3 flex-col",
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            required: props.required,
            name: props.name,
            horizontal: props.horizontal,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn RadioItem(props: RadioItemProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        radio_group::RadioItem {
            class: format!("cn-ui-radio-item {} items-center gap-2 flex-row p-0", Styles::cn_ui_radio_item.inner),
            value: props.value,
            index: props.index,
            disabled: props.disabled,
            attributes: props.attributes,
            {props.children}
        }
    }
}
