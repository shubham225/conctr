use crate::components::ui::theme::Theme;
use dioxus::prelude::*;

/// Application surface and shared stylesheet loader. Providers may be nested.
#[component]
pub fn ThemeProvider(#[props(default)] theme: Theme, children: Element) -> Element {
    let theme = match theme {
        Theme::System => None,
        Theme::Light => Some("light"),
        Theme::Dark => Some("dark"),
    };
    rsx! { crate::assets::styles::ConctrStyles {} div { class: "cn-body", "data-theme": theme, {children} } }
}
