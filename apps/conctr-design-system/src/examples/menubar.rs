use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut selected = use_signal(|| "None".to_string());
    rsx! {
    Menubar { for (index, title) in ["Connection", "Edit"].iter().enumerate() { MenubarMenu { index, MenubarTrigger { "{title}" } MenubarContent {
        MenubarItem { index: 0usize, value: "New connection".to_string(), on_select: move |v| selected.set(v), "New connection" }
        MenubarItem { index: 1usize, value: "Save connection".to_string(), on_select: move |v| selected.set(v), "Save connection" }
    } } } }
    p { role: "status", "Selected action: {selected}" }
        }
}
