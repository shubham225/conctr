use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/ui/table_container/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/table_container/style.css",
    manganis::AssetOptions::css_module()
);

/// Styled `div`; accepts native attributes and arbitrary children.
#[component]
pub fn TableContainer(
    #[props(extends = GlobalAttributes)]
    #[props(extends = div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "cn-table-wrap {Styles::cn_table_wrap.inner} bg-card border border-solid border-border rounded-lg text-card-foreground"
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        document::Stylesheet { href: STYLES }
        div { ..merged,{children} }
    }
}
