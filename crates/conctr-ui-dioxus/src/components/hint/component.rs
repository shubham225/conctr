use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

/// Styled `span`; accepts native attributes and arbitrary children.
#[component]
pub fn Hint(
    #[props(extends = GlobalAttributes)]
    #[props(extends = span)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(span { class: "cn-hint" });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! { span { ..merged, {children} } }
}
