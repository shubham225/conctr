use dioxus::prelude::*;

#[css_module("/src/components/ui/logo/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/logo/style.css",
    manganis::AssetOptions::css_module()
);

/// Supplied horizontal wordmark, automatically chosen for the current theme.
#[component]
pub fn Logo() -> Element {
    use crate::assets::images::{HORIZONTAL_COLOR_DARK_SVG, HORIZONTAL_COLOR_SVG};
    rsx! {
        document::Stylesheet { href: STYLES }

        img {
            class: "cn-logo--light",
            src: HORIZONTAL_COLOR_SVG,
            alt: "Conctr",
            height: 32,
        }
        img {
            class: "cn-logo--dark",
            src: HORIZONTAL_COLOR_DARK_SVG,
            alt: "Conctr",
            height: 32,
        }
    }
}
