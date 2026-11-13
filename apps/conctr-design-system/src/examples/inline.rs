use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! { Inline { gap: Space::Sm, for name in ["REST", "SOAP", "OData", "GraphQL", "gRPC", "Webhook"] { Badge { "{name}" } } } }
}
