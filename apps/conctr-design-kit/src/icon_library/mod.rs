use conctr_ui_dioxus::{assets::icons, ui::card::Card};
use dioxus::prelude::*;

#[css_module("/src/icon_library/style.css")]
struct Styles;

const CSS: Asset = asset!("/src/icon_library/style.css", AssetOptions::css_module());

#[component]
pub(crate) fn IconPalette() -> Element {
    let mut monochrome = use_signal(|| false);
    let (brand, patterns) = if monochrome() {
        (icons::mono::BRAND_GROUPS, icons::mono::PATTERN_GROUPS)
    } else {
        (icons::color::BRAND_GROUPS, icons::color::PATTERN_GROUPS)
    };
    let count: usize = brand
        .iter()
        .chain(patterns)
        .map(|(_, icons)| icons.len())
        .sum();
    let image_class = if monochrome() { "dark:invert" } else { "" };
    rsx! {
        document::Stylesheet { href: CSS }
        section { "aria-label": "Icon library", class: "grid gap-8",
            div { class: Styles::toolbar.inner,
                span { class: "text-sm text-muted-foreground", "{count} icons" }
                label { class: Styles::switch_label.inner,
                    span { "Monochrome" }
                    input {
                        r#type: "checkbox",
                        role: "switch",
                        class: Styles::toggle.inner,
                        checked: monochrome(),
                        onchange: move |event| monochrome.set(event.checked()),
                    }
                }
            }
            for &(group, glyphs) in brand.iter().chain(patterns) {
                div { key: "{group}", class: "grid gap-3",
                    h2 { class: "text-base font-semibold", "{group}" }
                    ul { class: Styles::grid.inner,
                        for &(label, icon) in glyphs {
                            li { key: "{label}", class: "min-w-0",
                                Card { class: Styles::tile.inner,
                                    img { src: icon, alt: "", width: 64, height: 64, class: image_class }
                                    span { class: "text-sm font-medium", "{label}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
