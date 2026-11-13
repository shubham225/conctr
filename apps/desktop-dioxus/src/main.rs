use conctr_ui_dioxus::assets::styles;
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // Global app resources
        document::Link { rel: "stylesheet", href: styles::MAIN_CSS }

        "Hello World!!!"
    }
}
