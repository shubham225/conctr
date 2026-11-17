use crate::composed::{ConnectionFlow, Protocol};
use dioxus::prelude::*;
#[component]
pub fn ConnectionFlowDemo() -> Element {
    rsx! {
        ConnectionFlow {
            source: "SAP S/4HANA",
            target: "Salesforce",
            source_protocol: Protocol::OData,
            target_protocol: Protocol::Rest,
        }
    }
}
