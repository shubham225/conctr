use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut pressed = use_signal(|| false);
    rsx! {
    Stack {
        Cluster {
            Toggle { pressed: Some(pressed()), on_pressed_change: move |v| pressed.set(v), "Pin connection" }
            Toggle { default_pressed: true, "Selected" }
            Toggle { disabled: true, "Disabled" }
        }
        p { role: "status", if pressed() { "Connection pinned" } else { "Connection unpinned" } }
    }
        }
}
