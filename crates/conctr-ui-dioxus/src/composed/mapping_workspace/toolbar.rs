use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use dioxus::prelude::*;

/// Controlled direction switch and history controls for a two-way mapping draft.
#[component]
pub fn MappingWorkspaceToolbar(
    response: bool,
    can_undo: bool,
    can_redo: bool,
    on_direction: EventHandler<bool>,
    on_undo: EventHandler<()>,
    on_redo: EventHandler<()>,
) -> Element {
    rsx! {
        div { class: "flex flex-wrap items-center gap-3",
            div { role: "group", "aria-label": "Mapping direction", class: "flex gap-2",
                for (value, label) in [(false, "Request"), (true, "Response")] {
                    Button { variant: if response == value { ButtonVariant::Default } else { ButtonVariant::Ghost },
                        size: ButtonSize::Sm, aria_label: if response == value { format!("{label} mapping (selected)") } else { format!("{label} mapping") },
                        onclick: move |_| on_direction.call(value), "{label}"
                    }
                }
            }
            div { role: "group", "aria-label": "Draft history", class: "flex gap-2",
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, disabled: !can_undo,
                    onclick: move |_| on_undo.call(()), "Undo" }
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, disabled: !can_redo,
                    onclick: move |_| on_redo.call(()), "Redo" }
            }
        }
    }
}
