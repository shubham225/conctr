use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    TableContainer { Table { caption { "Example connections" } thead { tr { th { "Connection" } th { "Protocol" } th { "Status" } } } tbody {
        for (name, protocol, status, label) in [("Order sync", Protocol::Rest, Status::Success, "Healthy"), ("Inventory sync", Protocol::Soap, Status::Warning, "Degraded"), ("Customer sync", Protocol::OData, Status::Info, "Draft")] {
            tr { td { "{name}" } td { ProtocolBadge { protocol } } td { StatusBadge { status, "{label}" } } }
        }
    } } }
        }
}
