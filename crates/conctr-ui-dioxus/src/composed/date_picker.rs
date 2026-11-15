use super::Calendar;
use dioxus::prelude::*;
use time::Date;
/// Date picker with a kit calendar; selection is reported to the caller.
#[component]
pub fn DatePicker(
    #[props(default)] selected: Option<Date>,
    #[props(default)] on_change: Option<EventHandler<Date>>,
) -> Element {
    let mut open = use_signal(|| false);
    let mut local = use_signal(|| selected);
    rsx! {div{class:"relative w-fit",onkeydown:move|e|{if e.key()==Key::Escape{open.set(false);}},
    crate::ui::button::Button{variant:crate::ui::button::ButtonVariant::Outline,aria_label:"Choose date",onclick:move |_|open.set(!open()),icons::Calendar{class:"size-4"},{selected.or(local()).map(|d|d.to_string()).unwrap_or_else(||"Choose date".into())}}
    if open(){div{class:"absolute left-0 top-full z-50 mt-2",Calendar{selected:selected.or(local()),on_change:move|date|{local.set(Some(date));open.set(false);if let Some(cb)=on_change{cb.call(date);}}}}}
    }}
}
