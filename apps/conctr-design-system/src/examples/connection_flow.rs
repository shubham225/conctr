use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    Stack { for (state, label) in [(FlowState::Healthy, "Healthy example"), (FlowState::Paused, "Paused by the operator"), (FlowState::Failing, "Target rejected the request. Check the credentials.")] {
        ConnectionFlow { state, message: label.to_string(),
            source: rsx! { ConnectionNode { b { "Order service" } ProtocolBadge { protocol: Protocol::Rest } } },
            target: rsx! { ConnectionNode { b { "ERP system" } ProtocolBadge { protocol: Protocol::Soap } } },
        }
    } }
        }
}
