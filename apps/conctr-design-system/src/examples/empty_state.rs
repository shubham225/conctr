use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    EmptyState { title: "No connections yet", description: "Create a connection to start exchanging data between systems.", icon: rsx! { Icon { src: conctr_ui_dioxus::assets::images::ICON_COLOR_SVG, alt: "", } }, Button { "Create connection" } }
        }
}
