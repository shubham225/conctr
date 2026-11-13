use conctr_ui_dioxus::assets::{images, styles};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️
    let title = "CONCTR | where systems meet.";

    rsx! {
        // Global app resources
        document::Title {"{title}"}
        document::Link { rel: "icon", href: images::FAVICON }
        document::Link { rel: "stylesheet", href: styles::MAIN_CSS }

        "Hello World!!!"
    }
}
