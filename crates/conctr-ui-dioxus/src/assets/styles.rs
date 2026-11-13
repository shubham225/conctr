use dioxus::prelude::*;

/// Bundle the complete directory so relative font URLs resolve.
pub const FONTS_DIRECTORY: Asset = asset!("/assets/fonts");
pub const TOKENS_CSS: Asset = asset!("/assets/colors/tokens.css");
/// Main shared recipes and semantic theme properties.
pub const MAIN_CSS: Asset = asset!("/assets/styles/main.css");
/// Compatibility alias. New applications should use [`MAIN_CSS`] or [`ConctrStyles`].
pub const CONCTR_UI_CSS: Asset = MAIN_CSS;

pub const PROVIDER_CSS: Asset = asset!("/assets/styles/provider.css");

/// Legacy upstream palette aliases, retained for consumers with older custom CSS.
/// Library components use semantic `--cn-*` properties directly.
pub const REGISTRY_CSS: Asset = asset!("/assets/styles/registry.css");

/// Load once at the application root, before application-specific styles.
#[component]
pub fn ConctrStyles() -> Element {
    rsx! {
        document::Stylesheet { href: format!("{FONTS_DIRECTORY}/fonts.css") }
        document::Stylesheet { href: TOKENS_CSS }
        document::Stylesheet { href: MAIN_CSS }
        document::Stylesheet { href: PROVIDER_CSS }
        document::Stylesheet { href: REGISTRY_CSS }
        super::registry_styles::RegistryStyles {}
    }
}
