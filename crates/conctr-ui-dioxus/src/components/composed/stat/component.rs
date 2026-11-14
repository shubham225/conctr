use dioxus::prelude::*;

#[css_module("/src/components/composed/stat/style.css")]
struct Styles;

// Register for every document, including repeated SSR requests.
const STYLES: Asset = asset!(
    "/src/components/composed/stat/style.css",
    manganis::AssetOptions::css_module()
);

/// A statistic; values are supplied by the application.
#[component]
pub fn Stat(label: String, value: String) -> Element {
    rsx! {
        document::Stylesheet { href: STYLES }
        div { class: "cn-stat grid gap-1",
            span { class: "cn-stat__label text-sm text-muted-foreground", "{label}" }
            span { class: "cn-stat__value {Styles::cn_stat__value.inner} text-foreground", "{value}" }
        }
    }
}
