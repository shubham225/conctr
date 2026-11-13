use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    Stack { Container { p { "Container constrains the content width." } } Cluster { Badge { "Cluster" } Badge { "Wraps content" } Badge { "Uses space-3" } } Divider {} p { "Stack uses the space-4 gap." } KeyboardKey { "Enter" } }
        }
}
