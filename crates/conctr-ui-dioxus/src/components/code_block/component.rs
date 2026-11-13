use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

/// Styled `pre`; accepts native attributes and arbitrary children.
#[component]
pub fn CodeBlock(
    #[props(extends = GlobalAttributes)]
    #[props(extends = pre)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(pre { class: "cn-code" });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! { pre { ..merged, {children} } }
}
