//! CSS-driven loading feedback; no external script is required.
use dioxus::prelude::*;

/// Masks content while loading, preserving its geometry and disabling interaction.
/// Custom colors are CSS colors; duration is in seconds and radius in pixels.
#[component]
pub fn Shimmer(
    loading: Signal<bool>,
    #[props(into, optional)] shimmer_color: Option<String>,
    #[props(into, optional)] background_color: Option<String>,
    #[props(optional)] duration: Option<f64>,
    #[props(optional)] fallback_border_radius: Option<f64>,
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let class = tw_merge::tw_merge!("cn-shimmer relative", class.as_deref().unwrap_or(""));
    let duration = duration
        .filter(|value| value.is_finite() && *value > 0.0)
        .unwrap_or(1.6);
    let radius = fallback_border_radius
        .filter(|value| value.is_finite() && *value >= 0.0)
        .unwrap_or(14.0);
    let background = background_color.as_deref().unwrap_or("var(--cn-sunken)");
    let highlight = shimmer_color.as_deref().unwrap_or("var(--cn-raised)");
    rsx! {
        div { class, "data-name": "Shimmer", "data-shimmer-loading": loading().to_string(),
            "aria-busy": loading().to_string(),
            style: "--shimmer-duration:{duration}s;--shimmer-radius:{radius}px;--shimmer-bg:{background};--shimmer-highlight:{highlight}",
            div { inert: loading(), {children} }
            if loading() { span { class: "sr-only", role: "status", "Loading content" } }
        }
    }
}
