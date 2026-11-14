use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

/// Styled `div`; accepts native attributes and arbitrary children.
#[component]
pub fn Cluster(
    #[props(extends = GlobalAttributes)]
    #[props(extends = div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "cn-cluster flex flex-wrap items-center gap-3"
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        div { ..merged,{children} }
    }
}
