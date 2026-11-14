use dioxus::prelude::*;

#[css_module("/src/components/primitives/divider/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/primitives/divider/style.css",
    manganis::AssetOptions::css_module()
);

/// Visual separator.
#[component]
pub fn Divider() -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }
        hr { class: "cn-divider {Styles::cn_divider.inner}" }
    }
}
