use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut enabled = use_signal(|| false);
    rsx! {
    Stack {
        Cluster { Switch { id: "switch-live", checked: enabled(), on_checked_change: move |next| enabled.set(next) } Label { html_for: "switch-live", "Enable connection" } }
        Cluster { Switch { id: "switch-on", default_checked: true, disabled: true } Label { html_for: "switch-on", "Disabled on" } }
        Cluster { Switch { id: "switch-off", disabled: true } Label { html_for: "switch-off", "Disabled off" } }
        p { role: "status", if enabled() { "Connection enabled" } else { "Connection paused" } }
    }
        }
}
