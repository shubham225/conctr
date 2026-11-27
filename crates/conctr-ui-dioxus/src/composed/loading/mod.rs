//! Conctr loading indicator: animated icon with optional status text
//! and progress bar.

use dioxus::prelude::*;

// Embedded at compile time, so the loader is styled on its very first frame.
const STYLES: &str = include_str!("styles.css");

/// Animated Conctr loader. Shows only the icon unless asked for more.
///
/// ```ignore
/// ConctrLoader {}
/// ConctrLoader { text: "Loading" }
/// ConctrLoader { fullscreen: true, text: "Syncing workspace", progress_bar: true }
/// ```
#[component]
pub fn ConctrLoader(
    /// Status text under the icon. Hidden when empty, which is the default.
    #[props(into, default)]
    text: String,
    /// Show a thin looping progress bar at the bottom.
    #[props(default)]
    progress_bar: bool,
    /// Icon width and height in pixels.
    #[props(default = 128)]
    size: u32,
    /// Cover the whole viewport with a background.
    #[props(default)]
    fullscreen: bool,
) -> Element {
    let class = if fullscreen {
        "conctr-loader conctr-loader--fullscreen"
    } else {
        "conctr-loader"
    };

    rsx! {
        style { dangerous_inner_html: STYLES }

        div {
            class: "{class}",
            role: "status",
            aria_live: "polite",
            // Screen readers still get a name when there is no visible text
            aria_label: if text.is_empty() { "Loading" },

            svg {
                class: "conctr-loader__icon",
                width: "{size}",
                height: "{size}",
                view_box: "0 0 64 64",
                "aria-hidden": "true",

                // Outer hexagon and corner dots
                g { class: "conctr-loader__frame",
                    polygon { points: "32,8 52.8,20 52.8,44 32,56 11.2,44 11.2,20" }
                    circle { cx: "52.8", cy: "20", r: "1.8" }
                    circle { cx: "32", cy: "56", r: "1.8" }
                    circle { cx: "11.2", cy: "20", r: "1.8" }
                }

                g { class: "conctr-loader__spokes",
                    path { class: "conctr-loader__spoke", d: "M32 32V8" }
                    path { class: "conctr-loader__spoke", d: "M32 32L52.8 44" }
                    path { class: "conctr-loader__spoke", d: "M32 32L11.2 44" }
                }

                // Nodes
                circle {
                    class: "conctr-loader__node conctr-loader__node--circle",
                    cx: "32",
                    cy: "8",
                    r: "5.5",
                    fill: "#14B8A6",
                }
                rect {
                    class: "conctr-loader__node conctr-loader__node--square",
                    x: "47.3",
                    y: "38.5",
                    width: "11",
                    height: "11",
                    rx: "2.2",
                    fill: "#F2603C",
                }
                polygon {
                    class: "conctr-loader__node conctr-loader__node--triangle",
                    points: "11.2,37.5 17.6,49 4.8,49",
                    fill: "#F5A524",
                    stroke: "#F5A524",
                    stroke_width: "1.5",
                    stroke_linejoin: "round",
                }

                // Core is drawn last so the nodes disappear underneath it
                polygon {
                    class: "conctr-loader__core",
                    points: "32,22 40.7,27 40.7,37 32,42 23.3,37 23.3,27",
                    fill: "#4F46E5",
                    stroke: "#4F46E5",
                    stroke_width: "2",
                    stroke_linejoin: "round",
                }
            }

            if !text.is_empty() {
                span { class: "conctr-loader__text", "{text}" }
            }

            if progress_bar {
                div { class: "conctr-loader__bar", "aria-hidden": "true",
                    span {}
                }
            }
        }
    }
}
