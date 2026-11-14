use dioxus::prelude::*;

#[css_module("/src/components/ui/nav_item/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/nav_item/style.css",
    manganis::AssetOptions::css_module()
);

/// Navigation anchor with the kit's selected state.
#[component]
pub fn NavItem(href: String, #[props(default)] active: bool, children: Element) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }
        a {
            class: "cn-nav__item {Styles::cn_nav__item.inner} flex items-center rounded-md text-sm gap-3 font-medium py-0 no-underline px-3 h-9",
            href,
            aria_current: if active { Some("page") } else { None },
            {children}
        }
    }
}
