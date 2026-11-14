use conctr_ui_dioxus::assets::styles;
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // Global app resources
        styles::ConctrStyles {}

        "Hello World!!!"
    }
}
