use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    nav { aria_label: "Breadcrumb", Breadcrumb { li { a { href: "/", "Workspace" } } li { a { href: "/components/navigation", "Connections" } } li { aria_current: "page", "Order sync" } } }
        }
}
