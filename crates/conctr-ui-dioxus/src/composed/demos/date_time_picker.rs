use crate::composed::DateTimePicker;
use dioxus::prelude::*;
#[component]
pub fn DateTimePickerDemo() -> Element {
    let mut selected = use_signal(|| None);
    rsx! {
        div { class: "grid gap-4",
            DateTimePicker {
                selected: selected(),
                on_change: move |value| selected.set(Some(value)),
            }
            p { class: "text-sm text-muted-foreground",
                {
                    selected()
                        .map(|value| format!("Scheduled for {value} (local time)"))
                        .unwrap_or_else(|| "Choose a date and local time.".into())
                }
            }
        }
    }
}
