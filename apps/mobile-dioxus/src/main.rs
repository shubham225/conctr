use conctr_ui_dioxus::assets::{images, styles};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: images::FAVICON }
        styles::ConctrStyles {}

        "Hello World!!!"
    }
}
