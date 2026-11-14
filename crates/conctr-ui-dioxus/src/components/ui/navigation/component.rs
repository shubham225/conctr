use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

/// Styled `nav`; accepts native attributes and arbitrary children.
#[component]
pub fn Navigation(
    #[props(extends = GlobalAttributes)]
    #[props(extends = nav)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(nav {
        class: "cn-nav grid gap-0.5 w-60"
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        nav { ..merged,{children} }
    }
}
