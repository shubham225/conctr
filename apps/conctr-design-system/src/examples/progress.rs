use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut value = use_signal(|| 40.0f64);
    rsx! {
    Stack {
        Progress { value: Some(value()), max: 100.0, aria_label: "Example sync" }
        p { role: "status", "Example sync: {value():.0}%" }
        Cluster {
            Button { variant: ButtonVariant::Secondary, onclick: move |_| value.set((value() + 10.0).min(100.0)), "Advance progress" }
            Button { variant: ButtonVariant::Ghost, onclick: move |_| value.set(0.0), "Reset progress" }
        }
        Progress { value: None, aria_label: "Indeterminate progress" }
    }
        }
}
