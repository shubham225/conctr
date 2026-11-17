use crate::composed::{TagInput, TagVariant};
use dioxus::prelude::*;
#[component]
pub fn TagInputDemo() -> Element {
    let mut current = use_signal(Vec::<String>::new);
    rsx! {
        div { class: "grid gap-4",
            TagInput {
                initial: vec!["Production".into(), "Orders".into()],
                variant: TagVariant::Primary,
                on_change: move |tags| current.set(tags),
            }
            p { class: "text-sm text-muted-foreground",
                "Press Enter or comma to add a tag. Use Backspace to remove the last one."
            }
            TagInput {
                initial: vec!["Read only".into()],
                variant: TagVariant::Secondary,
                disabled: true,
            }
        }
    }
}
