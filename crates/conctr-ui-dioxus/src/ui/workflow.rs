//! Conctr facade for Rust UI `workflow`.
/// Presentation state and hooks for the canvas; independent of executable flows.
pub use crate::hooks::use_workflow::{
    WorkflowEdge, WorkflowNode, WorkflowNodeKind, WorkflowState, use_workflow,
};
pub use crate::rust_ui::ui::workflow::*;

use dioxus::prelude::*;
use std::rc::Rc;

/// Workflow canvas with preview coordinates measured in the canvas viewport.
#[component]
pub fn WorkflowCanvas(
    state: WorkflowState,
    children: Element,
    #[props(optional)] overlay: Option<Element>,
) -> Element {
    let mut state = state;
    let mut anchor = use_signal(|| None::<Rc<MountedData>>);
    let mut origin = use_signal(|| None);
    rsx! {
        div {
            onpointerdown: move |_| {
                let element = anchor.peek().clone();
                spawn(async move {
                    if let Some(element) = element
                        && let Ok(rect) = element.get_client_rect().await
                    {
                        origin.set(Some((rect.min_x(), rect.min_y())));
                    }
                });
            },
            onmousemove: move |event| {
                // The upstream handler uses offsets relative to the event target.
                // Correct them after bubbling out of the canvas, including over ports.
                if let Some((x, y)) = *origin.peek()
                    && state.connecting.peek().is_some()
                {
                    let cursor = event.client_coordinates();
                    state.update_connect_mouse(cursor.x - x, cursor.y - y);
                }
            },
            crate::rust_ui::ui::workflow::WorkflowCanvas {
                state,
                overlay: rsx! {
                    span {
                        class: "pointer-events-none absolute top-0 left-0",
                        "aria-hidden": "true",
                        onmounted: move |event| anchor.set(Some(event.data())),
                    }
                    {overlay}
                },
                {children}
            }
        }
    }
}
