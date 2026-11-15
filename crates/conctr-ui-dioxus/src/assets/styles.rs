use dioxus::prelude::*;
pub const LICENSES: Asset = asset!("/assets/licenses");
pub const FONTS: Asset = asset!("/assets/fonts");
pub const TOKENS_CSS: Asset = asset!("/assets/colors/tokens.css");
pub const MAIN_CSS: Asset = asset!("/assets/styles/main.css");
pub const TAILWIND_CSS: Asset = asset!("/assets/styles/tailwind.css");
/// Load the complete bundled brand system; consumers require no Node tooling.
#[component]
pub fn ConctrStyles() -> Element {
    rsx! {
        document::Link { rel: "license", href: format!("{LICENSES}/NOTICE.txt") }
        document::Stylesheet { href: format!("{FONTS}/fonts.css") }
        document::Stylesheet { href: TOKENS_CSS }
        document::Stylesheet { href: MAIN_CSS }
        document::Stylesheet { href: TAILWIND_CSS }
    }
}
