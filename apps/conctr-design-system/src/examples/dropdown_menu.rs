use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut selected = use_signal(|| "None".to_string());
    rsx! {
    DropdownMenu { DropdownMenuTrigger { "Connection actions" } DropdownMenuContent {
        for (index, action) in ["Edit connection", "Duplicate connection", "Delete connection"].iter().enumerate() {
            DropdownMenuItem::<String> { index, value: action.to_string(), on_select: move |value| selected.set(value), "{action}" }
        }
    } }
    p { role: "status", "Selected action: {selected}" }
        }
}
