use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
    Button { onclick: move |_| open.set(true), "Open dialog" }
    Dialog { open: open(), on_open_change: move |v| open.set(v),
        DialogTitle { "Connection details" }
        DialogDescription { "A dialog composed from accessible Dioxus primitives." }
        ModalFooter { Button { variant: ButtonVariant::Secondary, onclick: move |_| open.set(false), "Close dialog" } }
    }
        }
}
