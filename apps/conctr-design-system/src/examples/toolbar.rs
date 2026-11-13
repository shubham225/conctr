use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut selection = use_signal(|| "None".to_string());
    rsx! {
    Toolbar { aria_label: "Example actions", ToolbarGroup {
        ToolbarButton { index: 0usize, on_click: move |_| selection.set("Create".to_string()), "Create" }
        ToolbarButton { index: 1usize, on_click: move |_| selection.set("Edit".to_string()), "Edit" }
    } ToolbarSeparator {} ToolbarGroup { ToolbarButton { index: 2usize, on_click: move |_| selection.set("Archive".to_string()), "Archive" } } }
    p { role: "status", "Selected action: {selection}" }
        }
}
