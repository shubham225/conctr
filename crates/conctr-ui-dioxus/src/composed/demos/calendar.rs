use crate::composed::Calendar;
use dioxus::prelude::*;
#[component]
pub fn CalendarDemo() -> Element {
    let mut selected = use_signal(|| None);
    rsx! {
        div { class: "grid gap-4",
            Calendar {
                selected: selected(),
                on_change: move |date| selected.set(Some(date)),
            }
            p {
                {
                    selected()
                        .map(|date| format!("Selected: {date}"))
                        .unwrap_or_else(|| "Choose a day.".into())
                }
            }
        }
    }
}
