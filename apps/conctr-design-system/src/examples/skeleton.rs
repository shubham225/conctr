use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    Stack { Skeleton { height: "var(--cn-space-6)", width: "50%" } Skeleton {} Skeleton { width: "75%" } Hint { "Skeletons respect reduced-motion preferences." } }
        }
}
