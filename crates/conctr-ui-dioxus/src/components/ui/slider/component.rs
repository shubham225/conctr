use dioxus::prelude::*;
use dioxus_primitives::slider::{self, RangeSliderProps, SliderProps};

#[css_module("/src/components/ui/slider/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/ui/slider/style.css",
    manganis::AssetOptions::css_module()
);

#[component]
pub fn Slider(props: SliderProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        slider::Slider {
            class: "cn-ui-slider {Styles::cn_ui_slider.inner} flex items-center relative min-w-50",
            value: props.value,
            default_value: props.default_value,
            min: props.min,
            max: props.max,
            step: props.step,
            disabled: props.disabled,
            horizontal: props.horizontal,
            inverted: props.inverted,
            on_value_change: props.on_value_change,
            label: props.label,
            attributes: props.attributes,
            slider::SliderTrack { class: "cn-ui-slider-track {Styles::cn_ui_slider_track.inner} relative rounded-full box-border bg-muted grow",
                slider::SliderRange { class: "cn-ui-slider-range {Styles::cn_ui_slider_range.inner} absolute rounded-full h-full bg-primary" }
                slider::SliderThumb { class: "cn-ui-slider-thumb {Styles::cn_ui_slider_thumb.inner} absolute rounded-full border-0 box-border w-4 h-4 top-1/2 block bg-card" }
            }
        }
    }
}

#[component]
pub fn RangeSlider(props: RangeSliderProps) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }

        slider::RangeSlider {
            class: "cn-ui-slider {Styles::cn_ui_slider.inner} flex items-center relative min-w-50",
            value: props.value,
            default_value: props.default_value,
            min: props.min,
            max: props.max,
            step: props.step,
            disabled: props.disabled,
            horizontal: props.horizontal,
            inverted: props.inverted,
            on_value_change: props.on_value_change,
            label: props.label,
            attributes: props.attributes,
            slider::SliderTrack { class: "cn-ui-slider-track {Styles::cn_ui_slider_track.inner} relative rounded-full box-border bg-muted grow",
                slider::SliderRange { class: "cn-ui-slider-range {Styles::cn_ui_slider_range.inner} absolute rounded-full h-full bg-primary" }
                slider::SliderThumb { class: "cn-ui-slider-thumb {Styles::cn_ui_slider_thumb.inner} absolute rounded-full border-0 box-border w-4 h-4 top-1/2 block bg-card", index: 0usize }
                slider::SliderThumb { class: "cn-ui-slider-thumb {Styles::cn_ui_slider_thumb.inner} absolute rounded-full border-0 box-border w-4 h-4 top-1/2 block bg-card", index: 1usize }
            }
        }
    }
}
