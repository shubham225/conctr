use conctr_ui_dioxus::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut dark = use_signal(|| false);
    rsx! {
        ThemeProvider { theme: if dark() { Theme::Dark } else { Theme::Light },
            div { class: "flex items-center gap-4 rounded-lg border border-border bg-background p-6 text-foreground",
                ThemeToggle { dark: dark(), on_change: move |value| dark.set(value) }
                // ThemeToggle handles the reveal; callers only supply theme state.
                span { if dark() { "Dark appearance" } else { "Light appearance" } }
                ThemeToggle { dark: dark(), on_change: move |_| {}, disabled: true }
            }
        }
    }
}
