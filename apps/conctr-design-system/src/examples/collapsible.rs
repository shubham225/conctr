use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    Collapsible { CollapsibleTrigger { "View request details" } CollapsibleContent { CodeBlock { "GET /api/connections\nAccept: application/json" } } }
        }
}
