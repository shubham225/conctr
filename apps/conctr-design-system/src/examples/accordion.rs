use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    Accordion { allow_multiple_open: false, horizontal: false,
        for (index, (title, description)) in [("What is a connection?", "A connection describes how systems exchange data."), ("Which protocols are supported?", "Use REST, SOAP, OData and the other protocol primitives in your application."), ("Disabled section", "Unavailable content")].iter().enumerate() {
            AccordionItem { index, disabled: index == 2,
                AccordionTrigger { "{title}" }
                AccordionContent { p { "{description}" } }
            }
        }
    }
        }
}
