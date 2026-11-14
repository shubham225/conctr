use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut saved = use_signal(|| false);
    rsx! {
        Form {
            onsubmit: move |event: FormEvent| {
                event.prevent_default();
                saved.set(true);
            },
            Field {
                Label { html_for: "form-name", "Connection name" }
                Input { id: "form-name", name: "name", required: true }
            }
            div {
                Button { r#type: "submit", "Save connection" }
            }
            if saved() {
                Alert { status: Status::Success, "Example form saved locally." }
            }
        }
    }
}
