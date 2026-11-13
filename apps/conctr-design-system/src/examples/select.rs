use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    Field { span { class: "cn-label", "Protocol" }
        Select::<String> { aria_label: "Protocol", default_value: "REST".to_string(),
            for (index, protocol) in ["REST", "SOAP", "OData", "GraphQL", "gRPC"].iter().enumerate() {
                SelectOption::<String> { index, value: protocol.to_string(), "{protocol}" }
            }
        }
    }
        }
}
