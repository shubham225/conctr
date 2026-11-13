use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    Navbar { aria_label: "Example navigation",
        NavbarNav { index: 0usize, NavbarTrigger { "Components" } NavbarContent {
            NavbarItem { index: 0usize, value: "button".to_string(), to: crate::Route::ComponentPage { slug: "button".into() }, "Button" }
            NavbarItem { index: 1usize, value: "input".to_string(), to: crate::Route::ComponentPage { slug: "input".into() }, "Input" }
        } }
        NavbarItem { index: 1usize, value: "foundations".to_string(), to: crate::Route::Home {}, "Foundations" }
    }
        }
}
