use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    Navigation { aria_label: "Example sidebar", NavGroup { "Workspace" } NavItem { href: "#connections", active: true, "Connections" } NavItem { href: "#runs", "Runs" } NavGroup { "Administration" } NavItem { href: "#settings", "Settings" } }
        }
}
