use dioxus::prelude::*;
use dioxus_primitives::dialog::{self, DialogDescriptionProps, DialogRootProps, DialogTitleProps};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/ui/dialog/style.css")]
struct Styles;

#[component]
pub fn Dialog(props: DialogRootProps) -> Element {
    let base = attributes!(div {
        class: format!("{} cn-modal cn-dialog", Styles::dx_dialog),
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        dialog::DialogRoot {
            class: format!("{} cn-dialog-backdrop", Styles::dx_dialog_backdrop),
            id: props.id,
            is_modal: props.is_modal,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            dialog::DialogContent {
                class: None,
                attributes: merged,
                {props.children}
            }
        }
    }
}

#[component]
pub fn DialogTitle(props: DialogTitleProps) -> Element {
    let base = attributes!(h2 {
        class: format!("{} cn-dialog-title", Styles::dx_dialog_title),
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        dialog::DialogTitle {
            id: props.id,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn DialogDescription(props: DialogDescriptionProps) -> Element {
    let base = attributes!(p {
        class: format!("{} cn-dialog-description", Styles::dx_dialog_description),
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        dialog::DialogDescription {
            id: props.id,
            attributes: merged,
            {props.children}
        }
    }
}
