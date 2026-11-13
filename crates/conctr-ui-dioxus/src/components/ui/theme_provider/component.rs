use crate::components::ui::theme::Theme;
use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

/// Application surface and shared stylesheet loader. Providers may be nested.
#[component]
pub fn ThemeProvider(
    #[props(default)] theme: Theme,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let theme = match theme {
        Theme::System => None,
        Theme::Light => Some("light"),
        Theme::Dark => Some("dark"),
    };
    let base = attributes!(div { class: "cn-body", "data-theme": theme });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! { crate::assets::styles::ConctrStyles {} div { ..merged, {children} } }
}
