use dioxus::prelude::*;

/// A regular action inside a disclosure menu (native Tab navigation).
#[component]
pub fn MenuItem(
    #[props(default)] danger: bool,
    #[props(default)] disabled: bool,
    onclick: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    rsx! { button { class: if danger { "cn-menu__item cn-menu__item--danger" } else { "cn-menu__item" }, r#type: "button", disabled, onclick, {children} } }
}
