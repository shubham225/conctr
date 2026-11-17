//! Conctr calendar primitives with keyboard-accessible day selection.
pub use crate::rust_ui::ui::date_picker::*;
use dioxus::prelude::*;
/// Selects a day with a native button while preserving range-selection props.
#[allow(non_snake_case)]
pub fn DatePickerCell(props: DatePickerCellProps) -> Element {
    let date = time::Date::from_calendar_date(props.year, props.month, props.day).ok();
    let current = date.is_some_and(|d| d == (props.start_date)() || d == (props.end_date)());
    let range = date.is_some_and(|d| d > (props.start_date)() && d < (props.end_date)());
    rsx! {
        td { class: "p-0",
            button { r#type: "button", "data-name": "DatePickerCell",
                class: tw_merge::tw_merge!("inline-flex size-9 items-center justify-center rounded-sm text-sm hover:bg-accent", if range { "bg-accent" } else { "" }, props.class.as_deref().unwrap_or("")),
                disabled: props.disabled || date.is_none(),
                "aria-label": date.map(|date| date.to_string()),
                "aria-current": current.to_string(),
                onclick: move |_| props.on_click.call(props.day),
                "{props.day}"
            }
        }
    }
}
