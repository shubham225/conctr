use dioxus::prelude::*;
/// Empty content with an explanation and caller-supplied primary action.
#[component]
pub fn EmptyState(
    title: String,
    description: String,
    icon: Option<Element>,
    children: Element,
) -> Element {
    rsx! { section { class: "cn-empty-state", {icon} h3 { "{title}" } p { "{description}" } {children} } }
}
