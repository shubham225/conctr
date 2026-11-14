//! Shared foundations loaded by [`ConctrStyles`]. Component CSS modules register
//! themselves locally; there is no global component stylesheet registry.
use dioxus::prelude::*;
/// Bundle the complete directory so relative font URLs resolve.
pub const FONTS_DIRECTORY: Asset = asset!("/assets/fonts");
pub const TOKENS_CSS: Asset = asset!("/assets/colors/tokens.css");
/// Runtime theme, typography and shared interaction styles.
pub const MAIN_CSS: Asset = asset!("/assets/styles/main.css");
/// Precompiled library utilities; consumers need no Tailwind tooling.
pub const TAILWIND_CSS: Asset = asset!("/assets/styles/tailwind.css");
/// Compatibility alias. New applications should use [`MAIN_CSS`] or [`ConctrStyles`].
pub const CONCTR_UI_CSS: Asset = MAIN_CSS;
/// Compatibility alias: provider defaults are now part of [`MAIN_CSS`].
pub const PROVIDER_CSS: Asset = MAIN_CSS;
/// Load the library's fonts, tokens, theme and Tailwind utilities.
/// [`crate::components::ThemeProvider`] includes this automatically.
#[component]
pub fn ConctrStyles() -> Element {
    rsx! {
        document::Stylesheet { href: format!("{FONTS_DIRECTORY}/fonts.css") }
        document::Stylesheet { href: TOKENS_CSS }
        document::Stylesheet { href: MAIN_CSS }
        document::Stylesheet { href: TAILWIND_CSS }
    }
}
