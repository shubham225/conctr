use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut query = use_signal(String::new);
    rsx! {
    Stack {
        Combobox::<String> { query: Some(query()), on_query_change: move |v| query.set(v), placeholder: "Find a protocol…", aria_label: "Protocol", list_aria_label: "Protocols",
            ComboboxEmpty { "No matching protocols." }
            for (index, protocol) in ["REST", "SOAP", "OData", "GraphQL", "gRPC", "Webhook"].iter().enumerate() {
                ComboboxOption::<String> { index, value: protocol.to_string(), text_value: protocol.to_string(), "{protocol}" }
            }
        }
        Hint { "Type to filter, then select with the keyboard or pointer." }
    }
        }
}
