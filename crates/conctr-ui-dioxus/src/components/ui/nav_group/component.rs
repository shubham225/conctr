use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/ui/nav_group/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/nav_group/style.css",
    manganis::AssetOptions::css_module()
);

/// Styled `div`; accepts native attributes and arbitrary children.
#[component]
pub fn NavGroup(
    #[props(extends = GlobalAttributes)]
    #[props(extends = div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "cn-nav__group {Styles::cn_nav__group.inner} text-xs font-medium text-subtle-foreground"
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        document::Stylesheet { href: STYLES }
        div { ..merged,{children} }
    }
}
