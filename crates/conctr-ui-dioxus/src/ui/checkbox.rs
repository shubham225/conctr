//! Supports local state when no controlled value is supplied.
use dioxus::prelude::*;
#[component]
pub fn Checkbox(
    #[props(default)] class: Option<String>,
    #[props(default)] checked: Option<bool>,
    #[props(default)] disabled: bool,
    #[props(default)] on_checked_change: Option<EventHandler<bool>>,
    #[props(default)] on_change: Option<EventHandler<bool>>,
    #[props(default="Checkbox".into())] aria_label: String,
) -> Element {
    let mut local = use_signal(|| false);
    let value = checked.unwrap_or(local());
    let change = move |next| {
        if checked.is_none() {
            local.set(next);
        }
        if let Some(cb) = on_checked_change {
            cb.call(next);
        }
        if let Some(cb) = on_change {
            cb.call(next);
        }
    };
    rsx! {crate::rust_ui::ui::checkbox::Checkbox {class,checked:value,disabled,aria_label,on_checked_change:change}}
}
