use dioxus::prelude::*;
use dioxus_primitives::alert_dialog::{
    self, AlertDialogActionProps, AlertDialogActionsProps, AlertDialogCancelProps,
    AlertDialogDescriptionProps, AlertDialogRootProps, AlertDialogTitleProps,
};

#[css_module("/src/components/ui/alert_dialog/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/alert_dialog/style.css",
    manganis::AssetOptions::css_module()
);
#[css_module("/src/components/ui/button/style.css")]
struct ButtonStyles;

// Register for every document, including repeated SSR requests.
const BUTTON_STYLES: Asset = asset!(
    "/src/components/ui/button/style.css",
    manganis::AssetOptions::css_module()
);

#[component]
pub fn AlertDialog(props: AlertDialogRootProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }
        document::Stylesheet { href: BUTTON_STYLES }

        alert_dialog::AlertDialogRoot {
            class: "cn-ui-alert-dialog-backdrop {Styles::cn_ui_alert_dialog_backdrop.inner} cn-dialog-backdrop {Styles::cn_dialog_backdrop.inner} inset-0 fixed",
            id: props.id,
            default_open: props.default_open,
            open: props.open,
            on_open_change: props.on_open_change,
            attributes: props.attributes,
            alert_dialog::AlertDialogContent { class: "cn-ui-alert-dialog cn-modal {Styles::cn_modal.inner} cn-dialog {Styles::cn_dialog.inner} flex flex-col text-left top-1/2 fixed left-1/2 overflow-y-auto", {props.children} }
        }
    }
}

#[component]
pub fn AlertDialogTitle(props: AlertDialogTitleProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }
        document::Stylesheet { href: BUTTON_STYLES }

        alert_dialog::AlertDialogTitle {
            class: "cn-ui-alert-dialog-title cn-dialog-title {Styles::cn_dialog_title.inner} text-balance",
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn AlertDialogDescription(props: AlertDialogDescriptionProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }
        document::Stylesheet { href: BUTTON_STYLES }

        alert_dialog::AlertDialogDescription {
            class: "cn-ui-alert-dialog-description {Styles::cn_ui_alert_dialog_description.inner} cn-dialog-description {Styles::cn_dialog_description.inner}",
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn AlertDialogActions(props: AlertDialogActionsProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }
        document::Stylesheet { href: BUTTON_STYLES }

        alert_dialog::AlertDialogActions {
            class: "cn-ui-alert-dialog-actions {Styles::cn_ui_alert_dialog_actions.inner} cn-dialog-actions flex gap-3 mt-2",
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn AlertDialogCancel(props: AlertDialogCancelProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }
        document::Stylesheet { href: BUTTON_STYLES }

        alert_dialog::AlertDialogCancel {
            on_click: props.on_click,
            class: "cn-ui-alert-dialog-cancel {Styles::cn_ui_alert_dialog_cancel.inner} cn-btn {ButtonStyles::cn_btn.inner} cn-btn--secondary items-center gap-2 inline-flex justify-center whitespace-nowrap",
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn AlertDialogAction(props: AlertDialogActionProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }
        document::Stylesheet { href: BUTTON_STYLES }

        alert_dialog::AlertDialogAction {
            class: "cn-ui-alert-dialog-action {Styles::cn_ui_alert_dialog_action.inner} cn-btn {ButtonStyles::cn_btn.inner} cn-btn--danger items-center gap-2 inline-flex justify-center whitespace-nowrap",
            on_click: props.on_click,
            attributes: props.attributes,
            {props.children}
        }
    }
}
