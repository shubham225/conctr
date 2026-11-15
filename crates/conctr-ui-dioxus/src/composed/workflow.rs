//! Reusable canvas frame over the Conctr UI facade.
use crate::ui::workflow::{WorkflowCanvas, WorkflowControls, WorkflowMinimap, WorkflowState};
use dioxus::prelude::*;
/// Interactive canvas frame. The caller supplies state and rendered nodes.
/// This component has no persistence or flow-execution semantics.
#[component]
pub fn WorkflowBuilder(state: WorkflowState, children: Element) -> Element {
    rsx! {
        section { "aria-label": "Workflow builder", class: "overflow-hidden rounded-lg border border-border bg-background",
            WorkflowCanvas { state,
                overlay: rsx! { WorkflowControls { state } WorkflowMinimap { state } },
                {children}
            }
        }
    }
}
