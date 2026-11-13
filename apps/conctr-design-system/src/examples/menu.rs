use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut selected = use_signal(|| "None".to_string());
    rsx! {
        DropdownMenu {
            DropdownMenuTrigger { "Connection actions" }
            DropdownMenuContent {
                DropdownMenuItem::<String> { index: 0usize, value: "view".to_string(), on_select: move |_| selected.set("View".into()), "View connection" }
                DropdownMenuItem::<String> { index: 1usize, value: "edit".to_string(), on_select: move |_| selected.set("Edit".into()), "Edit connection" }
            }
        }
        p { role: "status", "Selected action: {selected}" }
    }
}
