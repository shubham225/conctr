use conctr_ui_dioxus::assets::styles;
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // Global app resources
        document::Link { rel: "stylesheet", href: styles::CONCTR_UI_CSS }

        "Hello World!!!"
    }
}
