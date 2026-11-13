use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut state = use_signal(|| CheckboxState::Checked);
    rsx! {
    Stack {
        Cluster { Checkbox { id: "check-connection", checked: Some(state()), on_checked_change: move |next| state.set(next) } Label { html_for: "check-connection", "Enable connection" } }
        Cluster { Checkbox { id: "check-mixed", default_checked: CheckboxState::Indeterminate } Label { html_for: "check-mixed", "Mixed selection" } }
        Cluster { Checkbox { id: "check-disabled", disabled: true } Label { html_for: "check-disabled", "Unavailable connection" } }
    }
        }
}
