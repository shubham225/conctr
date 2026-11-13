use crate::components::ui::status::Status;
use dioxus::prelude::*;

/// Accessible message; danger messages are announced immediately.
#[component]
pub fn StatusAlert(#[props(default)] status: Status, children: Element) -> Element {
    rsx! { div { class: "cn-alert cn-alert--{status.class()}", role: if status == Status::Danger { "alert" } else { "status" }, {children} } }
}
