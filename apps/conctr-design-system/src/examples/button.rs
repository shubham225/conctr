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
        Cluster {
            Tooltip {
                TooltipTrigger { as: move |attrs: Vec<Attribute>| rsx! {
                    Button { size: ButtonSize::Icon, variant: ButtonVariant::Secondary, aria_label: "Connection settings", attributes: attrs,
                        UiIcon { name: IconName::Settings }
                    }
                } }
                TooltipContent { "Connection settings" }
            }
            Button { size: ButtonSize::IconSm, variant: ButtonVariant::Ghost, aria_label: "Connection settings", UiIcon { name: IconName::Settings } }
            Button { size: ButtonSize::IconLg, aria_label: "Connection settings", UiIcon { name: IconName::Settings } }
        }
        Button { variant: ButtonVariant::Danger, style: "--radius: 999px;", "Custom rounded action" }
        p { role: "status", "Test action invoked {count} times." }
    }
        }
}
