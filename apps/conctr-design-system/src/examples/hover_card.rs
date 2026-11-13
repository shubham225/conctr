use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    HoverCard { HoverCardTrigger { "Order sync" } HoverCardContent { Stack { strong { "Example connection" } p { "REST source to SOAP target." } StatusBadge { status: Status::Info, "Draft" } } } }
        }
}
