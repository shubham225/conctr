use crate::components::ui::{alert::Alert, status::Status};
use dioxus::prelude::*;

/// Compatibility adapter. Prefer `Alert { status, .. }` in new code.
#[component]
pub fn StatusAlert(#[props(default)] status: Status, children: Element) -> Element {
    rsx! { Alert { status, {children} } }
}
