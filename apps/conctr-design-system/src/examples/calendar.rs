use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let today =
        time::Date::from_calendar_date(2026, time::Month::November, 12).unwrap_or(time::Date::MIN);
    let mut selected = use_signal(|| None::<time::Date>);
    rsx! {
    Calendar { today: today, view_date: today, selected_date: selected(), on_date_change: move |v| selected.set(v) }
    p { role: "status", "Selected date: {selected():?}" }
        }
}
