use conctr_ui_dioxus::composed::DatePicker;
use dioxus::prelude::*;

/// Selection is owned by the page so it can be used when submitting a form.
#[component]
pub fn DatePickerDemo() -> Element {
    let mut selected = use_signal(|| None);
    rsx! {
        div { class: "grid gap-4",
            DatePicker {
                selected: selected(),
                on_change: move |date| selected.set(Some(date)),
            }
            p { class: "text-sm text-muted-foreground",
                {
                    selected()
                        .map(|date| format!("Selected date: {date}"))
                        .unwrap_or_else(|| "Select a date to schedule the demo.".into())
                }
            }
        }
    }
}
