use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut notes = use_signal(String::new);
    rsx! {
    Field { Label { html_for: "notes", "Connection notes" } Textarea { id: "notes", value: notes(), oninput: move |e: FormEvent| notes.set(e.value()), placeholder: "Describe the connection and its purpose." } Hint { "Keep operational context with the connection." } }
        }
}
