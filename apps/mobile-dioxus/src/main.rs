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
        document::Link { rel: "stylesheet", href: styles::CONCTR_UI_CSS }

        "Hello World!!!"
    }
}
