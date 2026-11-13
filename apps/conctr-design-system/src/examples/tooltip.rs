use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    Tooltip { TooltipTrigger { "Connection details" } TooltipContent { "View the source, target and mapping." } }
        }
}
