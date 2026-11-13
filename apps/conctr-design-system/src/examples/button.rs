use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut count = use_signal(|| 0);
    rsx! {
    Stack {
        Cluster {
            Button { onclick: move |_| count += 1, "Test connection" }
            Button { variant: ButtonVariant::Secondary, "Configure source" }
            Button { variant: ButtonVariant::Ghost, "View details" }
            Button { variant: ButtonVariant::Danger, "Delete connection" }
        }
        Cluster {
            Button { size: ButtonSize::Sm, "Small action" }
            Button { "Default action" }
            Button { size: ButtonSize::Lg, "Large action" }
        }
        Cluster { Button { disabled: true, "Disabled" } Button { loading: true, "Testing connection" } }
        p { role: "status", "Test action invoked {count} times." }
    }
        }
}
