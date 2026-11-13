use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut value = use_signal(|| 30.0);
    rsx! {
    Stack {
        Slider { label: "Retry interval", horizontal: true, min: 0.0, max: 100.0, step: 1.0, default_value: 30.0, on_value_change: move |v: f64| value.set(v) }
        p { role: "status", "Retry interval: {value:.0} seconds" }
        RangeSlider { label: "Allowed range", horizontal: true, min: 0.0, max: 100.0, default_value: 20.0..80.0 }
    }
        }
}
