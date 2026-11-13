use dioxus::prelude::*;

/// Navigation anchor with the kit's selected state.
#[component]
pub fn NavItem(href: String, #[props(default)] active: bool, children: Element) -> Element {
    rsx! { a { class: "cn-nav__item", href, aria_current: if active { Some("page") } else { None }, {children} } }
}
