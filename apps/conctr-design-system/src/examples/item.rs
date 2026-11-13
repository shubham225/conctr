use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    Stack { Item { ItemContent { ItemTitle { "Order sync" } ItemDescription { "Example integration configuration." } } ItemActions { Button { variant: ButtonVariant::Secondary, "View connection" } } } ItemSeparator {} Item { ItemContent { ItemTitle { "Inventory sync" } ItemDescription { "REST source, SOAP target." } } } }
        }
}
