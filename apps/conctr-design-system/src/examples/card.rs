use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    div { class: "ds-grid",
        Card { CardHeader { CardTitle { "Order sync" } CardDescription { "Example connection configuration." } } CardContent { ProtocolBadge { protocol: Protocol::Rest } } CardFooter { Button { variant: ButtonVariant::Secondary, "View connection" } } }
        Card { class: "cn-card--compact cn-card--raised", CardHeader { CardTitle { "Compact raised card" } } CardContent { p { "Shared surface, radius and shadow tokens." } } }
    }
        }
}
