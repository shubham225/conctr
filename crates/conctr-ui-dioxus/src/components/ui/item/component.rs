use crate::components::ui::separator::Separator;
use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

#[css_module("/src/components/ui/item/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/item/style.css",
    manganis::AssetOptions::css_module()
);

#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum ItemVariant {
    #[default]
    Default,
    Outline,
    Muted,
}

impl ItemVariant {
    pub fn class(&self) -> &'static str {
        match self {
            ItemVariant::Default => "default",
            ItemVariant::Outline => "outline",
            ItemVariant::Muted => "muted",
        }
    }
}

#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum ItemSize {
    #[default]
    Default,
    Sm,
}

impl ItemSize {
    pub fn class(&self) -> &'static str {
        match self {
            ItemSize::Default => "default",
            ItemSize::Sm => "sm",
        }
    }
}

#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum ItemMediaVariant {
    #[default]
    Default,
    Icon,
    Image,
}

impl ItemMediaVariant {
    pub fn class(&self) -> &'static str {
        match self {
            ItemMediaVariant::Default => "default",
            ItemMediaVariant::Icon => "icon",
            ItemMediaVariant::Image => "image",
        }
    }
}

#[component]
pub fn ItemGroup(
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "cn-ui-item-group flex flex-col",
        role: "list",
        "data-slot": "item-group",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        div { ..merged,{children} }
    }
}

#[component]
pub fn ItemSeparator(
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
) -> Element {
    let base = attributes!(div {
        class: "cn-ui-item-separator m-0",
        "data-slot": "item-separator",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        Separator { horizontal: true, decorative: true, attributes: merged }
    }
}

#[component]
pub fn Item(
    #[props(default)] variant: ItemVariant,
    #[props(default)] size: ItemSize,
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    r#as: Option<Callback<Vec<Attribute>, Element>>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "cn-ui-item {Styles::cn_ui_item.inner} flex flex-wrap items-center text-sm box-border",
        "data-slot": "item",
        "data-variant": variant.class(),
        "data-size": size.class(),
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        if let Some(dynamic) = r#as {
            {dynamic.call(merged)}
        } else {
            div {
                onclick: move |event| {
                    if let Some(f) = &onclick {
                        f.call(event);
                    }
                },
                onkeydown: move |event| {
                    if let Some(f) = &onkeydown {
                        f.call(event);
                    }
                },
                ..merged,
                {children}
            }
        }
    }
}

#[component]
pub fn ItemMedia(
    #[props(default)] variant: ItemMediaVariant,
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "cn-ui-item-media {Styles::cn_ui_item_media.inner} flex items-center gap-2 shrink-0 justify-center",
        "data-slot": "item-media",
        "data-variant": variant.class(),
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        div { ..merged,{children} }
    }
}

#[component]
pub fn ItemContent(
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "cn-ui-item-content {Styles::cn_ui_item_content.inner} flex gap-1 flex-col",
        "data-slot": "item-content",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        div { ..merged,{children} }
    }
}

#[component]
pub fn ItemTitle(
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "cn-ui-item-title {Styles::cn_ui_item_title.inner} flex items-center gap-2 w-fit",
        "data-slot": "item-title",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        div { ..merged,{children} }
    }
}

#[component]
pub fn ItemDescription(
    #[props(extends=GlobalAttributes)]
    #[props(extends=p)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(p {
        class: "cn-ui-item-description {Styles::cn_ui_item_description.inner} text-left overflow-hidden text-balance m-0",
        "data-slot": "item-description",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        p { ..merged,{children} }
    }
}

#[component]
pub fn ItemActions(
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "cn-ui-item-actions flex items-center gap-2",
        "data-slot": "item-actions",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        div { ..merged,{children} }
    }
}

#[component]
pub fn ItemHeader(
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "cn-ui-item-header {Styles::cn_ui_item_header.inner}",
        "data-slot": "item-header",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        div { ..merged,{children} }
    }
}

#[component]
pub fn ItemFooter(
    #[props(extends=GlobalAttributes)]
    #[props(extends=div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "cn-ui-item-footer {Styles::cn_ui_item_footer.inner}",
        "data-slot": "item-footer",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        div { ..merged,{children} }
    }
}
