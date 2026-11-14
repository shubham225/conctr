use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/ui/skeleton/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/skeleton/style.css",
    manganis::AssetOptions::css_module()
);

#[component]
pub fn Skeleton(#[props(extends=GlobalAttributes)] attributes: Vec<Attribute>) -> Element {
    let base = attributes!(div {
        class: "cn-ui-skeleton {Styles::cn_ui_skeleton.inner} overflow-hidden h-4 rounded-sm",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        div { ..merged }
    }
}
