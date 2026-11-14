use crate::components::ui::protocol::Protocol;
use dioxus::prelude::*;

#[css_module("/src/components/ui/badge/style.css")]
struct BadgeStyles;

// Register for every document, including repeated SSR requests.
const BADGE_STYLES: Asset = asset!(
    "/src/components/ui/badge/style.css",
    manganis::AssetOptions::css_module()
);
#[css_module("/src/components/ui/shape/style.css")]
struct ShapeStyles;

// Register for every document, including repeated SSR requests.
const SHAPE_STYLES: Asset = asset!(
    "/src/components/ui/shape/style.css",
    manganis::AssetOptions::css_module()
);

/// Protocol identified by both shape and text.
#[component]
pub fn ProtocolBadge(protocol: Protocol) -> Element {
    rsx! {
        document::Stylesheet { href: BADGE_STYLES }
        document::Stylesheet { href: SHAPE_STYLES }
        span { class: "cn-badge {BadgeStyles::cn_badge.inner} items-center h-6 inline-flex py-0 px-2 whitespace-nowrap",
            i {
                class: "cn-shape {ShapeStyles::cn_shape.inner} flex-none h-2.5 w-2.5",
                "data-s": protocol.shape(),
                style: "--c:{protocol.color()}",
                aria_hidden: "true",
            }
            "{protocol.label()}"
        }
    }
}
