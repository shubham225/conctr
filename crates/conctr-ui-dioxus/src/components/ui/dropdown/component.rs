use dioxus::prelude::*;

#[css_module("/src/components/ui/button/style.css")]
struct ButtonStyles;

// Register for every document, including repeated SSR requests.
const BUTTON_STYLES: Asset = asset!(
    "/src/components/ui/button/style.css",
    manganis::AssetOptions::css_module()
);
#[css_module("/src/components/ui/menu/style.css")]
struct MenuStyles;

// Register for every document, including repeated SSR requests.
const MENU_STYLES: Asset = asset!(
    "/src/components/ui/menu/style.css",
    manganis::AssetOptions::css_module()
);

/// Native disclosure menu. Browser handles focus and keyboard activation.
#[component]
pub fn Dropdown(label: String, children: Element) -> Element {
    rsx! {
        document::Stylesheet { href: BUTTON_STYLES }
        document::Stylesheet { href: MENU_STYLES }
        details {
            summary { class: "cn-btn {ButtonStyles::cn_btn.inner} cn-btn--secondary items-center gap-2 inline-flex justify-center whitespace-nowrap", "{label}" }
            div { class: "cn-menu {MenuStyles::cn_menu.inner} border border-solid border-border rounded-md p-1 bg-popover min-w-50 text-popover-foreground", {children} }
        }
    }
}
