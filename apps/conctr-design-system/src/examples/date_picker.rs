use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut selected = use_signal(|| None::<time::Date>);
    rsx! {
    Stack {
        DatePicker { selected_date: selected(), on_value_change: move |v| selected.set(v) }
        p { role: "status", "Selected date: {selected():?}" }
    }
        }
}
