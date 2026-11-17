use crate::{composed::InputGroup, ui::input::Input};
use dioxus::prelude::*;
#[component]
pub fn InputGroupDemo() -> Element {
    rsx! {
        InputGroup { prefix: rsx! { "https://" }, suffix: rsx! { "Endpoint" },
            Input { placeholder: "api.example.test" }
        }
    }
}
