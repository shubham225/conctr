use crate::components::ui::dialog::{Dialog, DialogTitle};
use dioxus::prelude::*;

/// Controlled Conctr modal composed from the Dioxus dialog primitive.
/// Mount while open, handle `onclose`, and include a close action in the footer.
#[component]
pub fn Modal(id: String, title: String, onclose: EventHandler<()>, children: Element) -> Element {
    rsx! {
        Dialog {
            id: id.clone(), open: true,
            on_open_change: move |open: bool| { if !open { onclose.call(()); } },
            DialogTitle { id: format!("{id}-title"), "{title}" }
            {children}
        }
    }
}
