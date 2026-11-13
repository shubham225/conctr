use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    Cluster {
        Avatar { size: AvatarImageSize::Small, "AB" }
        Avatar { size: AvatarImageSize::Medium, "CD" }
        Avatar { size: AvatarImageSize::Large, "EF" }
        Avatar { AvatarImage { src: conctr_ui_dioxus::assets::images::APP_ICON_SVG.to_string(), alt: "Conctr" } "CN" }
    }
        }
}
