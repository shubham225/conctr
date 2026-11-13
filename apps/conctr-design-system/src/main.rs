mod catalog;
mod examples;
mod foundations;
mod shell;

use conctr_ui_dioxus::{assets::images, prelude::*};
use dioxus::prelude::*;
use shell::{ComponentPage, Foundation, Home, NotFound, Shell};

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[layout(Shell)]
    #[route("/")]
    Home {},
    #[route("/foundations/:topic")]
    Foundation { topic: String },
    #[route("/components/:slug")]
    ComponentPage { slug: String },
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let dark = use_signal(|| false);
    use_context_provider(|| dark);
    rsx! {
        document::Title { "Conctr design system" }
        document::Link { rel: "icon", href: images::FAVICON }
        document::Stylesheet { href: asset!("/assets/gallery.css") }
        ThemeProvider { theme: if dark() { Theme::Dark } else { Theme::Light },
            ToastProvider { Router::<Route> {} }
        }
    }
}
