use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Stack {
            Cluster {
                Kbd { "⌘" }
                Kbd { "⇧" }
                Kbd { "⌥" }
                Kbd { "⌃" }
                Kbd { "Enter" }
            }
            Cluster {
                span { "Open command menu" }
                KbdGroup { aria_label: "Control plus K",
                    Kbd { "Ctrl" }
                    span { "+" }
                    Kbd { "K" }
                }
            }
            Cluster {
                span { "Navigate" }
                KbdGroup { aria_label: "Arrow keys",
                    Kbd { "↑" }
                    Kbd { "↓" }
                }
                span { "Select" }
                Kbd { "Enter" }
                span { "Dismiss" }
                Kbd { "Esc" }
            }
            Button { variant: ButtonVariant::Secondary,
                "Confirm action"
                Kbd { "Enter" }
            }
        }
    }
}
