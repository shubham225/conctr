use dioxus::prelude::*;

#[css_module("/src/components/ui/icon/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/icon/style.css",
    manganis::AssetOptions::css_module()
);

/// Bundled artwork. Monochrome SVG masks inherit the surrounding text color,
/// including dark themes. Pass empty alt text beside a visible label.
#[component]
pub fn Icon(
    src: Asset,
    #[props(default)] alt: String,
    #[props(default)] monochrome: bool,
) -> Element {
    if monochrome {
        rsx! {
            document::Stylesheet { href: STYLES }
            span {
                class: "cn-icon-mask {Styles::cn_icon_mask.inner} shrink-0 w-5 h-5 inline-block bg-current",
                style: format!("mask-image: url('{src}');"),
                role: (!alt.is_empty()).then_some("img"),
                aria_hidden: alt.is_empty().then_some("true"),
                aria_label: (!alt.is_empty()).then_some(alt),
            }
        }
    } else {
        rsx! {
            document::Stylesheet { href: STYLES }
            img {
                src,
                alt,
                width: 20,
                height: 20,
            }
        }
    }
}

/// General interface symbols from the existing Lucide dependency. Protocols
/// continue to use the supplied kit artwork and their fixed shape meanings.
#[derive(Clone, Copy, PartialEq)]
pub enum IconName {
    Connections,
    Runs,
    Settings,
    Search,
    Add,
    Expand,
    Panel,
    Folder,
    More,
    Close,
    User,
}

/// A 20px current-color interface icon. Decorative unless `label` is supplied.
#[component]
pub fn UiIcon(name: IconName, #[props(default)] label: Option<String>) -> Element {
    use dioxus_icons::lucide::{
        Cable, ChevronDown, Ellipsis, Folder, History, PanelLeft, Plus, Search, Settings, User, X,
    };
    let content = match name {
        IconName::Connections => rsx! {
            document::Stylesheet { href: STYLES }
            Cable { size: "20" }
        },
        IconName::Runs => rsx! {
            document::Stylesheet { href: STYLES }
            History { size: "20" }
        },
        IconName::Settings => rsx! {
            document::Stylesheet { href: STYLES }
            Settings { size: "20" }
        },
        IconName::Search => rsx! {
            document::Stylesheet { href: STYLES }
            Search { size: "20" }
        },
        IconName::Add => rsx! {
            document::Stylesheet { href: STYLES }
            Plus { size: "20" }
        },
        IconName::Expand => rsx! {
            document::Stylesheet { href: STYLES }
            ChevronDown { size: "20" }
        },
        IconName::Panel => rsx! {
            document::Stylesheet { href: STYLES }
            PanelLeft { size: "20" }
        },
        IconName::Folder => rsx! {
            document::Stylesheet { href: STYLES }
            Folder { size: "20" }
        },
        IconName::More => rsx! {
            document::Stylesheet { href: STYLES }
            Ellipsis { size: "20" }
        },
        IconName::Close => rsx! {
            document::Stylesheet { href: STYLES }
            X { size: "20" }
        },
        IconName::User => rsx! {
            document::Stylesheet { href: STYLES }
            User { size: "20" }
        },
    };
    rsx! {
        document::Stylesheet { href: STYLES }
        span {
            class: "cn-ui-icon {Styles::cn_ui_icon.inner} items-center shrink-0 justify-center text-inherit",
            role: label.as_ref().map(|_| "img"),
            aria_hidden: label.is_none().then_some("true"),
            aria_label: label,
            {content}
        }
    }
}
