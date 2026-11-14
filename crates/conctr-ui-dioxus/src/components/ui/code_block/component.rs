use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/ui/code_block/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/code_block/style.css",
    manganis::AssetOptions::css_module()
);

/// Styled `pre`; accepts native attributes and arbitrary children.
#[component]
pub fn CodeBlock(
    #[props(extends = GlobalAttributes)]
    #[props(extends = pre)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(pre {
        class: "cn-code {Styles::cn_code.inner} border border-solid border-border rounded-md m-0 bg-muted p-4 overflow-x-auto"
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        document::Stylesheet { href: STYLES }
        pre { ..merged,{children} }
    }
}
