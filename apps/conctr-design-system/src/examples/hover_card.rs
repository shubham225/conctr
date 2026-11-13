use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    div { class: "ds-overlay-demo",
    HoverCard { HoverCardTrigger { "Order sync" } HoverCardContent { Stack { strong { "Example connection" } p { "REST source to SOAP target." } Badge { variant: BadgeVariant::Info, "Draft" } } } }
        }
    }
}
