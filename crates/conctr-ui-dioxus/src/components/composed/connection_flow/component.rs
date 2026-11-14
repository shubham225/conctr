use dioxus::prelude::*;

#[css_module("/src/components/composed/connection_flow/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/composed/connection_flow/style.css",
    manganis::AssetOptions::css_module()
);

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
        document::Stylesheet { href: STYLES }

        div { class: "cn-stack min-w-0 flex flex-col gap-4",
            div { class: "cn-flow {Styles::cn_flow.inner} items-center", "data-state": state_name,
                {source}
                div { class: "cn-link {Styles::cn_link.inner}", aria_hidden: "true" }
                div { class: "cn-hub {Styles::cn_hub.inner} grid w-18 h-20 bg-primary text-primary-foreground place-items-center", "CONCTR" }
                div { class: "cn-link {Styles::cn_link.inner}", aria_hidden: "true" }
                {target}
            }
            if let Some(message) = message {
                p { role: "status", "{message}" }
            }
        }
    }
}
