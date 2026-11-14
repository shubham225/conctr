use crate::components::ui::protocol::Protocol;
use dioxus::prelude::*;

#[css_module("/src/components/ui/shape/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/shape/style.css",
    manganis::AssetOptions::css_module()
);

/// Decorative protocol marker. Pair it with visible protocol text.
#[component]
pub fn Shape(protocol: Protocol) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }
        i {
            class: "cn-shape {Styles::cn_shape.inner} flex-none h-2.5 w-2.5",
            "data-s": protocol.shape(),
            style: "--c:{protocol.color()}",
            aria_hidden: "true",
        }
    }
}
