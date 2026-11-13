use dioxus::prelude::*;
/// Semantic heading levels, styled with the kit's Sora type scale.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum HeadingLevel {
    #[default]
    H1,
    H2,
    H3,
    H4,
}
#[component]
pub fn Heading(#[props(default)] level: HeadingLevel, children: Element) -> Element {
    match level {
        HeadingLevel::H1 => rsx! { h1 { class: "cn-h1", {children} } },
        HeadingLevel::H2 => rsx! { h2 { class: "cn-h2", {children} } },
        HeadingLevel::H3 => rsx! { h3 { class: "cn-h3", {children} } },
        HeadingLevel::H4 => rsx! { h4 { class: "cn-h4", {children} } },
    }
}
