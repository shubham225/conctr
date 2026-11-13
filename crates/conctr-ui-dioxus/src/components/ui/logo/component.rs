use dioxus::prelude::*;

/// Supplied horizontal wordmark, automatically chosen for the current theme.
#[component]
pub fn Logo() -> Element {
    use crate::assets::images::{HORIZONTAL_COLOR_DARK_SVG, HORIZONTAL_COLOR_SVG};
    rsx! {
        img { class: "cn-logo--light", src: HORIZONTAL_COLOR_SVG, alt: "Conctr", height: 32 }
        img { class: "cn-logo--dark", src: HORIZONTAL_COLOR_DARK_SVG, alt: "Conctr", height: 32 }
    }
}
