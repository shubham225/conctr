use conctr_ui_dioxus::ui::button::{Button, ButtonSize, ButtonVariant};
use dioxus::prelude::*;
#[component]
pub fn ButtonDemo() -> Element {
    rsx! {
        div { class: "grid gap-6",
            div { class: "flex flex-wrap gap-3",
                Button { "Create connection" }
                Button { variant: ButtonVariant::Outline, "View details" }
                Button { variant: ButtonVariant::Ghost, "Cancel" }
                Button { variant: ButtonVariant::Destructive, "Delete connection" }
            }
            div { class: "flex flex-wrap items-center gap-3",
                Button { size: ButtonSize::Icon, aria_label: "Add connection",
                    icons::Plus { class: "size-4" }
                }
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Icon,
                    aria_label: "Edit connection",
                    icons::Pencil { class: "size-4" }
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Icon,
                    aria_label: "More options",
                    icons::Ellipsis { class: "size-4" }
                }
            }
        }
    }
}
