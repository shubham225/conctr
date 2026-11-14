use crate::components::primitives::{Align, Space};
use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

/// Styled `div`; accepts native attributes and arbitrary children.
#[component]
pub fn Stack(
    #[props(default)] gap: Space,
    #[props(default)] align: Align,
    #[props(extends = GlobalAttributes)]
    #[props(extends = div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "cn-stack min-w-0 flex flex-col gap-4",
        style: format!("gap: {}; align-items: {};", gap.token(), align.css())
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        div { ..merged,{children} }
    }
}
