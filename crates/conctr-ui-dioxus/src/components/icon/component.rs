use dioxus::prelude::*;

/// Bundled icon or image. Pass empty alt text when the adjacent label names it.
#[component]
pub fn Icon(src: Asset, #[props(default)] alt: String) -> Element {
    rsx! { img { src, alt, width: 20, height: 20 } }
}
