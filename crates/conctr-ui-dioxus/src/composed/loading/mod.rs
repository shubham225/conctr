//! Animated Conctr loading indicator with optional progress.

use crate::ui::progress::Progress;
use dioxus::prelude::*;

#[css_module("/src/composed/loading/styles.css")]
struct Styles;

const CSS: Asset = asset!(
    "/src/composed/loading/styles.css",
    AssetOptions::css_module()
);
const LOGO: &str = include_str!("logo.svg");

/// Centers the animated Conctr mark above its status text and optional progress.
#[component]
pub fn ConctrLoader(
    /// Caption below the icon. Pass an empty string to hide it.
    #[props(into, default = "Loading...".to_string())]
    text: String,
    /// Show an indeterminate bar when no finite value is supplied.
    #[props(default)]
    progress_bar: bool,
    /// Completion percentage. A finite value shows a determinate bar, clamped to 0–100.
    value: Option<f64>,
    /// Icon width and height in pixels.
    #[props(default = 128)]
    size: u32,
    /// Center the icon within a viewport-covering surface.
    #[props(default)]
    fullscreen: bool,
) -> Element {
    let layout = if fullscreen {
        Styles::fullscreen.inner
    } else {
        ""
    };
    let percent = value.filter(|v| v.is_finite()).map(|v| v.clamp(0.0, 100.0));
    let label = if text.is_empty() { "Loading" } else { &text };

    rsx! {
        document::Stylesheet { href: CSS }
        div {
            class: "flex flex-col items-center text-center {layout}",
            style: "--loader-size: {size}px",
            div { class: Styles::icon.inner, dangerous_inner_html: LOGO }
            div { class: "flex w-full flex-col items-center gap-3 {Styles::details}",
                span { role: "status", aria_label: if text.is_empty() { "Loading" }, "{text}" }
                if percent.is_some() || progress_bar {
                    div {
                        class: Styles::track.inner,
                        role: "progressbar",
                        aria_label: "{label}",
                        aria_valuemin: "0",
                        aria_valuemax: "100",
                        aria_valuenow: percent.map(|v| v.to_string()),
                        if let Some(value) = percent {
                            div { aria_hidden: "true", Progress { value } }
                        } else {
                            div { class: "h-2 overflow-hidden rounded-full bg-secondary {Styles::looping}",
                                div { class: "h-full bg-primary" }
                            }
                        }
                    }
                }
            }
        }
    }
}
