use dioxus::prelude::*;
use dioxus_icons::lucide::X;
use dioxus_primitives::dialog::{
    self, DialogCtx, DialogDescriptionProps, DialogRootProps, DialogTitleProps,
};
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

#[css_module("/src/components/ui/sheet/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/sheet/style.css",
    manganis::AssetOptions::css_module()
);

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SheetSide {
    Top,
    #[default]
    Right,
    Bottom,
    Left,
}

impl SheetSide {
    pub fn as_str(&self) -> &'static str {
        match self {
            SheetSide::Top => "top",
            SheetSide::Right => "right",
            SheetSide::Bottom => "bottom",
            SheetSide::Left => "left",
        }
    }
}

#[component]
pub fn Sheet(props: DialogRootProps) -> Element {
    let content_base = attributes!(div {
        class: "cn-ui-sheet {Styles::cn_ui_sheet.inner} flex bg-card text-foreground flex-col gap-4 box-border fixed bg-popover gap-6 max-h-dvh overflow-y-auto",
        "data-slot": "sheet-content",
        "data-side": SheetSide::Right.as_str(),
    });
    let content_attributes = merge_attributes(vec![content_base, props.attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        dialog::DialogRoot {
            class: "cn-ui-sheet-root {Styles::cn_ui_sheet_root.inner} inset-0 fixed",
            "data-slot": "sheet-root",
            id: props.id,
            is_modal: props.is_modal,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            dialog::DialogContent { class: None, attributes: content_attributes, {props.children} }
        }
    }
}

#[component]
pub fn SheetContentClose(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let base = attributes!(button {
        class: "cn-ui-sheet-close {Styles::cn_ui_sheet_close.inner} flex items-center absolute p-0 cursor-pointer w-6 h-6 justify-center top-4 right-4 w-(--cn-control-sm) h-(--cn-control-sm)",
    });
    let attributes = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        SheetClose { attributes,
            X { size: "20px" }
        }
    }
}

#[component]
pub fn SheetHeader(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        div {
            class: "cn-ui-sheet-header {Styles::cn_ui_sheet_header.inner} flex flex-col",
            "data-slot": "sheet-header",
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn SheetFooter(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        div {
            class: "cn-ui-sheet-footer {Styles::cn_ui_sheet_footer.inner} flex-col mt-auto",
            "data-slot": "sheet-footer",
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn SheetTitle(props: DialogTitleProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        dialog::DialogTitle {
            id: props.id,
            class: "cn-ui-sheet-title {Styles::cn_ui_sheet_title.inner} text-foreground",
            "data-slot": "sheet-title",
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn SheetDescription(props: DialogDescriptionProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        dialog::DialogDescription {
            id: props.id,
            class: "cn-ui-sheet-description {Styles::cn_ui_sheet_description.inner} text-muted-foreground",
            "data-slot": "sheet-description",
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn SheetClose(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    r#as: Option<Callback<Vec<Attribute>, Element>>,
    children: Element,
) -> Element {
    let ctx: DialogCtx = use_context();

    let base = attributes! {
        button {
            onclick: move |_| {
                ctx.set_open(false);
            }
        }
    };
    let merged = merge_attributes(vec![base, attributes]);

    if let Some(dynamic) = r#as {
        dynamic.call(merged)
    } else {
        rsx! {
            document::Stylesheet { href: STYLES }

            button { ..merged,{children} }
        }
    }
}
