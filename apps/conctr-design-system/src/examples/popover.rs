use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    div { class: "ds-overlay-demo",
    PopoverRoot { PopoverTrigger { "Connection settings" } PopoverContent { Stack { strong { "Retry policy" } p { "Retry failed requests up to three times." } } } }
        }
    }
}
