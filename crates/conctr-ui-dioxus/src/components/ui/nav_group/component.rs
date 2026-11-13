use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

/// Styled `div`; accepts native attributes and arbitrary children.
#[component]
pub fn NavGroup(
    #[props(extends = GlobalAttributes)]
    #[props(extends = div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "cn-nav__group"
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! { div { ..merged, {children} } }
}
