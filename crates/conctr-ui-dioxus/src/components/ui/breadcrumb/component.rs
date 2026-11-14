use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/ui/breadcrumb/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/breadcrumb/style.css",
    manganis::AssetOptions::css_module()
);

/// Styled `ol`; accepts native attributes and arbitrary children.
#[component]
pub fn Breadcrumb(
    #[props(extends = GlobalAttributes)]
    #[props(extends = ol)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(ol {
        class: "cn-breadcrumb {Styles::cn_breadcrumb.inner} flex flex-wrap gap-2 p-0 list-none"
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        document::Stylesheet { href: STYLES }
        ol { ..merged,{children} }
    }
}
