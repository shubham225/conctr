use dioxus::prelude::*;
/// Application-controlled theme switch; no hidden global preference or browser storage.
#[component]
pub fn BrandThemeToggle(dark: bool, on_change: EventHandler<bool>) -> Element {
    rsx! { crate::ui::button::Button {
        variant: crate::ui::button::ButtonVariant::Outline,
        aria_label: if dark { "Switch to light theme" } else { "Switch to dark theme" },
        onclick: move |_| on_change.call(!dark),
        size: crate::ui::button::ButtonSize::Icon,
        if dark { icons::Sun { class: "size-5" } } else { icons::Moon { class: "size-5" } }
    } }
}

/// Toggle application-provided `Signal<bool>` theme state in showcase examples.
#[component]
pub fn ThemeToggle() -> Element {
    let mut dark = use_context::<Signal<bool>>();
    rsx! { BrandThemeToggle { dark: dark(), on_change: move |value| dark.set(value) } }
}
