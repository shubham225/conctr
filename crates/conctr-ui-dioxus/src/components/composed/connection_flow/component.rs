use dioxus::prelude::*;

/// Execution state of the connection summary. A failing flow also needs a message.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum FlowState {
    #[default]
    Healthy,
    Paused,
    Failing,
}

/// Source/hub/target summary with application-provided nodes and optional status text.
#[component]
pub fn ConnectionFlow(
    source: Element,
    target: Element,
    #[props(default)] state: FlowState,
    message: Option<String>,
) -> Element {
    let state_name = match state {
        FlowState::Healthy => "healthy",
        FlowState::Paused => "paused",
        FlowState::Failing => "failing",
    };
    rsx! {
        div { class: "cn-stack",
            div { class: "cn-flow", "data-state": state_name,
                {source}
                div { class: "cn-link", aria_hidden: "true" }
                div { class: "cn-hub", "CONCTR" }
                div { class: "cn-link", aria_hidden: "true" }
                {target}
            }
            if let Some(message) = message { p { role: "status", "{message}" } }
        }
    }
}
