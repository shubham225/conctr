use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
    Stack {
        span { class: "cn-label", "Visible fields" }
        ToggleGroup { horizontal: true, allow_multiple_pressed: true,
            ToggleItem { index: 0usize, "Source" }
            ToggleItem { index: 1usize, "Protocol" }
            ToggleItem { index: 2usize, "Status" }
            ToggleItem { index: 3usize, disabled: true, "Credentials" }
        }
        Hint { "Use arrow keys to move and Space to select." }
    }
        }
}
