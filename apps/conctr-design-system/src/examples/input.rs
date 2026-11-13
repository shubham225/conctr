use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut endpoint = use_signal(String::new);
    rsx! {
    Stack {
        Field { Label { html_for: "endpoint", "Endpoint URL" } Input { id: "endpoint", value: endpoint(), oninput: move |e: FormEvent| endpoint.set(e.value()), placeholder: "https://api.example.com" } Hint { "Include the protocol and hostname." } }
        Field { Label { html_for: "invalid-endpoint", "Endpoint with error" } Input { id: "invalid-endpoint", aria_invalid: "true", aria_describedby: "endpoint-error", value: "api.example.com" } FieldError { id: "endpoint-error", "Add the scheme, for example https://." } }
        Input { aria_label: "Disabled endpoint", disabled: true, placeholder: "Disabled endpoint" }
    }
        }
}
