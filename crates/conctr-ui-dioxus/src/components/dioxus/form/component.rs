use dioxus::prelude::*;
/// Native form boundary. Submission policy and validation belong to the caller.
#[component]
pub fn Form(
    onsubmit: EventHandler<FormEvent>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = form)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! { form { class: "cn-stack", onsubmit, ..attributes, {children} } }
}
