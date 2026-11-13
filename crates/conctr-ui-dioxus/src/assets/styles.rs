use dioxus::prelude::*;

/// Bundle the complete directory so relative font URLs resolve.
pub const FONTS_DIRECTORY: Asset = asset!("/assets/fonts");
pub const TOKENS_CSS: Asset = asset!("/assets/colors/tokens.css");
pub const CONCTR_UI_CSS: Asset = asset!("/assets/styles/conctr-ui.css");

pub const PROVIDER_CSS: Asset = asset!("/assets/styles/provider.css");

pub const REGISTRY_CSS: Asset = asset!("/assets/styles/registry.css");

/// Load once at the application root, before application-specific styles.
#[component]
pub fn ConctrStyles() -> Element {
    rsx! {
        document::Stylesheet { href: format!("{FONTS_DIRECTORY}/fonts.css") }
        document::Stylesheet { href: TOKENS_CSS }
        document::Stylesheet { href: CONCTR_UI_CSS }
        document::Stylesheet { href: PROVIDER_CSS }
        document::Stylesheet { href: REGISTRY_CSS }
        super::registry_styles::RegistryStyles {}
    }
}
