use dioxus::prelude::*;
use dioxus_icons::lucide::Check;
use dioxus_primitives::checkbox::{self, CheckboxProps};

#[css_module("/src/components/ui/checkbox/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/checkbox/style.css",
    manganis::AssetOptions::css_module()
);

#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        checkbox::Checkbox {
            class: "cn-ui-checkbox {Styles::cn_ui_checkbox.inner} cn-check {Styles::cn_check.inner} p-0 flex-none m-0 appearance-none",
            checked: props.checked,
            default_checked: props.default_checked,
            required: props.required,
            disabled: props.disabled,
            name: props.name,
            value: props.value,
            on_checked_change: props.on_checked_change,
            attributes: props.attributes,
            checkbox::CheckboxIndicator { class: "cn-ui-checkbox-indicator flex items-center justify-center",
                Check { size: "1rem" }
            }
        }
    }
}
