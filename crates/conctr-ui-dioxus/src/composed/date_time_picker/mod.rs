use super::{DatePicker, TimePicker};
use dioxus::prelude::*;

#[css_module("/src/composed/date_time_picker/style.css")]
struct Styles;
const CSS: Asset = asset!(
    "/src/composed/date_time_picker/style.css",
    AssetOptions::css_module()
);

use time::{PrimitiveDateTime, Time};
/// Date and minute-precision time composed as a local value; timezone policy belongs to the caller.
#[component]
pub fn DateTimePicker(
    #[props(default)] selected: Option<PrimitiveDateTime>,
    #[props(default)] on_change: Option<EventHandler<PrimitiveDateTime>>,
) -> Element {
    let mut date = use_signal(|| selected.map(|value| value.date()));
    let mut time = use_signal(|| selected.map_or(Time::MIDNIGHT, |value| value.time()));
    // Supplied values take precedence; local signals allow standalone use.
    let current_date = selected.map(|value| value.date()).or(date());
    let current_time = selected.map_or(time(), |value| value.time());
    rsx! {
        document::Stylesheet { href: CSS }
        div { class: "cn-date-time-picker {Styles::date_time_picker.inner}",
            div {
                span { class: "cn-picker-label {Styles::picker_label.inner}", "Date" }
                DatePicker {
                    selected: current_date,
                    on_change: move |value| {
                        date.set(Some(value));
                        if let Some(handler) = on_change {
                            handler.call(PrimitiveDateTime::new(value, current_time));
                        }
                    },
                }
            }
            div {
                span { class: "cn-picker-label {Styles::picker_label.inner}", "Local time" }
                TimePicker {
                    selected: Some(current_time),
                    on_change: move |value| {
                        time.set(value);
                        if let (Some(date), Some(handler)) = (current_date, on_change) {
                            handler.call(PrimitiveDateTime::new(date, value));
                        }
                    },
                }
            }
        }
    }
}
