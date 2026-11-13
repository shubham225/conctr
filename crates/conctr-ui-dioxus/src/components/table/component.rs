use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

/// Styled `table`; accepts native attributes and arbitrary children.
#[component]
pub fn Table(
    #[props(extends = GlobalAttributes)]
    #[props(extends = table)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(table { class: "cn-table" });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! { table { ..merged, {children} } }
}
