use dioxus::prelude::*;
use dioxus_primitives::dialog::{self, DialogDescriptionProps, DialogRootProps, DialogTitleProps};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/ui/dialog/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/dialog/style.css",
    manganis::AssetOptions::css_module()
);

#[component]
pub fn Dialog(props: DialogRootProps) -> Element {
    let base = attributes!(div {
        class: "cn-ui-dialog cn-modal {Styles::cn_modal.inner} cn-dialog {Styles::cn_dialog.inner} flex flex-col text-left top-1/2 fixed left-1/2 overflow-y-auto",
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        dialog::DialogRoot {
            class: "cn-ui-dialog-backdrop {Styles::cn_ui_dialog_backdrop.inner} cn-dialog-backdrop {Styles::cn_dialog_backdrop.inner} inset-0 fixed",
            id: props.id,
            is_modal: props.is_modal,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            dialog::DialogContent { class: None, attributes: merged, {props.children} }
        }
    }
}

#[component]
pub fn DialogTitle(props: DialogTitleProps) -> Element {
    let base = attributes!(h2 {
        class: "cn-ui-dialog-title cn-dialog-title {Styles::cn_dialog_title.inner} text-balance",
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        dialog::DialogTitle { id: props.id, attributes: merged, {props.children} }
    }
}

#[component]
pub fn DialogDescription(props: DialogDescriptionProps) -> Element {
    let base = attributes!(p {
        class: "cn-ui-dialog-description {Styles::cn_ui_dialog_description.inner} cn-dialog-description {Styles::cn_dialog_description.inner}",
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        document::Stylesheet { href: STYLES }

        dialog::DialogDescription { id: props.id, attributes: merged, {props.children} }
    }
}
