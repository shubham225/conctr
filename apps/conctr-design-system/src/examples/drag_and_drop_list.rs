use conctr_ui_dioxus::components::drag_and_drop_list::DragAndDropList;
use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    DragAndDropList { aria_label: "Connection processing order", is_removable: true,
        items: ["Read source", "Map fields", "Write target"].iter().map(|label| rsx! { div { key: "{label}", "{label}" } }).collect::<Vec<Element>>(),
    }
    Hint { "Drag a row or focus its handle and use the keyboard instructions." }
        }
}
