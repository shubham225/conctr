use dioxus::prelude::*;

#[css_module("/src/composed/time_picker/style.css")]
struct Styles;
const CSS: Asset = asset!(
    "/src/composed/time_picker/style.css",
    AssetOptions::css_module()
);

use time::Time;
/// Minute-precision local time input. Values contain no timezone conversion.
#[component]
pub fn TimePicker(
    #[props(default)] selected: Option<Time>,
    #[props(default)] disabled: bool,
    #[props(default = "Choose time".into())] label: String,
    #[props(default)] on_change: Option<EventHandler<Time>>,
) -> Element {
    let mut local = use_signal(|| selected);
    let current = selected.or(local());
    rsx! {
        document::Stylesheet { href: CSS }
        div { class: "cn-time-picker {Styles::time_picker.inner}",
            icons::Clock { class: "size-4" }
            input {
                r#type: "time",
                "aria-label": label,
                disabled,
                step: "60",
                value: current.map(|t| format!("{:02}:{:02}", t.hour(), t.minute())).unwrap_or_default(),
                oninput: move |event| {
                    let value = event.value();
                    if let Some((hour, minute)) = value.split_once(':')
                        && let (Ok(hour), Ok(minute)) = (hour.parse(), minute.parse())
                        && let Ok(time) = Time::from_hms(hour, minute, 0)
                    {
                        local.set(Some(time));
                        if let Some(handler) = on_change {
                            handler.call(time);
                        }
                    }
                },
            }
        }
    }
}
