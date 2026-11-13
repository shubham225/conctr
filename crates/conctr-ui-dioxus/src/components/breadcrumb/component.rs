use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

/// Styled `ol`; accepts native attributes and arbitrary children.
#[component]
pub fn Breadcrumb(
    #[props(extends = GlobalAttributes)]
    #[props(extends = ol)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(ol {
        class: "cn-breadcrumb"
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! { ol { ..merged, {children} } }
}
