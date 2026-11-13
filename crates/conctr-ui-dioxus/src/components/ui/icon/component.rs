use dioxus::prelude::*;

/// Bundled artwork. Monochrome SVG masks inherit the surrounding text color,
/// including dark themes. Pass empty alt text beside a visible label.
#[component]
pub fn Icon(
    src: Asset,
    #[props(default)] alt: String,
    #[props(default)] monochrome: bool,
) -> Element {
    if monochrome {
        rsx! { span {
            class: "cn-icon-mask",
            style: format!("mask-image: url('{src}');"),
            role: (!alt.is_empty()).then_some("img"),
            aria_hidden: alt.is_empty().then_some("true"),
            aria_label: (!alt.is_empty()).then_some(alt),
        } }
    } else {
        rsx! { img { src, alt, width: 20, height: 20 } }
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
        IconName::Connections => rsx! { Cable { size: "20" } },
        IconName::Runs => rsx! { History { size: "20" } },
        IconName::Settings => rsx! { Settings { size: "20" } },
        IconName::Search => rsx! { Search { size: "20" } },
        IconName::Add => rsx! { Plus { size: "20" } },
        IconName::Expand => rsx! { ChevronDown { size: "20" } },
        IconName::Panel => rsx! { PanelLeft { size: "20" } },
        IconName::Folder => rsx! { Folder { size: "20" } },
        IconName::More => rsx! { Ellipsis { size: "20" } },
        IconName::Close => rsx! { X { size: "20" } },
        IconName::User => rsx! { User { size: "20" } },
    };
    rsx! { span { class: "cn-ui-icon", role: label.as_ref().map(|_| "img"), aria_hidden: label.is_none().then_some("true"), aria_label: label, {content} } }
}
