use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/composed/connection_node/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/composed/connection_node/style.css",
    manganis::AssetOptions::css_module()
);

/// Styled `div`; accepts native attributes and arbitrary children.
#[component]
pub fn ConnectionNode(
    #[props(extends = GlobalAttributes)]
    #[props(extends = div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "cn-node {Styles::cn_node.inner} grid bg-card gap-1.5"
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        document::Stylesheet { href: STYLES }
        div { ..merged,{children} }
    }
}
