use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
    Button { onclick: move |_| open.set(true), "Open Conctr modal" }
    if open() { Modal { id: "kit-modal", title: "Save connection?", onclose: move |_| open.set(false), p { "A convenience wrapper over the shared Dioxus Dialog." } ModalFooter { Button { variant: ButtonVariant::Secondary, onclick: move |_| open.set(false), "Cancel" } Button { onclick: move |_| open.set(false), "Save connection" } } } }
        }
}
