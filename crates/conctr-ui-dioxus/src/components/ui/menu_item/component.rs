use dioxus::prelude::*;

#[css_module("/src/components/ui/menu_item/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/menu_item/style.css",
    manganis::AssetOptions::css_module()
);
#[css_module("/src/components/ui/menu/style.css")]
struct MenuStyles;

// Register for every document, including repeated SSR requests.
const MENU_STYLES: Asset = asset!(
    "/src/components/ui/menu/style.css",
    manganis::AssetOptions::css_module()
);

/// A regular action inside a disclosure menu (native Tab navigation).
#[component]
pub fn MenuItem(
    #[props(default)] danger: bool,
    #[props(default)] disabled: bool,
    onclick: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    let class = format!(
        "cn-menu__item {} {} flex items-center gap-3 border-0 w-full text-left cursor-pointer py-0 px-3 rounded-sm h-9",
        Styles::cn_menu__item.inner,
        if danger { "cn-menu__item--danger" } else { "" },
    );
    rsx! {
        document::Stylesheet { href: STYLES }
        document::Stylesheet { href: MENU_STYLES }
        button {
            class,
            r#type: "button",
            disabled,
            onclick,
            {children}
        }
    }
}
