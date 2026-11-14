use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/ui/table/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/table/style.css",
    manganis::AssetOptions::css_module()
);

/// Styled `table`; accepts native attributes and arbitrary children.
#[component]
pub fn Table(
    #[props(extends = GlobalAttributes)]
    #[props(extends = table)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(table {
        class: "cn-table {Styles::cn_table.inner} w-full"
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        document::Stylesheet { href: STYLES }
        table { ..merged,{children} }
    }
}
