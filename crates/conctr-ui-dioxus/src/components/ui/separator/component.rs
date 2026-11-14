use dioxus::prelude::*;
use dioxus_primitives::separator::{self, SeparatorProps};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/ui/separator/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/separator/style.css",
    manganis::AssetOptions::css_module()
);

#[component]
pub fn Separator(props: SeparatorProps) -> Element {
    let base = attributes!(div {
        class: "cn-ui-separator {Styles::cn_ui_separator.inner} bg-border",
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        separator::Separator {
            horizontal: props.horizontal,
            decorative: props.decorative,
            attributes: merged,
            {props.children}
        }
    }
}
