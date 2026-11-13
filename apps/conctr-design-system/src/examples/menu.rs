use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut selected = use_signal(|| "None".to_string());
    rsx! {
    Cluster { Dropdown { label: "Native actions", MenuItem { onclick: move |_| selected.set("Edit".into()), "Edit connection" } MenuItem { danger: true, onclick: move |_| selected.set("Delete".into()), "Delete connection" } } Menu { MenuItem { onclick: move |_| selected.set("View".into()), "View connection" } } }
    p { role: "status", "Selected action: {selected}" }
        }
}
