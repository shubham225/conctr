use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/ui/menu/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/menu/style.css",
    manganis::AssetOptions::css_module()
);

/// Styled `div`; accepts native attributes and arbitrary children.
#[component]
pub fn Menu(
    #[props(extends = GlobalAttributes)]
    #[props(extends = div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "cn-menu {Styles::cn_menu.inner} border border-solid border-border rounded-md p-1 bg-popover min-w-50 text-popover-foreground"
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        document::Stylesheet { href: STYLES }
        div { ..merged,{children} }
    }
}
