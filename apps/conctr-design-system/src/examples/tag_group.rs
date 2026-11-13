use conctr_ui_dioxus::components::tag_group::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    TagGroup { TagGroupLabel { "Protocols" } TagList { TagGroupEmpty { "No protocols selected." }
        for (index, protocol) in ["REST", "SOAP", "OData"].iter().enumerate() { Tag { index, value: *protocol, "{protocol}" RemoveButton {} } }
    } }
        }
}
