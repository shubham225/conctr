use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

/// Styled `kbd`; accepts native attributes and arbitrary children.
#[component]
pub fn KeyboardKey(
    #[props(extends = GlobalAttributes)]
    #[props(extends = kbd)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(kbd { class: "cn-kbd" });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! { kbd { ..merged, {children} } }
}
