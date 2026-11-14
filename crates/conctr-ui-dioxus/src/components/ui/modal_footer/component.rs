use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

/// Styled `div`; accepts native attributes and arbitrary children.
#[component]
pub fn ModalFooter(
    #[props(extends = GlobalAttributes)]
    #[props(extends = div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "cn-modal__footer flex flex-wrap gap-3 justify-end mt-2 mt-6"
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        div { ..merged,{children} }
    }
}
