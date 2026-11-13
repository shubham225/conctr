use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut selected = use_signal(|| "None".to_string());
    rsx! {
    ContextMenu { ContextMenuTrigger { div { class: "ds-context-target", "Right-click here for connection actions" } } ContextMenuContent {
        ContextMenuItem { index: 0usize, value: "Edit connection".to_string(), on_select: move |v| selected.set(v), "Edit connection" }
        ContextMenuItem { index: 1usize, value: "Duplicate connection".to_string(), on_select: move |v| selected.set(v), "Duplicate connection" }
    } }
    p { role: "status", "Selected action: {selected}" }
        }
}
