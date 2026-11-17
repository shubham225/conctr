use crate::composed::TimePicker;
use dioxus::prelude::*;
#[component]
pub fn TimePickerDemo() -> Element {
    let mut selected = use_signal(|| None);
    rsx! {
        div { class: "grid gap-4",
            TimePicker {
                selected: selected(),
                on_change: move |value| selected.set(Some(value)),
            }
            p { class: "text-sm text-muted-foreground",
                {
                    selected()
                        .map(|value| format!("Local time: {value}"))
                        .unwrap_or_else(|| "Choose a time.".into())
                }
            }
        }
    }
}
