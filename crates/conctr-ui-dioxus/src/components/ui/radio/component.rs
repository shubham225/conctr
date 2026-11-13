use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

/// Labelled native radio control with browser keyboard behavior.
#[component]
pub fn Radio(
    label: String,
    #[props(extends = GlobalAttributes)]
    #[props(extends = input)]
    attributes: Vec<Attribute>,
    onchange: Option<EventHandler<FormEvent>>,
) -> Element {
    let base = attributes!(input {
        class: "cn-radio",
        r#type: "radio"
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! { label { class: "cn-option", input { onchange: move |e| { if let Some(f) = onchange { f.call(e); } }, ..merged } "{label}" } }
}
