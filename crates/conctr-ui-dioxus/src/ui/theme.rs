use dioxus::prelude::*;
/// Explicit theme scopes produce the same initial server and client markup.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum Theme {
    #[default]
    Light,
    Dark,
}
/// Brand surface and style loader. Nest providers to preview both palettes.
#[component]
pub fn ThemeProvider(#[props(default)] theme: Theme, children: Element) -> Element {
    rsx! {
        crate::assets::styles::ConctrStyles {}
        div { class: "cn-body min-h-full bg-background text-foreground font-sans", "data-theme": if theme == Theme::Dark { "dark" } else { "light" }, crate::ui::toast::ToastProvider { {children} } }
    }
}
